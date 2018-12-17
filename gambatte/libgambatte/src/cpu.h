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
#ifndef CPU_H
#define CPU_H

#include "memory.h"
#include "newstate.h"

namespace gambatte {

class CPU {
	Memory memory;
	
	uint32_t cycleCounter_;

	uint16_t PC;
	uint16_t SP;
	
	uint32_t HF1, HF2, ZF, CF;

	uint8_t A, B, C, D, E, /*F,*/ H, L;

	bool skip;
	
	int32_t *interruptAddresses;
	uint32_t numInterruptAddresses;
	int32_t hitInterruptAddress;

	void process(uint32_t cycles);

public:
	
	CPU();
// 	void halt();

// 	uint32_t interrupt(uint32_t address, uint32_t cycleCounter);
	
	int32_t runFor(uint32_t cycles);
	void setStatePtrs(SaveState &state);
	void loadState(const SaveState &state);
	void setLayers(uint8_t mask) { memory.setLayers(mask); }
	
	void loadSavedata(const uint8_t *data) { memory.loadSavedata(data); }
	size_t saveSavedataLength() {return memory.saveSavedataLength(); }
	void saveSavedata(uint8_t *dest) { memory.saveSavedata(dest); }
	
	bool getMemoryArea(int32_t which, uint8_t **data, int32_t *length) { return memory.getMemoryArea(which, data, length); }

	void setVideoBuffer(uint32_t *const videoBuf, const int32_t pitch) {
		memory.setVideoBuffer(videoBuf, pitch);
	}
	
	void setInputGetter(uint8_t (*getInput)(void *), void* context) {
		memory.setInputGetter(getInput, context);
	}

	void setRTCCallback(uint32_t (*callback)(void*), void* context) {
		memory.setRTCCallback(callback, context);
	}
	
	int32_t load(const uint8_t *romfiledata, size_t romfilelength, bool forceDmg, bool multicartCompat) {
		return memory.loadROM(romfiledata, romfilelength, forceDmg, multicartCompat);
	}
	
	bool loaded() const { return memory.loaded(); }

	void setSoundBuffer() { memory.setSoundBuffer(); }
	uint32_t fillSoundBuffer() { return memory.fillSoundBuffer(cycleCounter_); }
	
	uint8_t* cgbBiosBuffer() { return memory.cgbBiosBuffer(); }
	uint8_t* dmgBiosBuffer() { return memory.dmgBiosBuffer(); }

	uint8_t ExternalRead(uint16_t addr) { return memory.peek(addr); }
	void ExternalWrite(uint16_t addr, uint8_t val) { memory.write_nocb(addr, val, cycleCounter_); }

	void GetRegs(uint32_t *dest);

	void SetInterruptAddresses(int32_t *addrs, size_t numAddrs);
	inline int32_t GetHitInterruptAddress() { return hitInterruptAddress; }

	uint16_t getDivState() { return memory.getDivState(cycleCounter_); }

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
