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
#include "video.h"
#include "savestate.h"
#include <cstring>
#include <algorithm>

namespace gambatte {

void LCD::setDmgPalette(uint32_t *const palette, const uint32_t *const dmgColors, const uint32_t data) {
	palette[0] = dmgColors[data      & 3];
	palette[1] = dmgColors[data >> 2 & 3];
	palette[2] = dmgColors[data >> 4 & 3];
	palette[3] = dmgColors[data >> 6 & 3];
}

uint32_t LCD::gbcToRgb32(const uint32_t bgr15) {
	uint32_t const r = bgr15       & 0x1F;
	uint32_t const g = bgr15 >>  5 & 0x1F;
	uint32_t const b = bgr15 >> 10 & 0x1F;

	// Use manual color conversion instead of palettes.
	return ((r * 13 + g * 2 + b) >> 1) << 16 | (g * 3 + b) << 9 | (r * 3 + g * 2 + b * 11) >> 1;
}


LCD::LCD(const uint8_t *const oamram, const uint8_t *const vram, const VideoInterruptRequester memEventRequester) :
	ppu(nextM0Time_, oamram, vram),
	eventTimes_(memEventRequester),
	statReg(0),
	m2IrqStatReg_(0),
	m1IrqStatReg_(0)
{
	std::memset( bgpData, 0, sizeof  bgpData);
	std::memset(objpData, 0, sizeof objpData);

	for (size_t i = 0; i < sizeof(dmgColorsRgb32) / sizeof(dmgColorsRgb32[0]); ++i)
		setDmgPaletteColor(i, (3 - (i & 3)) * 85 * 0x010101);

	reset(oamram, vram, false);
	setVideoBuffer(0, 160);
}

void LCD::reset(const uint8_t *const oamram, const uint8_t *vram, const bool cgb) {
	ppu.reset(oamram, vram, cgb);
	lycIrq.setCgb(cgb);
	refreshPalettes();
}

void LCD::setCgb(bool cgb) {
	ppu.setCgb(cgb);
}

static uint32_t mode2IrqSchedule(const uint32_t statReg, const LyCounter &lyCounter, const uint32_t cycleCounter) {
	if (!(statReg & 0x20))
		return DISABLED_TIME;
	
	uint32_t next = lyCounter.time() - cycleCounter;
	
	if (lyCounter.ly() >= 143 || (lyCounter.ly() == 142 && next <= 4) || (statReg & 0x08)) {
		next += (153u - lyCounter.ly()) * lyCounter.lineTime();
	} else {
		if (next <= 4)
			next += lyCounter.lineTime();
		
		next -= 4;
	}
	
	return cycleCounter + next;
}

static inline uint32_t m0IrqTimeFromXpos166Time(const uint32_t xpos166Time, const bool cgb, const bool ds) {
	return xpos166Time + cgb - ds;
}

static inline uint32_t hdmaTimeFromM0Time(const uint32_t m0Time, const bool ds) {
	return m0Time + 1 - ds;
}

static uint32_t nextHdmaTime(const uint32_t lastM0Time,
		const uint32_t nextM0Time, const uint32_t cycleCounter, const bool ds) {
	return cycleCounter < hdmaTimeFromM0Time(lastM0Time, ds)
	                    ? hdmaTimeFromM0Time(lastM0Time, ds)
	                    : hdmaTimeFromM0Time(nextM0Time, ds);
}

void LCD::setStatePtrs(SaveState &state) {
	state.ppu.bgpData.set(  bgpData, sizeof  bgpData);
	state.ppu.objpData.set(objpData, sizeof objpData);
	ppu.setStatePtrs(state);
}

void LCD::loadState(const SaveState &state, const uint8_t *const oamram) {
	statReg = state.mem.ioamhram.get()[0x141];
	m2IrqStatReg_ = statReg;
	m1IrqStatReg_ = statReg;

	ppu.loadState(state, oamram);
	lycIrq.loadState(state);
	m0Irq_.loadState(state);

	if (ppu.lcdc() & 0x80) {
		nextM0Time_.predictNextM0Time(ppu);
		lycIrq.reschedule(ppu.lyCounter(), ppu.now());
		
		eventTimes_.setm<ONESHOT_LCDSTATIRQ>(state.ppu.pendingLcdstatIrq
							? ppu.now() + 1 : static_cast<uint32_t>(DISABLED_TIME));
		eventTimes_.setm<ONESHOT_UPDATEWY2>(state.ppu.oldWy != state.mem.ioamhram.get()[0x14A]
							? ppu.now() + 1 : static_cast<uint32_t>(DISABLED_TIME));
		eventTimes_.set<LY_COUNT>(ppu.lyCounter().time());
		eventTimes_.setm<SPRITE_MAP>(SpriteMapper::schedule(ppu.lyCounter(), ppu.now()));
		eventTimes_.setm<LYC_IRQ>(lycIrq.time());
		eventTimes_.setm<MODE1_IRQ>(ppu.lyCounter().nextFrameCycle(144 * 456, ppu.now()));
		eventTimes_.setm<MODE2_IRQ>(mode2IrqSchedule(statReg, ppu.lyCounter(), ppu.now()));
		eventTimes_.setm<MODE0_IRQ>((statReg & 0x08) ? ppu.now() + state.ppu.nextM0Irq : static_cast<uint32_t>(DISABLED_TIME));
		eventTimes_.setm<HDMA_REQ>(state.mem.hdmaTransfer
				? nextHdmaTime(ppu.lastM0Time(), nextM0Time_.predictedNextM0Time(), ppu.now(), isDoubleSpeed())
				: static_cast<uint32_t>(DISABLED_TIME));
	} else for (int32_t i = 0; i < NUM_MEM_EVENTS; ++i)
		eventTimes_.set(static_cast<MemEvent>(i), DISABLED_TIME);
	
	refreshPalettes();
}

void LCD::refreshPalettes() {
	if (ppu.cgb()) {
		for (uint32_t i = 0; i < 8 * 8; i += 2) {
			ppu.bgPalette()[i >> 1] = gbcToRgb32( bgpData[i] |  bgpData[i + 1] << 8);
			ppu.spPalette()[i >> 1] = gbcToRgb32(objpData[i] | objpData[i + 1] << 8);
		}
	} else {
		setDmgPalette(ppu.bgPalette()    , dmgColorsRgb32    ,  bgpData[0]);
		setDmgPalette(ppu.spPalette()    , dmgColorsRgb32 + 4, objpData[0]);
		setDmgPalette(ppu.spPalette() + 4, dmgColorsRgb32 + 8, objpData[1]);
	}
}

void LCD::copyCgbPalettesToDmg() {
	for (uint32_t i = 0; i < 4; i++) {
		dmgColorsRgb32[i] = gbcToRgb32(bgpData[i * 2] | bgpData[i * 2 + 1] << 8);
	}
	for (uint32_t i = 0; i < 8; i++) {
		dmgColorsRgb32[i + 4] = gbcToRgb32(objpData[i * 2] | objpData[i * 2 + 1] << 8);
	}
}

void LCD::blackScreen() {
	if (ppu.cgb()) {
		for (uint32_t i = 0; i < 8 * 8; i += 2) {
			ppu.bgPalette()[i >> 1] = 0;
			ppu.spPalette()[i >> 1] = 0;
		}
	}
	else {
		for (uint32_t i = 0; i < 4; i++) {
			dmgColorsRgb32[i] = 0;
		}
		for (uint32_t i = 0; i < 8; i++) {
			dmgColorsRgb32[i + 4] = 0;
		}
	}
}

namespace {

template<typename T>
static void clear(T *buf, const uint32_t color, const int32_t dpitch) {
	uint32_t lines = 144;

	while (lines--) {
		std::fill_n(buf, 160, color);
		buf += dpitch;
	}
}

}

void LCD::updateScreen(const bool blanklcd, const uint32_t cycleCounter) {
	update(cycleCounter);
	
	if (blanklcd && ppu.frameBuf().fb()) {
		const uint32_t color = ppu.cgb() ? gbcToRgb32(0xFFFF) : dmgColorsRgb32[0];
		clear(ppu.frameBuf().fb(), color, ppu.frameBuf().pitch());
	}
}

void LCD::resetCc(const uint32_t oldCc, const uint32_t newCc) {
	update(oldCc);
	ppu.resetCc(oldCc, newCc);
	
	if (ppu.lcdc() & 0x80) {
		const uint32_t dec = oldCc - newCc;
		
		nextM0Time_.invalidatePredictedNextM0Time();
		lycIrq.reschedule(ppu.lyCounter(), newCc);
		
		for (int32_t i = 0; i < NUM_MEM_EVENTS; ++i) {
			if (eventTimes_(static_cast<MemEvent>(i)) != DISABLED_TIME)
				eventTimes_.set(static_cast<MemEvent>(i), eventTimes_(static_cast<MemEvent>(i)) - dec);
		}
		
		eventTimes_.set<LY_COUNT>(ppu.lyCounter().time());
	}
}

void LCD::speedChange(const uint32_t cycleCounter) {
	update(cycleCounter);
	ppu.speedChange(cycleCounter);
	
	if (ppu.lcdc() & 0x80) {
		nextM0Time_.predictNextM0Time(ppu);
		lycIrq.reschedule(ppu.lyCounter(), cycleCounter);
		
		eventTimes_.set<LY_COUNT>(ppu.lyCounter().time());
		eventTimes_.setm<SPRITE_MAP>(SpriteMapper::schedule(ppu.lyCounter(), cycleCounter));
		eventTimes_.setm<LYC_IRQ>(lycIrq.time());
		eventTimes_.setm<MODE1_IRQ>(ppu.lyCounter().nextFrameCycle(144 * 456, cycleCounter));
		eventTimes_.setm<MODE2_IRQ>(mode2IrqSchedule(statReg, ppu.lyCounter(), cycleCounter));
		
		if (eventTimes_(MODE0_IRQ) != DISABLED_TIME && eventTimes_(MODE0_IRQ) - cycleCounter > 1)
			eventTimes_.setm<MODE0_IRQ>(m0IrqTimeFromXpos166Time(ppu.predictedNextXposTime(166), ppu.cgb(), isDoubleSpeed()));
		
		if (hdmaIsEnabled() && eventTimes_(HDMA_REQ) - cycleCounter > 1) {
			eventTimes_.setm<HDMA_REQ>(nextHdmaTime(ppu.lastM0Time(),
					nextM0Time_.predictedNextM0Time(), cycleCounter, isDoubleSpeed()));
		}
	}
}

static inline uint32_t m0TimeOfCurrentLine(const uint32_t nextLyTime,
		const uint32_t lastM0Time, const uint32_t nextM0Time)
{
	return nextM0Time < nextLyTime ? nextM0Time : lastM0Time;
}

uint32_t LCD::m0TimeOfCurrentLine(const uint32_t cc) {
	if (cc >= nextM0Time_.predictedNextM0Time()) {
		update(cc);
		nextM0Time_.predictNextM0Time(ppu);
	}
	
	return gambatte::m0TimeOfCurrentLine(ppu.lyCounter().time(), ppu.lastM0Time(), nextM0Time_.predictedNextM0Time());
}

static bool isHdmaPeriod(const LyCounter &lyCounter,
		const uint32_t m0TimeOfCurrentLy, const uint32_t cycleCounter)
{
	const uint32_t timeToNextLy = lyCounter.time() - cycleCounter;
	
	return /*(ppu.lcdc & 0x80) && */lyCounter.ly() < 144 && timeToNextLy > 4
			&& cycleCounter >= hdmaTimeFromM0Time(m0TimeOfCurrentLy, lyCounter.isDoubleSpeed());
}

void LCD::enableHdma(const uint32_t cycleCounter) {
	if (cycleCounter >= nextM0Time_.predictedNextM0Time()) {
		update(cycleCounter);
		nextM0Time_.predictNextM0Time(ppu);
	} else if (cycleCounter >= eventTimes_.nextEventTime())
		update(cycleCounter);
	
	if (isHdmaPeriod(ppu.lyCounter(),
			gambatte::m0TimeOfCurrentLine(ppu.lyCounter().time(),
				ppu.lastM0Time(), nextM0Time_.predictedNextM0Time()), cycleCounter)) {
		eventTimes_.flagHdmaReq();
	}
	
	eventTimes_.setm<HDMA_REQ>(nextHdmaTime(ppu.lastM0Time(), nextM0Time_.predictedNextM0Time(), cycleCounter, isDoubleSpeed()));
}

void LCD::disableHdma(const uint32_t cycleCounter) {
	if (cycleCounter >= eventTimes_.nextEventTime())
		update(cycleCounter);
	
	eventTimes_.setm<HDMA_REQ>(DISABLED_TIME);
}

bool LCD::vramAccessible(const uint32_t cycleCounter) {
	if (cycleCounter >= eventTimes_.nextEventTime())
		update(cycleCounter);
	
	return !(ppu.lcdc() & 0x80) || ppu.lyCounter().ly() >= 144
			|| ppu.lyCounter().lineCycles(cycleCounter) < 80U
			|| cycleCounter + isDoubleSpeed() - ppu.cgb() + 2 >= m0TimeOfCurrentLine(cycleCounter);
}

bool LCD::cgbpAccessible(const uint32_t cycleCounter) {
	if (cycleCounter >= eventTimes_.nextEventTime())
		update(cycleCounter);
	
	return !(ppu.lcdc() & 0x80) || ppu.lyCounter().ly() >= 144
			|| ppu.lyCounter().lineCycles(cycleCounter) < 80U + isDoubleSpeed()
			|| cycleCounter >= m0TimeOfCurrentLine(cycleCounter) + 3 - isDoubleSpeed();
}

void LCD::doCgbColorChange(uint8_t *const pdata,
		uint32_t *const palette, uint32_t index, const uint32_t data) {
	pdata[index] = data;
	index >>= 1;
	palette[index] = gbcToRgb32(pdata[index << 1] | pdata[(index << 1) + 1] << 8);
}

void LCD::doCgbBgColorChange(uint32_t index, const uint32_t data, const uint32_t cycleCounter) {
	if (cgbpAccessible(cycleCounter)) {
		update(cycleCounter);
		doCgbColorChange(bgpData, ppu.bgPalette(), index, data);
	}
}

void LCD::doCgbSpColorChange(uint32_t index, const uint32_t data, const uint32_t cycleCounter) {
	if (cgbpAccessible(cycleCounter)) {
		update(cycleCounter);
		doCgbColorChange(objpData, ppu.spPalette(), index, data);
	}
}

bool LCD::oamReadable(const uint32_t cycleCounter) {
	if (!(ppu.lcdc() & 0x80) || ppu.inactivePeriodAfterDisplayEnable(cycleCounter))
		return true;
	
	if (cycleCounter >= eventTimes_.nextEventTime())
		update(cycleCounter);

	if (ppu.lyCounter().lineCycles(cycleCounter) + 4 - ppu.lyCounter().isDoubleSpeed() * 3u >= 456)
		return ppu.lyCounter().ly() >= 144-1 && ppu.lyCounter().ly() != 153;

	return ppu.lyCounter().ly() >= 144 || cycleCounter + isDoubleSpeed() - ppu.cgb() + 2 >= m0TimeOfCurrentLine(cycleCounter);
}

bool LCD::oamWritable(const uint32_t cycleCounter) {
	if (!(ppu.lcdc() & 0x80) || ppu.inactivePeriodAfterDisplayEnable(cycleCounter))
		return true;
	
	if (cycleCounter >= eventTimes_.nextEventTime())
		update(cycleCounter);

	if (ppu.lyCounter().lineCycles(cycleCounter) + 3 + ppu.cgb() - ppu.lyCounter().isDoubleSpeed() * 2u >= 456)
		return ppu.lyCounter().ly() >= 144-1 && ppu.lyCounter().ly() != 153;

	return ppu.lyCounter().ly() >= 144 || cycleCounter + isDoubleSpeed() - ppu.cgb() + 2 >= m0TimeOfCurrentLine(cycleCounter);
}

void LCD::mode3CyclesChange() {
	nextM0Time_.invalidatePredictedNextM0Time();

	if (eventTimes_(MODE0_IRQ) != DISABLED_TIME
			&& eventTimes_(MODE0_IRQ) > m0IrqTimeFromXpos166Time(ppu.now(), ppu.cgb(), isDoubleSpeed())) {
		eventTimes_.setm<MODE0_IRQ>(m0IrqTimeFromXpos166Time(ppu.predictedNextXposTime(166), ppu.cgb(), isDoubleSpeed()));
	}

	if (eventTimes_(HDMA_REQ) != DISABLED_TIME
			&& eventTimes_(HDMA_REQ) > hdmaTimeFromM0Time(ppu.lastM0Time(), isDoubleSpeed())) {
		nextM0Time_.predictNextM0Time(ppu);
		eventTimes_.setm<HDMA_REQ>(hdmaTimeFromM0Time(nextM0Time_.predictedNextM0Time(), isDoubleSpeed()));
	}
}

void LCD::wxChange(const uint32_t newValue, const uint32_t cycleCounter) {
	update(cycleCounter + isDoubleSpeed() + 1);
	ppu.setWx(newValue);
	mode3CyclesChange();
}

void LCD::wyChange(const uint32_t newValue, const uint32_t cycleCounter) {
	update(cycleCounter + 1);
	ppu.setWy(newValue);
// 	mode3CyclesChange(); // should be safe to wait until after wy2 delay, because no mode3 events are close to when wy1 is read.
	
	// wy2 is a delayed version of wy. really just slowness of ly == wy comparison.
	if (ppu.cgb() && (ppu.lcdc() & 0x80)) {
		eventTimes_.setm<ONESHOT_UPDATEWY2>(cycleCounter + 5);
	} else {
		update(cycleCounter + 2);
		ppu.updateWy2();
		mode3CyclesChange();
	}
}

void LCD::scxChange(const uint32_t newScx, const uint32_t cycleCounter) {
	update(cycleCounter + ppu.cgb() + isDoubleSpeed());
	ppu.setScx(newScx);
	mode3CyclesChange();
}

void LCD::scyChange(const uint32_t newValue, const uint32_t cycleCounter) {
	update(cycleCounter + ppu.cgb() + isDoubleSpeed());
	ppu.setScy(newValue);
}

void LCD::oamChange(const uint32_t cycleCounter) {
	if (ppu.lcdc() & 0x80) {
		update(cycleCounter);
		ppu.oamChange(cycleCounter);
		eventTimes_.setm<SPRITE_MAP>(SpriteMapper::schedule(ppu.lyCounter(), cycleCounter));
	}
}

void LCD::oamChange(const uint8_t *const oamram, const uint32_t cycleCounter) {
	update(cycleCounter);
	ppu.oamChange(oamram, cycleCounter);
	
	if (ppu.lcdc() & 0x80)
		eventTimes_.setm<SPRITE_MAP>(SpriteMapper::schedule(ppu.lyCounter(), cycleCounter));
}

void LCD::lcdcChange(const uint32_t data, const uint32_t cycleCounter) {
	const uint32_t oldLcdc = ppu.lcdc();
	update(cycleCounter);
	
	if ((oldLcdc ^ data) & 0x80) {
		ppu.setLcdc(data, cycleCounter);
		
		if (data & 0x80) {
			lycIrq.lcdReset();
			m0Irq_.lcdReset(statReg, lycIrq.lycReg());
			
			if (lycIrq.lycReg() == 0 && (statReg & 0x40))
				eventTimes_.flagIrq(2);

			nextM0Time_.predictNextM0Time(ppu);
			lycIrq.reschedule(ppu.lyCounter(), cycleCounter);
			
			eventTimes_.set<LY_COUNT>(ppu.lyCounter().time());
			eventTimes_.setm<SPRITE_MAP>(SpriteMapper::schedule(ppu.lyCounter(), cycleCounter));
			eventTimes_.setm<LYC_IRQ>(lycIrq.time());
			eventTimes_.setm<MODE1_IRQ>(ppu.lyCounter().nextFrameCycle(144 * 456, cycleCounter));
			eventTimes_.setm<MODE2_IRQ>(mode2IrqSchedule(statReg, ppu.lyCounter(), cycleCounter));
			
			if (statReg & 0x08)
				eventTimes_.setm<MODE0_IRQ>(m0IrqTimeFromXpos166Time(ppu.predictedNextXposTime(166), ppu.cgb(), isDoubleSpeed()));
			
			if (hdmaIsEnabled()) {
				eventTimes_.setm<HDMA_REQ>(nextHdmaTime(ppu.lastM0Time(),
						nextM0Time_.predictedNextM0Time(), cycleCounter, isDoubleSpeed()));
			}
		} else for (int32_t i = 0; i < NUM_MEM_EVENTS; ++i)
			eventTimes_.set(static_cast<MemEvent>(i), DISABLED_TIME);
	} else if (data & 0x80) {
		if (ppu.cgb()) {
			ppu.setLcdc((oldLcdc & ~0x14) | (data & 0x14), cycleCounter);
			
			if ((oldLcdc ^ data) & 0x04)
				eventTimes_.setm<SPRITE_MAP>(SpriteMapper::schedule(ppu.lyCounter(), cycleCounter));
			
			update(cycleCounter + isDoubleSpeed() + 1);
			ppu.setLcdc(data, cycleCounter + isDoubleSpeed() + 1);
			
			if ((oldLcdc ^ data) & 0x20)
				mode3CyclesChange();
		} else {
			ppu.setLcdc(data, cycleCounter);
			
			if ((oldLcdc ^ data) & 0x04)
				eventTimes_.setm<SPRITE_MAP>(SpriteMapper::schedule(ppu.lyCounter(), cycleCounter));
			
			if ((oldLcdc ^ data) & 0x22)
				mode3CyclesChange();
		}
	} else
		ppu.setLcdc(data, cycleCounter);
}

namespace {
struct LyCnt {
	uint32_t ly; int32_t timeToNextLy;
	LyCnt(uint32_t ly, int32_t timeToNextLy) : ly(ly), timeToNextLy(timeToNextLy) {}
};

static LyCnt const getLycCmpLy(LyCounter const &lyCounter, uint32_t cc) {
	uint32_t ly = lyCounter.ly();
	int32_t timeToNextLy = lyCounter.time() - cc;

	if (ly == 153) {
		if (timeToNextLy -  (448 << lyCounter.isDoubleSpeed()) > 0) {
			timeToNextLy -= (448 << lyCounter.isDoubleSpeed());
		} else {
			ly = 0;
			timeToNextLy += lyCounter.lineTime();
		}
	}

	return LyCnt(ly, timeToNextLy);
}
}

void LCD::lcdstatChange(uint32_t const data, uint32_t const cycleCounter) {
	if (cycleCounter >= eventTimes_.nextEventTime())
		update(cycleCounter);

	uint32_t const old = statReg;
	statReg = data;
	lycIrq.statRegChange(data, ppu.lyCounter(), cycleCounter);
	
	if (ppu.lcdc() & 0x80) {
		int32_t const timeToNextLy = ppu.lyCounter().time() - cycleCounter;
		LyCnt const lycCmp = getLycCmpLy(ppu.lyCounter(), cycleCounter);

		if (!ppu.cgb()) {
			if (ppu.lyCounter().ly() < 144) {
				if (cycleCounter + 1 < m0TimeOfCurrentLine(cycleCounter)) {
					if (lycCmp.ly == lycIrq.lycReg() && !(old & 0x40))
						eventTimes_.flagIrq(2);
				} else {
					if (!(old & 0x08) && !(lycCmp.ly == lycIrq.lycReg() && (old & 0x40)))
						eventTimes_.flagIrq(2);
				}
			} else {
				if (!(old & 0x10) && !(lycCmp.ly == lycIrq.lycReg() && (old & 0x40)))
					eventTimes_.flagIrq(2);
			}
		} else if (data & ~old & 0x78) {
			bool const lycperiod = lycCmp.ly == lycIrq.lycReg() && lycCmp.timeToNextLy > 4 - isDoubleSpeed() * 4;

			if (!(lycperiod && (old & 0x40))) {
				if (ppu.lyCounter().ly() < 144) {
					if (cycleCounter + isDoubleSpeed() * 2 < m0TimeOfCurrentLine(cycleCounter) || timeToNextLy <= 4) {
						if (lycperiod && (data & 0x40))
							eventTimes_.flagIrq(2);
					} else if (!(old & 0x08)) {
						if ((data & 0x08) || (lycperiod && (data & 0x40)))
							eventTimes_.flagIrq(2);
					}
				} else if (!(old & 0x10)) {
					if ((data & 0x10) && (ppu.lyCounter().ly() < 153 || timeToNextLy > 4 - isDoubleSpeed() * 4)) {
						eventTimes_.flagIrq(2);
					} else if (lycperiod && (data & 0x40))
						eventTimes_.flagIrq(2);
				}
			}

			if ((data & 0x28) == 0x20 && !(old & 0x20)
					&& ((timeToNextLy <= 4 && ppu.lyCounter().ly() < 143)
					    || (timeToNextLy == 456*2 && ppu.lyCounter().ly() < 144))) {
				eventTimes_.flagIrq(2);
			}
		}

		if ((data & 0x08) && eventTimes_(MODE0_IRQ) == DISABLED_TIME) {
			update(cycleCounter);
			eventTimes_.setm<MODE0_IRQ>(m0IrqTimeFromXpos166Time(ppu.predictedNextXposTime(166), ppu.cgb(), isDoubleSpeed()));
		}

		eventTimes_.setm<MODE2_IRQ>(mode2IrqSchedule(data, ppu.lyCounter(), cycleCounter));
		eventTimes_.setm<LYC_IRQ>(lycIrq.time());
	}
	
	m2IrqStatReg_ = eventTimes_(MODE2_IRQ) - cycleCounter > (ppu.cgb() - isDoubleSpeed()) * 4U
			? data : (m2IrqStatReg_ & 0x10) | (statReg & ~0x10);
	m1IrqStatReg_ = eventTimes_(MODE1_IRQ) - cycleCounter > (ppu.cgb() - isDoubleSpeed()) * 4U
			? data : (m1IrqStatReg_ & 0x08) | (statReg & ~0x08);
	
	m0Irq_.statRegChange(data, eventTimes_(MODE0_IRQ), cycleCounter, ppu.cgb());
}

void LCD::lycRegChange(uint32_t const data, uint32_t const cycleCounter) {
	uint32_t const old = lycIrq.lycReg();

	if (data == old)
		return;

	if (cycleCounter >= eventTimes_.nextEventTime())
		update(cycleCounter);

	m0Irq_.lycRegChange(data, eventTimes_(MODE0_IRQ), cycleCounter, isDoubleSpeed(), ppu.cgb());	
	lycIrq.lycRegChange(data, ppu.lyCounter(), cycleCounter);
	
	if (!(ppu.lcdc() & 0x80))
		return;
	
	eventTimes_.setm<LYC_IRQ>(lycIrq.time());

	int32_t const timeToNextLy = ppu.lyCounter().time() - cycleCounter;
	
	if ((statReg & 0x40) && data < 154
			&& (ppu.lyCounter().ly() < 144
			    ? !(statReg & 0x08) || cycleCounter < m0TimeOfCurrentLine(cycleCounter) || timeToNextLy <= 4 << ppu.cgb()
			    : !(statReg & 0x10) || (ppu.lyCounter().ly() == 153 && timeToNextLy <= 4 && ppu.cgb() && !isDoubleSpeed()))) {
		LyCnt lycCmp = getLycCmpLy(ppu.lyCounter(), cycleCounter);

		if (lycCmp.timeToNextLy <= 4 << ppu.cgb()) {
			lycCmp.ly = old != lycCmp.ly || (lycCmp.timeToNextLy <= 4 && ppu.cgb() && !isDoubleSpeed())
			          ? (lycCmp.ly == 153 ? 0 : lycCmp.ly + 1)
			          : 0xFF; // simultaneous ly/lyc inc. lyc flag never goes low -> no trigger.
		}

		if (data == lycCmp.ly) {
			if (ppu.cgb() && !isDoubleSpeed()) {
				eventTimes_.setm<ONESHOT_LCDSTATIRQ>(cycleCounter + 5);
			} else
				eventTimes_.flagIrq(2);
		}
	}
}

uint8_t LCD::getStat(uint32_t const lycReg, uint32_t const cycleCounter) {
	uint8_t stat = 0;

	if (ppu.lcdc() & 0x80) {
		if (cycleCounter >= eventTimes_.nextEventTime())
			update(cycleCounter);

		int32_t const timeToNextLy = ppu.lyCounter().time() - cycleCounter;

		if (ppu.lyCounter().ly() > 143) {
			if (ppu.lyCounter().ly() < 153 || timeToNextLy > 4 - isDoubleSpeed() * 4)
				stat = 1;
		} else {
			uint32_t const lineCycles = 456 - (timeToNextLy >> isDoubleSpeed());

			if (lineCycles < 80) {
				if (!ppu.inactivePeriodAfterDisplayEnable(cycleCounter))
					stat = 2;
			} else if (cycleCounter + isDoubleSpeed() - ppu.cgb() + 2 < m0TimeOfCurrentLine(cycleCounter))
				stat = 3;
		}

		LyCnt const lycCmp = getLycCmpLy(ppu.lyCounter(), cycleCounter);

		if (lycReg == lycCmp.ly && lycCmp.timeToNextLy > 4 - isDoubleSpeed() * 4)
			stat |= 4;
	}

	return stat;
}

inline void LCD::doMode2IrqEvent() {
	const uint32_t ly = eventTimes_(LY_COUNT) - eventTimes_(MODE2_IRQ) < 8
			? (ppu.lyCounter().ly() == 153 ? 0 : ppu.lyCounter().ly() + 1)
			: ppu.lyCounter().ly();
	
	if ((ly != 0 || !(m2IrqStatReg_ & 0x10)) &&
			(!(m2IrqStatReg_ & 0x40) || (lycIrq.lycReg() != 0 ? ly != (lycIrq.lycReg() + 1U) : ly > 1))) {
		eventTimes_.flagIrq(2);
	}
	
	m2IrqStatReg_ = statReg;
	
	if (!(statReg & 0x08)) {
		uint32_t nextTime = eventTimes_(MODE2_IRQ) + ppu.lyCounter().lineTime();
		
		if (ly == 0) {
			nextTime -= 4;
		} else if (ly == 143)
			nextTime += ppu.lyCounter().lineTime() * 10 + 4;
		
		eventTimes_.setm<MODE2_IRQ>(nextTime);
	} else
		eventTimes_.setm<MODE2_IRQ>(eventTimes_(MODE2_IRQ) + (70224 << isDoubleSpeed()));
}

inline void LCD::event() {
	switch (eventTimes_.nextEvent()) {
	case MEM_EVENT:
		switch (eventTimes_.nextMemEvent()) {
		case MODE1_IRQ:
			eventTimes_.flagIrq((m1IrqStatReg_ & 0x18) == 0x10 ? 3 : 1);
			m1IrqStatReg_ = statReg;
			eventTimes_.setm<MODE1_IRQ>(eventTimes_(MODE1_IRQ) + (70224 << isDoubleSpeed()));
			break;
			
		case LYC_IRQ: {
			uint8_t ifreg = 0;
			lycIrq.doEvent(&ifreg, ppu.lyCounter());
			eventTimes_.flagIrq(ifreg);
			eventTimes_.setm<LYC_IRQ>(lycIrq.time());
			break;
		}
		
		case SPRITE_MAP:
			eventTimes_.setm<SPRITE_MAP>(ppu.doSpriteMapEvent(eventTimes_(SPRITE_MAP)));
			mode3CyclesChange();
			break;
		
		case HDMA_REQ:
			eventTimes_.flagHdmaReq();
			nextM0Time_.predictNextM0Time(ppu);
			eventTimes_.setm<HDMA_REQ>(hdmaTimeFromM0Time(nextM0Time_.predictedNextM0Time(), isDoubleSpeed()));
			break;
		
		case MODE2_IRQ:
			doMode2IrqEvent();
			break;
		
		case MODE0_IRQ:
			{
				uint8_t ifreg = 0;
				m0Irq_.doEvent(&ifreg, ppu.lyCounter().ly(), statReg, lycIrq.lycReg());
				eventTimes_.flagIrq(ifreg);
			}
			
			eventTimes_.setm<MODE0_IRQ>((statReg & 0x08)
					? m0IrqTimeFromXpos166Time(ppu.predictedNextXposTime(166), ppu.cgb(), isDoubleSpeed())
					: static_cast<uint32_t>(DISABLED_TIME));
			break;
		
		case ONESHOT_LCDSTATIRQ:
			eventTimes_.flagIrq(2);
			eventTimes_.setm<ONESHOT_LCDSTATIRQ>(DISABLED_TIME);
			break;
		
		case ONESHOT_UPDATEWY2:
			ppu.updateWy2();
			mode3CyclesChange();
			eventTimes_.setm<ONESHOT_UPDATEWY2>(DISABLED_TIME);
			break;
		}
		
		break;
		
	case LY_COUNT:
		ppu.doLyCountEvent();
		eventTimes_.set<LY_COUNT>(ppu.lyCounter().time());
		break;
	}
}

void LCD::update(const uint32_t cycleCounter) {
	if (!(ppu.lcdc() & 0x80))
		return;
	
	while (cycleCounter >= eventTimes_.nextEventTime()) {
		ppu.update(eventTimes_.nextEventTime());
		event();
	}
	
	ppu.update(cycleCounter);
}

void LCD::setVideoBuffer(uint32_t *const videoBuf, const size_t pitch) {
	ppu.setFrameBuf(videoBuf, pitch);
}

void LCD::setDmgPaletteColor(const uint32_t index, const uint32_t rgb32) {
	dmgColorsRgb32[index] = rgb32;
}

// don't need to save or load rgb32 color data

SYNCFUNC(LCD)
{
	SSS(ppu);
	NSS(bgpData);
	NSS(objpData);
	SSS(eventTimes_);
	SSS(m0Irq_);
	SSS(lycIrq);
	SSS(nextM0Time_);

	NSS(statReg);
	NSS(m2IrqStatReg_);
	NSS(m1IrqStatReg_);
}

}
