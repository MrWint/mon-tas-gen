/***************************************************************************
 *   Copyright (C) 2007 by Sindre Aamås                                    *
 *   aamas@stud.ntnu.no                                                    *
 *                                                                         *
 *   This program is free software; you can redistribute it and/or modify  *
 *   it under the terms of the GNU General Public License version 2 as     *
 *   published by the Free Software Foundation.                            *
 *                                                                         *
 *   This program is distributed in the hope that it will be useful,       *
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of        *
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the         *
 *   GNU General Public License version 2 for more details.                *
 *                                                                         *
 *   You should have received a copy of the GNU General Public License     *
 *   version 2 along with this program; if not, write to the               *
 *   Free Software Foundation, Inc.,                                       *
 *   59 Temple Place - Suite 330, Boston, MA  02111-1307, USA.             *
 ***************************************************************************/
#include "memory.h"
#include "video.h"
#include "sound.h"
#include "savestate.h"
#include <cstring>

namespace gambatte {

Memory::Memory(const Interrupter &interrupter_in, uint16_t &sp, uint16_t &pc)
: SP(sp),
  PC(pc),
  getInput(0),
  divLastUpdate(0),
  lastOamDmaUpdate(DISABLED_TIME),
  display(ioamhram, 0, VideoInterruptRequester(&intreq)),
  interrupter(interrupter_in),
  dmaSource(0),
  dmaDestination(0),
  oamDmaPos(0xFE),
  serialCnt(0),
  blanklcd(false),
  LINKCABLE(false),
  linkClockTrigger(false)
{
	intreq.setEventTime<BLIT>(144*456ul);
	intreq.setEventTime<END>(0);
}

void Memory::setStatePtrs(SaveState &state) {
	state.mem.ioamhram.set(ioamhram, sizeof ioamhram);

	cart.setStatePtrs(state);
	display.setStatePtrs(state);
	sound.setStatePtrs(state);
}


static inline int32_t serialCntFrom(const uint32_t cyclesUntilDone, const bool cgbFast) {
	return cgbFast ? (cyclesUntilDone + 0xF) >> 4 : (cyclesUntilDone + 0x1FF) >> 9;
}

void Memory::loadState(const SaveState &state) {
	biosMode = state.mem.biosMode;
	cgbSwitching = state.mem.cgbSwitching;
	agbMode = state.mem.agbMode;
	gbIsCgb_ = state.mem.gbIsCgb;
	sound.loadState(state);
	display.loadState(state, state.mem.oamDmaPos < 0xA0 ? cart.rdisabledRam() : ioamhram);
	tima.loadState(state, TimaInterruptRequester(intreq));
	cart.loadState(state);
	intreq.loadState(state);

	divLastUpdate = state.mem.divLastUpdate;
	intreq.setEventTime<SERIAL>(state.mem.nextSerialtime > state.cpu.cycleCounter ? state.mem.nextSerialtime : state.cpu.cycleCounter);
	intreq.setEventTime<UNHALT>(state.mem.unhaltTime);
	lastOamDmaUpdate = state.mem.lastOamDmaUpdate;
	dmaSource = state.mem.dmaSource;
	dmaDestination = state.mem.dmaDestination;
	oamDmaPos = state.mem.oamDmaPos;
	serialCnt = intreq.eventTime(SERIAL) != DISABLED_TIME
			? serialCntFrom(intreq.eventTime(SERIAL) - state.cpu.cycleCounter, ioamhram[0x102] & isCgb() * 2)
			: 8;

	cart.setVrambank(ioamhram[0x14F] & isCgb());
	cart.setOamDmaSrc(OAM_DMA_SRC_OFF);
	cart.setWrambank(isCgb() && (ioamhram[0x170] & 0x07) ? ioamhram[0x170] & 0x07 : 1);

	if (lastOamDmaUpdate != DISABLED_TIME) {
		oamDmaInitSetup();

		const uint32_t oamEventPos = oamDmaPos < 0xA0 ? 0xA0 : 0x100;

		intreq.setEventTime<OAM>(lastOamDmaUpdate + (oamEventPos - oamDmaPos) * 4);
	}

	intreq.setEventTime<BLIT>((ioamhram[0x140] & 0x80) ? display.nextMode1IrqTime() : state.cpu.cycleCounter);
	blanklcd = false;
	
	if (!isCgb())
		std::memset(cart.vramdata() + 0x2000, 0, 0x2000);
}

void Memory::setEndtime(const uint32_t cycleCounter, const uint32_t inc) {
	if (intreq.eventTime(BLIT) <= cycleCounter)
		intreq.setEventTime<BLIT>(intreq.eventTime(BLIT) + (70224 << isDoubleSpeed()));
	
	intreq.setEventTime<END>(cycleCounter + (inc << isDoubleSpeed()));
}

void Memory::updateSerial(const uint32_t cc) {
	if (!LINKCABLE) {
		if (intreq.eventTime(SERIAL) != DISABLED_TIME) {
			if (intreq.eventTime(SERIAL) <= cc) {
				ioamhram[0x101] = (((ioamhram[0x101] + 1) << serialCnt) - 1) & 0xFF;
				ioamhram[0x102] &= 0x7F;
				intreq.setEventTime<SERIAL>(DISABLED_TIME);
				intreq.flagIrq(8);
			} else {
				const int32_t targetCnt = serialCntFrom(intreq.eventTime(SERIAL) - cc, ioamhram[0x102] & isCgb() * 2);
				ioamhram[0x101] = (((ioamhram[0x101] + 1) << (serialCnt - targetCnt)) - 1) & 0xFF;
				serialCnt = targetCnt;
			}
		}
	}
	else {
		if (intreq.eventTime(SERIAL) != DISABLED_TIME) {
			if (intreq.eventTime(SERIAL) <= cc) {
				linkClockTrigger = true;
				intreq.setEventTime<SERIAL>(DISABLED_TIME);
			}
		}
	}
}

void Memory::updateTimaIrq(const uint32_t cc) {
	while (intreq.eventTime(TIMA) <= cc)
		tima.doIrqEvent(TimaInterruptRequester(intreq));
}

void Memory::updateIrqs(const uint32_t cc) {
	updateSerial(cc);
	updateTimaIrq(cc);
	display.update(cc);
}

uint32_t Memory::event(uint32_t cycleCounter) {
	if (lastOamDmaUpdate != DISABLED_TIME)
		updateOamDma(cycleCounter);

	switch (intreq.minEventId()) {
	case UNHALT:
		nontrivial_ff_write(0xFF04, 0, cycleCounter);
		PC = (PC + 1) & 0xFFFF;
		cycleCounter += 4;
		intreq.unhalt();
		intreq.setEventTime<UNHALT>(DISABLED_TIME);
		break;
	case END:
		intreq.setEventTime<END>(DISABLED_TIME - 1);

		while (cycleCounter >= intreq.minEventTime() && intreq.eventTime(END) != DISABLED_TIME)
			cycleCounter = event(cycleCounter);
		
		intreq.setEventTime<END>(DISABLED_TIME);

		break;
	case BLIT:
		{
			const bool lcden = ioamhram[0x140] >> 7 & 1;
			uint32_t blitTime = intreq.eventTime(BLIT);
			
			if (lcden | blanklcd) {
				display.updateScreen(blanklcd, cycleCounter);
				intreq.setEventTime<BLIT>(DISABLED_TIME);
				intreq.setEventTime<END>(DISABLED_TIME);
				
				while (cycleCounter >= intreq.minEventTime())
					cycleCounter = event(cycleCounter);
			} else
				blitTime += 70224 << isDoubleSpeed();
			
			blanklcd = lcden ^ 1;
			intreq.setEventTime<BLIT>(blitTime);
		}
		break;
	case SERIAL:
		updateSerial(cycleCounter);
		break;
	case OAM:
		intreq.setEventTime<OAM>(lastOamDmaUpdate == DISABLED_TIME ?
				static_cast<uint32_t>(DISABLED_TIME) : intreq.eventTime(OAM) + 0xA0 * 4);
		break;
	case DMA:
		{
			const bool doubleSpeed = isDoubleSpeed();
			uint32_t dmaSrc = dmaSource;
			uint32_t dmaDest = dmaDestination;
			uint32_t dmaLength = ((ioamhram[0x155] & 0x7F) + 0x1) * 0x10;
			uint32_t length = hdmaReqFlagged(intreq) ? 0x10 : dmaLength;
			
			ackDmaReq(&intreq);

			if ((static_cast<uint32_t>(dmaDest) + length) & 0x10000) {
				length = 0x10000 - dmaDest;
				ioamhram[0x155] |= 0x80;
			}

			dmaLength -= length;

			if (!(ioamhram[0x140] & 0x80))
				dmaLength = 0;

			{
				uint32_t lOamDmaUpdate = lastOamDmaUpdate;
				lastOamDmaUpdate = DISABLED_TIME;

				while (length--) {
					const uint32_t src = dmaSrc++ & 0xFFFF;
					const uint32_t data = ((src & 0xE000) == 0x8000 || src > 0xFDFF) ? 0xFF : read(src, cycleCounter);

					cycleCounter += 2 << doubleSpeed;

					if (cycleCounter - 3 > lOamDmaUpdate) {
						oamDmaPos = (oamDmaPos + 1) & 0xFF;
						lOamDmaUpdate += 4;

						if (oamDmaPos < 0xA0) {
							if (oamDmaPos == 0)
								startOamDma(lOamDmaUpdate - 1);

							ioamhram[src & 0xFF] = data;
						} else if (oamDmaPos == 0xA0) {
							endOamDma(lOamDmaUpdate - 1);
							lOamDmaUpdate = DISABLED_TIME;
						}
					}

					nontrivial_write(0x8000 | (dmaDest++ & 0x1FFF), data, cycleCounter);
				}

				lastOamDmaUpdate = lOamDmaUpdate;
			}

			cycleCounter += 4;

			dmaSource = dmaSrc;
			dmaDestination = dmaDest;
			ioamhram[0x155] = ((dmaLength / 0x10 - 0x1) & 0xFF) | (ioamhram[0x155] & 0x80);

			if ((ioamhram[0x155] & 0x80) && display.hdmaIsEnabled()) {
				if (lastOamDmaUpdate != DISABLED_TIME)
					updateOamDma(cycleCounter);
				
				display.disableHdma(cycleCounter);
			}
		}

		break;
	case TIMA:
		tima.doIrqEvent(TimaInterruptRequester(intreq));
		break;
	case VIDEO:
		display.update(cycleCounter);
		break;
	case INTERRUPTS:
		if (stopped) {
			intreq.setEventTime<INTERRUPTS>(DISABLED_TIME);
			break;
		}
		if (halted()) {
			if (gbIsCgb_ || (!gbIsCgb_ && cycleCounter <= halttime + 4))
				cycleCounter += 4;
			
			intreq.unhalt();
			intreq.setEventTime<UNHALT>(DISABLED_TIME);
		}
		
		if (ime()) {
			uint32_t address;

			cycleCounter += 12;
			display.update(cycleCounter);
			SP = (SP - 2) & 0xFFFF;
			write(SP + 1, PC >> 8, cycleCounter);
			uint32_t ie = intreq.iereg();

			cycleCounter += 4;
			display.update(cycleCounter);
			write(SP, PC & 0xFF, cycleCounter);
			const uint32_t pendingIrqs = ie & intreq.ifreg();

			cycleCounter += 4;
			display.update(cycleCounter);
			const uint32_t n = pendingIrqs & -pendingIrqs;
			
			if (n == 0) {
				address = 0;
			}
			else if (n < 8) {
				static const uint8_t lut[] = { 0x40, 0x48, 0x48, 0x50 };
				address = lut[n-1];
			} else
				address = 0x50 + n;
			
			intreq.ackIrq(n);
			PC = address;
		}
		
		break;
	}

	return cycleCounter;
}

uint32_t Memory::stop(uint32_t cycleCounter) {
	cycleCounter += 4;
	
	if (ioamhram[0x14D] & isCgb()) {
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		
		display.speedChange(cycleCounter);
		ioamhram[0x14D] ^= 0x81;

		intreq.setEventTime<BLIT>((ioamhram[0x140] & 0x80) ? display.nextMode1IrqTime() : cycleCounter + (70224 << isDoubleSpeed()));
		
		if (intreq.eventTime(END) > cycleCounter) {
			intreq.setEventTime<END>(cycleCounter + (isDoubleSpeed() ?
					(intreq.eventTime(END) - cycleCounter) << 1 : (intreq.eventTime(END) - cycleCounter) >> 1));
		}
		// when switching speed, it seems that the CPU spontaneously restarts soon?
		// otherwise, the cpu should be allowed to stay halted as long as needed
		// so only execute this line when switching speed
		intreq.halt();
		intreq.setEventTime<UNHALT>(cycleCounter + 0x20000);
	}
	else {
		stopped = true;
		intreq.halt();
	}

	return cycleCounter;
}

static void decCycles(uint32_t &counter, const uint32_t dec) {
	if (counter != DISABLED_TIME)
		counter -= dec;
}

void Memory::decEventCycles(const MemEventId eventId, const uint32_t dec) {
	if (intreq.eventTime(eventId) != DISABLED_TIME)
		intreq.setEventTime(eventId, intreq.eventTime(eventId) - dec);
}

uint32_t Memory::resetCounters(uint32_t cycleCounter) {
	if (lastOamDmaUpdate != DISABLED_TIME)
		updateOamDma(cycleCounter);

	updateIrqs(cycleCounter);

	const uint32_t oldCC = cycleCounter;

	{
		const uint32_t divinc = (cycleCounter - divLastUpdate) >> 8;
		ioamhram[0x104] = (ioamhram[0x104] + divinc) & 0xFF;
		divLastUpdate += divinc << 8;
	}

	const uint32_t dec = cycleCounter < 0x10000 ? 0 : (cycleCounter & ~0x7FFFul) - 0x8000;

	decCycles(divLastUpdate, dec);
	decCycles(lastOamDmaUpdate, dec);
	decEventCycles(SERIAL, dec);
	decEventCycles(OAM, dec);
	decEventCycles(BLIT, dec);
	decEventCycles(END, dec);
	decEventCycles(UNHALT, dec);

	cycleCounter -= dec;
	
	intreq.resetCc(oldCC, cycleCounter);
	tima.resetCc(oldCC, cycleCounter, TimaInterruptRequester(intreq));
	display.resetCc(oldCC, cycleCounter);
	sound.resetCounter(cycleCounter, oldCC, isDoubleSpeed());

	return cycleCounter;
}

void Memory::updateInput() {
	uint32_t state = 0xF;

	if ((ioamhram[0x100] & 0x30) != 0x30 && getInput) {
		uint32_t input = getInput(getInputContext);
		uint32_t dpad_state = ~input >> 4;
		uint32_t button_state = ~input;
		if (!(ioamhram[0x100] & 0x10))
			state &= dpad_state;
		if (!(ioamhram[0x100] & 0x20))
			state &= button_state;
	}

	if (state != 0xF && (ioamhram[0x100] & 0xF) == 0xF)
		intreq.flagIrq(0x10);

	ioamhram[0x100] = (ioamhram[0x100] & -0x10u) | state;
}

void Memory::updateOamDma(const uint32_t cycleCounter) {
	const uint8_t *const oamDmaSrc = oamDmaSrcPtr();
	uint32_t cycles = (cycleCounter - lastOamDmaUpdate) >> 2;

	while (cycles--) {
		oamDmaPos = (oamDmaPos + 1) & 0xFF;
		lastOamDmaUpdate += 4;

		if (oamDmaPos < 0xA0) {
			if (oamDmaPos == 0)
				startOamDma(lastOamDmaUpdate - 1);

			ioamhram[oamDmaPos] = oamDmaSrc ? oamDmaSrc[oamDmaPos] : cart.rtcRead();
		} else if (oamDmaPos == 0xA0) {
			endOamDma(lastOamDmaUpdate - 1);
			lastOamDmaUpdate = DISABLED_TIME;
			break;
		}
	}
}

void Memory::oamDmaInitSetup() {
	if (ioamhram[0x146] < 0xA0) {
		cart.setOamDmaSrc(ioamhram[0x146] < 0x80 ? OAM_DMA_SRC_ROM : OAM_DMA_SRC_VRAM);
	} else if (ioamhram[0x146] < 0xFE - isCgb() * 0x1E) {
		cart.setOamDmaSrc(ioamhram[0x146] < 0xC0 ? OAM_DMA_SRC_SRAM : OAM_DMA_SRC_WRAM);
	} else
		cart.setOamDmaSrc(OAM_DMA_SRC_INVALID);
}

static const uint8_t * oamDmaSrcZero() {
	static uint8_t zeroMem[0xA0];
	return zeroMem;
}

const uint8_t * Memory::oamDmaSrcPtr() const {
	switch (cart.oamDmaSrc()) {
	case OAM_DMA_SRC_ROM:  return cart.romdata(ioamhram[0x146] >> 6) + (ioamhram[0x146] << 8);
	case OAM_DMA_SRC_SRAM: return cart.rsrambankptr() ? cart.rsrambankptr() + (ioamhram[0x146] << 8) : 0;
	case OAM_DMA_SRC_VRAM: return cart.vrambankptr() + (ioamhram[0x146] << 8);
	case OAM_DMA_SRC_WRAM: return cart.wramdata(ioamhram[0x146] >> 4 & 1) + (ioamhram[0x146] << 8 & 0xFFF);
	case OAM_DMA_SRC_INVALID:
	case OAM_DMA_SRC_OFF:  break;
	}
	
	return ioamhram[0x146] == 0xFF && !isCgb() ? oamDmaSrcZero() : cart.rdisabledRam();
}

void Memory::startOamDma(const uint32_t cycleCounter) {
	display.oamChange(cart.rdisabledRam(), cycleCounter);
}

void Memory::endOamDma(const uint32_t cycleCounter) {
	oamDmaPos = 0xFE;
	cart.setOamDmaSrc(OAM_DMA_SRC_OFF);
	display.oamChange(ioamhram, cycleCounter);
}

uint32_t Memory::nontrivial_ff_read(const uint32_t P, const uint32_t cycleCounter) {
	if (lastOamDmaUpdate != DISABLED_TIME)
		updateOamDma(cycleCounter);

	switch (P & 0x7F) {
	case 0x00:
		updateInput();
		break;
	case 0x01:
	case 0x02:
		updateSerial(cycleCounter);
		break;
	case 0x04:
		{
			const uint32_t divcycles = (cycleCounter - divLastUpdate) >> 8;
			ioamhram[0x104] = (ioamhram[0x104] + divcycles) & 0xFF;
			divLastUpdate += divcycles << 8;
		}

		break;
	case 0x05:
		ioamhram[0x105] = tima.tima(cycleCounter);
		break;
	case 0x0F:
		updateIrqs(cycleCounter);
		ioamhram[0x10F] = intreq.ifreg();
		break;
	case 0x26:
		if (ioamhram[0x126] & 0x80) {
			sound.generate_samples(cycleCounter, isDoubleSpeed());
			ioamhram[0x126] = 0xF0 | sound.getStatus();
		} else
			ioamhram[0x126] = 0x70;

		break;
	case 0x30:
	case 0x31:
	case 0x32:
	case 0x33:
	case 0x34:
	case 0x35:
	case 0x36:
	case 0x37:
	case 0x38:
	case 0x39:
	case 0x3A:
	case 0x3B:
	case 0x3C:
	case 0x3D:
	case 0x3E:
	case 0x3F:
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		return sound.waveRamRead(P & 0xF);
	case 0x41:
		return ioamhram[0x141] | display.getStat(ioamhram[0x145], cycleCounter);
	case 0x44:
		return display.getLyReg(cycleCounter/*+4*/);
	case 0x69:
		return display.cgbBgColorRead(ioamhram[0x168] & 0x3F, cycleCounter);
	case 0x6B:
		return display.cgbSpColorRead(ioamhram[0x16A] & 0x3F, cycleCounter);
	default: break;
	}

	return ioamhram[P - 0xFE00];
}

static bool isInOamDmaConflictArea(const OamDmaSrc oamDmaSrc, const uint32_t addr, const bool cgb) {
	struct Area { uint16_t areaUpper, exceptAreaLower, exceptAreaWidth, pad; };
	
	static const Area cgbAreas[] = {
		{ 0xC000, 0x8000, 0x2000, 0 },
		{ 0xC000, 0x8000, 0x2000, 0 },
		{ 0xA000, 0x0000, 0x8000, 0 },
		{ 0xFE00, 0x0000, 0xC000, 0 },
		{ 0xC000, 0x8000, 0x2000, 0 },
		{ 0x0000, 0x0000, 0x0000, 0 }
	};
	
	static const Area dmgAreas[] = {
		{ 0xFE00, 0x8000, 0x2000, 0 },
		{ 0xFE00, 0x8000, 0x2000, 0 },
		{ 0xA000, 0x0000, 0x8000, 0 },
		{ 0xFE00, 0x8000, 0x2000, 0 },
		{ 0xFE00, 0x8000, 0x2000, 0 },
		{ 0x0000, 0x0000, 0x0000, 0 }
	};
	
	const Area *const a = cgb ? cgbAreas : dmgAreas;

	return addr < a[oamDmaSrc].areaUpper && addr - a[oamDmaSrc].exceptAreaLower >= a[oamDmaSrc].exceptAreaWidth;
}

uint32_t Memory::nontrivial_read(const uint32_t P, const uint32_t cycleCounter) {
	if (P < 0xFF80) {
		if (lastOamDmaUpdate != DISABLED_TIME) {
			updateOamDma(cycleCounter);
			
			if (isInOamDmaConflictArea(cart.oamDmaSrc(), P, isCgb()) && oamDmaPos < 0xA0)
				return ioamhram[oamDmaPos];
		}

		if (P < 0xC000) {
			if (P < 0x8000)
				return cart.romdata(P >> 14)[P];

			if (P < 0xA000) {
				if (!display.vramAccessible(cycleCounter))
					return 0xFF;

				return cart.vrambankptr()[P];
			}

			if (cart.rsrambankptr())
				return cart.rsrambankptr()[P];

			return cart.rtcRead();
		}

		if (P < 0xFE00)
			return cart.wramdata(P >> 12 & 1)[P & 0xFFF];

		if (P >= 0xFF00)
			return nontrivial_ff_read(P, cycleCounter);

		if (!display.oamReadable(cycleCounter) || oamDmaPos < 0xA0)
			return 0xFF;
	}

	return ioamhram[P - 0xFE00];
}

uint32_t Memory::nontrivial_peek(const uint32_t P) {
	if (P < 0xC000) {
		if (P < 0x8000)
			return cart.romdata(P >> 14)[P];

		if (P < 0xA000) {
			return cart.vrambankptr()[P];
		}

		if (cart.rsrambankptr())
			return cart.rsrambankptr()[P];

		return cart.rtcRead(); // verified side-effect free
	}
	if (P < 0xFE00)
		return cart.wramdata(P >> 12 & 1)[P & 0xFFF];
	if (P >= 0xFF00 && P < 0xFF80)
		return nontrivial_ff_peek(P);
	return ioamhram[P - 0xFE00];
}

uint32_t Memory::nontrivial_ff_peek(const uint32_t P) {
	// some regs may be somewhat wrong with this
	return ioamhram[P - 0xFE00];
}

void Memory::nontrivial_ff_write(const uint32_t P, uint32_t data, const uint32_t cycleCounter) {
	if (lastOamDmaUpdate != DISABLED_TIME)
		updateOamDma(cycleCounter);

	switch (P & 0xFF) {
	case 0x00:
		if ((data ^ ioamhram[0x100]) & 0x30) {
			ioamhram[0x100] = (ioamhram[0x100] & ~0x30u) | (data & 0x30);
			updateInput();
		}
		return;
	case 0x01:
		updateSerial(cycleCounter);
		break;
	case 0x02:
		updateSerial(cycleCounter);

		serialCnt = 8;
		intreq.setEventTime<SERIAL>((data & 0x81) == 0x81
				? (data & isCgb() * 2 ? (cycleCounter & ~0x7ul) + 0x10 * 8 : (cycleCounter & ~0xFFul) + 0x200 * 8)
				: static_cast<uint32_t>(DISABLED_TIME));

		data |= 0x7E - isCgb() * 2;
		break;
	case 0x04:
		ioamhram[0x104] = 0;
		divLastUpdate = cycleCounter;
		tima.resTac(cycleCounter, TimaInterruptRequester(intreq));
		return;
	case 0x05:
		tima.setTima(data, cycleCounter, TimaInterruptRequester(intreq));
		break;
	case 0x06:
		tima.setTma(data, cycleCounter, TimaInterruptRequester(intreq));
		break;
	case 0x07:
		data |= 0xF8;
		tima.setTac(data, cycleCounter, TimaInterruptRequester(intreq), gbIsCgb_);
		break;
	case 0x0F:
		updateIrqs(cycleCounter);
		intreq.setIfreg(0xE0 | data);
		return;
	case 0x10:
		if (!sound.isEnabled()) return;
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr10(data);
		data |= 0x80;
		break;
	case 0x11:
		if (!sound.isEnabled()) {
			if (isCgb())
				return;

			data &= 0x3F;
		}

		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr11(data);
		data |= 0x3F;
		break;
	case 0x12:
		if (!sound.isEnabled()) return;
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr12(data);
		break;
	case 0x13:
		if (!sound.isEnabled()) return;
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr13(data);
		return;
	case 0x14:
		if (!sound.isEnabled()) return;
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr14(data);
		data |= 0xBF;
		break;
	case 0x16:
		if (!sound.isEnabled()) {
			if (isCgb())
				return;

			data &= 0x3F;
		}

		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr21(data);
		data |= 0x3F;
		break;
	case 0x17:
		if (!sound.isEnabled()) return;
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr22(data);
		break;
	case 0x18:
		return;
	case 0x19:
		if (!sound.isEnabled()) return;
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr24(data);
		data |= 0xBF;
		break;
	case 0x1A:
		if (!sound.isEnabled()) return;
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr30(data);
		data |= 0x7F;
		break;
	case 0x1B:
		if (!sound.isEnabled() && isCgb()) return;
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr31(data);
		return;
	case 0x1C:
		if (!sound.isEnabled()) return;
		data |= 0x9F;
		break;
	case 0x1D:
		if (!sound.isEnabled()) return;
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr33(data);
		return;
	case 0x1E:
		if (!sound.isEnabled()) return;
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr34(data);
		data |= 0xBF;
		break;
	case 0x20:
		if (!sound.isEnabled() && isCgb()) return;
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr41(data);
		return;
	case 0x21:
		if (!sound.isEnabled()) return;
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr42(data);
		break;
	case 0x22:
		if (!sound.isEnabled()) return;
		break;
	case 0x23:
		if (!sound.isEnabled()) return;
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.set_nr44(data);
		data |= 0xBF;
		break;
	case 0x24:
		if (!sound.isEnabled()) return;
		break;
	case 0x25:
		if (!sound.isEnabled()) return;
		break;
	case 0x26:
		if ((ioamhram[0x126] ^ data) & 0x80) {
			sound.generate_samples(cycleCounter, isDoubleSpeed());

			if (!(data & 0x80)) {
				for (uint32_t i = 0xFF10; i < 0xFF26; ++i)
					ff_write(i, 0, cycleCounter);

				sound.setEnabled(false);
			} else {
				sound.reset();
				sound.setEnabled(true);
			}
		}

		data = (data & 0x80) | (ioamhram[0x126] & 0x7F);
		break;
	case 0x30:
	case 0x31:
	case 0x32:
	case 0x33:
	case 0x34:
	case 0x35:
	case 0x36:
	case 0x37:
	case 0x38:
	case 0x39:
	case 0x3A:
	case 0x3B:
	case 0x3C:
	case 0x3D:
	case 0x3E:
	case 0x3F:
		sound.generate_samples(cycleCounter, isDoubleSpeed());
		sound.waveRamWrite(P & 0xF, data);
		break;
	case 0x40:
		if (ioamhram[0x140] != data) {
			if ((ioamhram[0x140] ^ data) & 0x80) {
				const uint32_t lyc = display.getStat(ioamhram[0x145], cycleCounter) & 4;
				const bool hdmaEnabled = display.hdmaIsEnabled();
				
				display.lcdcChange(data, cycleCounter);
				ioamhram[0x144] = 0;
				ioamhram[0x141] &= 0xF8;

				if (data & 0x80) {
					intreq.setEventTime<BLIT>(display.nextMode1IrqTime() + (blanklcd ? 0 : 70224 << isDoubleSpeed()));
				} else {
					ioamhram[0x141] |= lyc;
					intreq.setEventTime<BLIT>(cycleCounter + (456 * 4 << isDoubleSpeed()));

					if (hdmaEnabled)
						flagHdmaReq(&intreq);
				}
			} else
				display.lcdcChange(data, cycleCounter);

			ioamhram[0x140] = data;
		}

		return;
	case 0x41:
		display.lcdstatChange(data, cycleCounter);
		data = (ioamhram[0x141] & 0x87) | (data & 0x78);
		break;
	case 0x42:
		display.scyChange(data, cycleCounter);
		break;
	case 0x43:
		display.scxChange(data, cycleCounter);
		break;
	case 0x45:
		display.lycRegChange(data, cycleCounter);
		break;
	case 0x46:
		if (lastOamDmaUpdate != DISABLED_TIME)
			endOamDma(cycleCounter);

		lastOamDmaUpdate = cycleCounter;
		intreq.setEventTime<OAM>(cycleCounter + 8);
		ioamhram[0x146] = data;
		oamDmaInitSetup();
		return;
	case 0x47:
		if (!isCgb())
			display.dmgBgPaletteChange(data, cycleCounter);

		break;
	case 0x48:
		if (!isCgb())
			display.dmgSpPalette1Change(data, cycleCounter);

		break;
	case 0x49:
		if (!isCgb())
			display.dmgSpPalette2Change(data, cycleCounter);

		break;
	case 0x4A:
		display.wyChange(data, cycleCounter);
		break;
	case 0x4B:
		display.wxChange(data, cycleCounter);
		break;
	case 0x4C:
		break;
	case 0x4D:
		if (isCgb())
			ioamhram[0x14D] = (ioamhram[0x14D] & ~1u) | (data & 1);
		return; // misleading indentation.
	case 0x4F:
		if (isCgb()) {
			cart.setVrambank(data & 1);
			ioamhram[0x14F] = 0xFE | data;
		}

		return;
	case 0x50:
		biosMode = false;
		if (cgbSwitching) {
			display.copyCgbPalettesToDmg();
			display.setCgb(false);
			cgbSwitching = false;
		}
		return;
	case 0x51:
		dmaSource = data << 8 | (dmaSource & 0xFF);
		return;
	case 0x52:
		dmaSource = (dmaSource & 0xFF00) | (data & 0xF0);
		return;
	case 0x53:
		dmaDestination = data << 8 | (dmaDestination & 0xFF);
		return;
	case 0x54:
		dmaDestination = (dmaDestination & 0xFF00) | (data & 0xF0);
		return;
	case 0x55:
		if (isCgb()) {
			ioamhram[0x155] = data & 0x7F;

			if (display.hdmaIsEnabled()) {
				if (!(data & 0x80)) {
					ioamhram[0x155] |= 0x80;
					display.disableHdma(cycleCounter);
				}
			} else {
				if (data & 0x80) {
					if (ioamhram[0x140] & 0x80) {
						display.enableHdma(cycleCounter);
					} else
						flagHdmaReq(&intreq);
				} else
					flagGdmaReq(&intreq);
			}
		}

		return;
	case 0x56:
		if (isCgb())
			ioamhram[0x156] = data | 0x3E;

		return;
	case 0x68:
		if (isCgb())
			ioamhram[0x168] = data | 0x40;

		return;
	case 0x69:
		if (isCgb()) {
			const uint32_t index = ioamhram[0x168] & 0x3F;

			display.cgbBgColorChange(index, data, cycleCounter);

			ioamhram[0x168] = (ioamhram[0x168] & ~0x3F) | ((index + (ioamhram[0x168] >> 7)) & 0x3F);
		}

		return;
	case 0x6A:
		if (isCgb())
			ioamhram[0x16A] = data | 0x40;

		return;
	case 0x6B:
		if (isCgb()) {
			const uint32_t index = ioamhram[0x16A] & 0x3F;

			display.cgbSpColorChange(index, data, cycleCounter);

			ioamhram[0x16A] = (ioamhram[0x16A] & ~0x3F) | ((index + (ioamhram[0x16A] >> 7)) & 0x3F);
		}

		return;
	case 0x6C:
		ioamhram[0x16C] = data | 0xFE;
		cgbSwitching = true;

		return;
	case 0x70:
		if (isCgb()) {
			cart.setWrambank((data & 0x07) ? (data & 0x07) : 1);
			ioamhram[0x170] = data | 0xF8;
		}

		return;
	case 0x72:
	case 0x73:
	case 0x74:
		if (isCgb())
			break;

		return;
	case 0x75:
		if (isCgb())
			ioamhram[0x175] = data | 0x8F;

		return;
	case 0xFF:
		intreq.setIereg(data);
		break;
	default:
		return;
	}

	ioamhram[P - 0xFE00] = data;
}

void Memory::nontrivial_write(const uint32_t P, const uint32_t data, const uint32_t cycleCounter) {
	if (lastOamDmaUpdate != DISABLED_TIME) {
		updateOamDma(cycleCounter);
		
		if (isInOamDmaConflictArea(cart.oamDmaSrc(), P, isCgb()) && oamDmaPos < 0xA0) {
			ioamhram[oamDmaPos] = data;
			return;
		}
	}

	if (P < 0xFE00) {
		if (P < 0xA000) {
			if (P < 0x8000) {
				cart.mbcWrite(P, data);
			} else if (display.vramAccessible(cycleCounter)) {
				display.vramChange(cycleCounter);
				cart.vrambankptr()[P] = data;
			}
		} else if (P < 0xC000) {
			if (cart.wsrambankptr())
				cart.wsrambankptr()[P] = data;
			else
				cart.rtcWrite(data);
		} else
			cart.wramdata(P >> 12 & 1)[P & 0xFFF] = data;
	} else if (P - 0xFF80u >= 0x7Fu) {
		if (P < 0xFF00) {
			if (display.oamWritable(cycleCounter) && oamDmaPos >= 0xA0 && (P < 0xFEA0 || isCgb())) {
				display.oamChange(cycleCounter);
				ioamhram[P - 0xFE00] = data;
			}
		} else
			nontrivial_ff_write(P, data, cycleCounter);
	} else
		ioamhram[P - 0xFE00] = data;
}

int32_t Memory::loadROM(const uint8_t *romfiledata, size_t romfilelength, const bool forceDmg, const bool multicartCompat) {
	if (const int32_t fail = cart.loadROM(romfiledata, romfilelength, forceDmg, multicartCompat))
		return fail;

	sound.init(cart.isCgb());
	display.reset(ioamhram, cart.vramdata(), cart.isCgb());

	return 0;
}

uint32_t Memory::fillSoundBuffer(const uint32_t cycleCounter) {
	sound.generate_samples(cycleCounter, isDoubleSpeed());
	return sound.fillBuffer();
}

bool Memory::getMemoryArea(int32_t which, uint8_t **data, int32_t *length) {
	if (!data || !length)
		return false;

	switch (which)
	{
	case 4: // oam
		*data = &ioamhram[0];
		*length = 160;
		return true;
	case 5: // hram
		*data = &ioamhram[384];
		*length = 128;
		return true;
	case 6: // bgpal
		*data = (uint8_t *)display.bgPalette();
		*length = 32;
		return true;
	case 7: // sppal
		*data = (uint8_t *)display.spPalette();
		*length = 32;
		return true;
	default: // pass to cartridge
		return cart.getMemoryArea(which, data, length);
	}
}

SYNCFUNC(Memory)
{
	SSS(cart);
	NSS(ioamhram);
	NSS(divLastUpdate);
	NSS(lastOamDmaUpdate);
	NSS(biosMode);
	NSS(cgbSwitching);
	NSS(stopped);
	NSS(halttime);

	SSS(intreq);
	SSS(tima);
	SSS(display);
	SSS(sound);

	NSS(dmaSource);
	NSS(dmaDestination);
	NSS(oamDmaPos);
	NSS(serialCnt);
	NSS(blanklcd);

	NSS(LINKCABLE);
	NSS(linkClockTrigger);
}

}
