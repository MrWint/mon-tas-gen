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
#ifndef MEMORY_H
#define MEMORY_H

static unsigned char const agbOverride[0xD] = { 0xFF, 0x00, 0xCD, 0x03, 0x35, 0xAA, 0x31, 0x90, 0x94, 0x00, 0x00, 0x00, 0x00 };

#include "mem/cartridge.h"
#include "video.h"
#include "sound.h"
#include "interrupter.h"
#include "tima.h"
#include "newstate.h"
#include "gambatte.h"

namespace gambatte {
class InputGetter;
class FilterInfo;

class Memory {
	Cartridge cart;
	unsigned char ioamhram[0x200];
	unsigned char cgbBios[0x900];
	unsigned char dmgBios[0x100];
	bool biosMode;
	bool cgbSwitching;
	bool agbMode;
	bool gbIsCgb_;
	bool stopped;
	unsigned short &SP;
	unsigned short &PC;
	unsigned long basetime;
	unsigned long halttime;

	unsigned (*getInput)(void *);
	void *getInputContext;
	unsigned long divLastUpdate;
	unsigned long lastOamDmaUpdate;
	
	InterruptRequester intreq;
	Tima tima;
	LCD display;
	PSG sound;
	Interrupter interrupter;
	
	unsigned short dmaSource;
	unsigned short dmaDestination;
	unsigned char oamDmaPos;
	unsigned char serialCnt;
	bool blanklcd;

	bool LINKCABLE;
	bool linkClockTrigger;

	void decEventCycles(MemEventId eventId, unsigned long dec);

	void oamDmaInitSetup();
	void updateOamDma(unsigned long cycleCounter);
	void startOamDma(unsigned long cycleCounter);
	void endOamDma(unsigned long cycleCounter);
	const unsigned char * oamDmaSrcPtr() const;
	
	unsigned nontrivial_ff_read(unsigned P, unsigned long cycleCounter);
	unsigned nontrivial_read(unsigned P, unsigned long cycleCounter);
	void nontrivial_ff_write(unsigned P, unsigned data, unsigned long cycleCounter);
	void nontrivial_write(unsigned P, unsigned data, unsigned long cycleCounter);
	
	unsigned nontrivial_peek(unsigned P);
	unsigned nontrivial_ff_peek(unsigned P);

	void updateSerial(unsigned long cc);
	void updateTimaIrq(unsigned long cc);
	void updateIrqs(unsigned long cc);
	
	bool isDoubleSpeed() const { return display.isDoubleSpeed(); }

public:
	explicit Memory(const Interrupter &interrupter, unsigned short &sp, unsigned short &pc);
	
	bool loaded() const { return cart.loaded(); }
	unsigned curRomBank() const { return cart.curRomBank(); }

	void setStatePtrs(SaveState &state);
	void loadState(const SaveState &state/*, unsigned long oldCc*/);
	void loadSavedata(const char *data) { cart.loadSavedata(data); }
	int saveSavedataLength() {return cart.saveSavedataLength(); }
	void saveSavedata(char *dest) { cart.saveSavedata(dest); }
	void updateInput();

	unsigned char* cgbBiosBuffer() { return (unsigned char*)cgbBios; }
	unsigned char* dmgBiosBuffer() { return (unsigned char*)dmgBios; }
	bool gbIsCgb() { return gbIsCgb_; }

	bool getMemoryArea(int which, unsigned char **data, int *length); // { return cart.getMemoryArea(which, data, length); }

	unsigned long stop(unsigned long cycleCounter);
	bool isCgb() const { return display.isCgb(); }
	bool ime() const { return intreq.ime(); }
	bool halted() const { return intreq.halted(); }
	unsigned long nextEventTime() const { return intreq.minEventTime(); }

	void setLayers(unsigned mask) { display.setLayers(mask); }
	
	bool isActive() const { return intreq.eventTime(END) != DISABLED_TIME; }
	
	long cyclesSinceBlit(const unsigned long cc) const {
		return cc < intreq.eventTime(BLIT) ? -1 : static_cast<long>((cc - intreq.eventTime(BLIT)) >> isDoubleSpeed());
	}

