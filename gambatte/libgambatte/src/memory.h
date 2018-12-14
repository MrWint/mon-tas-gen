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

#include "mem/cartridge.h"
#include "video.h"
#include "sound.h"
#include "interrupter.h"
#include "tima.h"
#include "newstate.h"
#include "gambatte.h"

static uint8_t const agbOverride[0xD] = { 0xFF, 0x00, 0xCD, 0x03, 0x35, 0xAA, 0x31, 0x90, 0x94, 0x00, 0x00, 0x00, 0x00 };

namespace gambatte {
class InputGetter;
class FilterInfo;

class Memory {
	Cartridge cart;
	uint8_t ioamhram[0x200];
	uint8_t cgbBios[0x900];
	uint8_t dmgBios[0x100];
	bool biosMode;
	bool cgbSwitching;
	bool agbMode;
	bool gbIsCgb_;
	bool stopped;
	uint16_t &SP;
	uint16_t &PC;
	uint32_t basetime;
	uint32_t halttime;

	uint8_t (*getInput)(void *);
	void *getInputContext;
	uint32_t divLastUpdate;
	uint32_t lastOamDmaUpdate;
	
	InterruptRequester intreq;
	Tima tima;
	LCD display;
	PSG sound;
	Interrupter interrupter;
	
	uint16_t dmaSource;
	uint16_t dmaDestination;
	uint8_t oamDmaPos;
	uint8_t serialCnt;
	bool blanklcd;

	bool LINKCABLE;
	bool linkClockTrigger;

	void decEventCycles(MemEventId eventId, uint32_t dec);

	void oamDmaInitSetup();
	void updateOamDma(uint32_t cycleCounter);
	void startOamDma(uint32_t cycleCounter);
	void endOamDma(uint32_t cycleCounter);
	const uint8_t * oamDmaSrcPtr() const;
	
	uint32_t nontrivial_ff_read(uint32_t P, uint32_t cycleCounter);
	uint32_t nontrivial_read(uint32_t P, uint32_t cycleCounter);
	void nontrivial_ff_write(uint32_t P, uint32_t data, uint32_t cycleCounter);
	void nontrivial_write(uint32_t P, uint32_t data, uint32_t cycleCounter);
	
	uint32_t nontrivial_peek(uint32_t P);
	uint32_t nontrivial_ff_peek(uint32_t P);

	void updateSerial(uint32_t cc);
	void updateTimaIrq(uint32_t cc);
	void updateIrqs(uint32_t cc);
	
	bool isDoubleSpeed() const { return display.isDoubleSpeed(); }

public:
	explicit Memory(const Interrupter &interrupter, uint16_t &sp, uint16_t &pc);
	
	bool loaded() const { return cart.loaded(); }
	uint32_t curRomBank() const { return cart.curRomBank(); }

	void setStatePtrs(SaveState &state);
	void loadState(const SaveState &state/*, uint32_t oldCc*/);
	void loadSavedata(const uint8_t *data) { cart.loadSavedata(data); }
	size_t saveSavedataLength() {return cart.saveSavedataLength(); }
	void saveSavedata(uint8_t *dest) { cart.saveSavedata(dest); }
	void updateInput();

	uint8_t* cgbBiosBuffer() { return cgbBios; }
	uint8_t* dmgBiosBuffer() { return dmgBios; }
	bool gbIsCgb() { return gbIsCgb_; }

	bool getMemoryArea(int32_t which, uint8_t **data, int32_t *length); // { return cart.getMemoryArea(which, data, length); }

	uint32_t stop(uint32_t cycleCounter);
	bool isCgb() const { return display.isCgb(); }
	bool ime() const { return intreq.ime(); }
	bool halted() const { return intreq.halted(); }
	uint32_t nextEventTime() const { return intreq.minEventTime(); }

	void setLayers(uint8_t mask) { display.setLayers(mask); }
	
	bool isActive() const { return intreq.eventTime(END) != DISABLED_TIME; }
	
	int32_t cyclesSinceBlit(const uint32_t cc) const {
		return cc < intreq.eventTime(BLIT) ? -1 : static_cast<int32_t>((cc - intreq.eventTime(BLIT)) >> isDoubleSpeed());
	}