	void halt(unsigned long cycleCounter) { halttime = cycleCounter; intreq.halt(); }
	void ei(unsigned long cycleCounter) { if (!ime()) { intreq.ei(cycleCounter); } }

	void di() { intreq.di(); }

	unsigned ff_read(const unsigned P, const unsigned long cycleCounter) {
		return P < 0xFF80 ? nontrivial_ff_read(P, cycleCounter) : ioamhram[P - 0xFE00];
	}

	unsigned readBios(const unsigned P) {
		if (gbIsCgb_) {
			if (agbMode && P >= 0xF3 && P < 0x100) {
				return (agbOverride[P - 0xF3] + cgbBios[P]) & 0xFF;
			}
			return cgbBios[P];
		}
		return dmgBios[P];
	}

	unsigned read(const unsigned P, const unsigned long cycleCounter) {
		if (biosMode && ((!gbIsCgb_ && P < 0x100) || (gbIsCgb_ && P < 0x900 && (P < 0x100 || P >= 0x200)))) {
			return readBios(P);
		}
		return cart.rmem(P >> 12) ? cart.rmem(P >> 12)[P] : nontrivial_read(P, cycleCounter);
	}

	unsigned read_excb(const unsigned P, const unsigned long cycleCounter, bool first) {
		if (biosMode && ((!gbIsCgb_ && P < 0x100) || (gbIsCgb_ && P < 0x900 && (P < 0x100 || P >= 0x200)))) {
			return readBios(P);
		}
		return cart.rmem(P >> 12) ? cart.rmem(P >> 12)[P] : nontrivial_read(P, cycleCounter);
	}

	unsigned peek(const unsigned P) {
		if (biosMode && ((!gbIsCgb_ && P < 0x100) || (gbIsCgb_ && P < 0x900 && (P < 0x100 || P >= 0x200)))) {
			return readBios(P);
		}
		return cart.rmem(P >> 12) ? cart.rmem(P >> 12)[P] : nontrivial_peek(P);
	}

	void write_nocb(const unsigned P, const unsigned data, const unsigned long cycleCounter) {
		if (cart.wmem(P >> 12)) {
			cart.wmem(P >> 12)[P] = data;
		} else
			nontrivial_write(P, data, cycleCounter);
	}
	
	void write(const unsigned P, const unsigned data, const unsigned long cycleCounter) {
		if (cart.wmem(P >> 12)) {
			cart.wmem(P >> 12)[P] = data;
		} else
			nontrivial_write(P, data, cycleCounter);
	}
	
	void ff_write(const unsigned P, const unsigned data, const unsigned long cycleCounter) {
		if (P - 0xFF80u < 0x7Fu) {
			ioamhram[P - 0xFE00] = data;
		} else
			nontrivial_ff_write(P, data, cycleCounter);
	}

	unsigned long event(unsigned long cycleCounter);
	unsigned long resetCounters(unsigned long cycleCounter);

	int loadROM(const char *romfiledata, unsigned romfilelength, bool forceDmg, bool multicartCompat);

	void setInputGetter(unsigned (*getInput)(void *), void* context) {
		this->getInput = getInput;
		this->getInputContext = context;
	}

	void setRTCCallback(std::uint32_t (*callback)(void*), void* context) {
		cart.setRTCCallback(callback, context);
	}

	void setBasetime(unsigned long cc) { basetime = cc; }
	void setEndtime(unsigned long cc, unsigned long inc);
	
	void setSoundBuffer() { sound.setBuffer(); }
	unsigned fillSoundBuffer(unsigned long cc);
	
	void setVideoBuffer(uint_least32_t *const videoBuf, const int pitch) {
		display.setVideoBuffer(videoBuf, pitch);
	}
	
	void blackScreen() {
		display.blackScreen();
	}

	std::uint16_t getDivState(unsigned long cc) {
		const std::uint32_t div = ioamhram[0x104];
		return (((div << 8) + cc - divLastUpdate) >> 2) & 0x3FFF;
	}

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