	void halt(uint32_t cycleCounter) { halttime = cycleCounter; intreq.halt(); }
	void ei(uint32_t cycleCounter) { if (!ime()) { intreq.ei(cycleCounter); } }

	void di() { intreq.di(); }

	uint32_t ff_read(const uint32_t P, const uint32_t cycleCounter) {
		return P < 0xFF80 ? nontrivial_ff_read(P, cycleCounter) : ioamhram[P - 0xFE00];
	}

	uint32_t readBios(const uint32_t P) {
		if (gbIsCgb_) {
			if (agbMode && P >= 0xF3 && P < 0x100) {
				return (agbOverride[P - 0xF3] + cgbBios[P]) & 0xFF;
			}
			return cgbBios[P];
		}
		return dmgBios[P];
	}

	uint32_t read(const uint32_t P, const uint32_t cycleCounter) {
		if (biosMode && ((!gbIsCgb_ && P < 0x100) || (gbIsCgb_ && P < 0x900 && (P < 0x100 || P >= 0x200)))) {
			return readBios(P);
		}
		return cart.rmem(P >> 12) ? cart.rmem(P >> 12)[P] : nontrivial_read(P, cycleCounter);
	}

	uint32_t read_excb(const uint32_t P, const uint32_t cycleCounter, bool first) {
		if (biosMode && ((!gbIsCgb_ && P < 0x100) || (gbIsCgb_ && P < 0x900 && (P < 0x100 || P >= 0x200)))) {
			return readBios(P);
		}
		return cart.rmem(P >> 12) ? cart.rmem(P >> 12)[P] : nontrivial_read(P, cycleCounter);
	}

	uint32_t peek(const uint32_t P) {
		if (biosMode && ((!gbIsCgb_ && P < 0x100) || (gbIsCgb_ && P < 0x900 && (P < 0x100 || P >= 0x200)))) {
			return readBios(P);
		}
		return cart.rmem(P >> 12) ? cart.rmem(P >> 12)[P] : nontrivial_peek(P);
	}

	void write_nocb(const uint32_t P, const uint32_t data, const uint32_t cycleCounter) {
		if (cart.wmem(P >> 12)) {
			cart.wmem(P >> 12)[P] = data;
		} else
			nontrivial_write(P, data, cycleCounter);
	}
	
	void write(const uint32_t P, const uint32_t data, const uint32_t cycleCounter) {
		if (cart.wmem(P >> 12)) {
			cart.wmem(P >> 12)[P] = data;
		} else
			nontrivial_write(P, data, cycleCounter);
	}
	
	void ff_write(const uint32_t P, const uint32_t data, const uint32_t cycleCounter) {
		if (P - 0xFF80u < 0x7Fu) {
			ioamhram[P - 0xFE00] = data;
		} else
			nontrivial_ff_write(P, data, cycleCounter);
	}

	uint32_t event(uint32_t cycleCounter);
	uint32_t resetCounters(uint32_t cycleCounter);

	int32_t loadROM(const uint8_t *romfiledata, uint32_t romfilelength, bool forceDmg, bool multicartCompat);

	void setInputGetter(uint8_t (*getInput)(void *), void* context) {
		this->getInput = getInput;
		this->getInputContext = context;
	}

	void setRTCCallback(uint32_t (*callback)(void*), void* context) {
		cart.setRTCCallback(callback, context);
	}

	void setBasetime(uint32_t cc) { basetime = cc; }
	void setEndtime(uint32_t cc, uint32_t inc);
	
	void setSoundBuffer() { sound.setBuffer(); }
	uint32_t fillSoundBuffer(uint32_t cc);
	
	void setVideoBuffer(uint32_t *const videoBuf, const int32_t pitch) {
		display.setVideoBuffer(videoBuf, pitch);
	}
	
	void blackScreen() {
		display.blackScreen();
	}

	uint16_t getDivState(uint32_t cc) {
		const uint32_t div = ioamhram[0x104];
		return (((div << 8) + cc - divLastUpdate) >> 2) & 0x3FFF;
	}

	inline bool isInterrupt(uint32_t romAddress) { return cart.isInterrupt(romAddress); }
	inline void setInterrupt(uint32_t romAddress) { cart.setInterrupt(romAddress); }
	inline void clearInterrupt(uint32_t romAddress) { cart.clearInterrupt(romAddress); }

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
