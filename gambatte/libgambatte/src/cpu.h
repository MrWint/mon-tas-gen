//
//   Copyright (C) 2007 by sinamas <sinamas at users.sourceforge.net>
//
//   This program is free software; you can redistribute it and/or modify
//   it under the terms of the GNU General Public License version 2 as
//   published by the Free Software Foundation.
//
//   This program is distributed in the hope that it will be useful,
//   but WITHOUT ANY WARRANTY; without even the implied warranty of
//   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//   GNU General Public License version 2 for more details.
//
//   You should have received a copy of the GNU General Public License
//   version 2 along with this program; if not, write to the
//   Free Software Foundation, Inc.,
//   51 Franklin St, Fifth Floor, Boston, MA  02110-1301, USA.
//

#ifndef CPU_H
#define CPU_H

#include "memory.h"
#include "newstate.h"

namespace gambatte {

class CPU {
public:
	CPU();
	long runFor(unsigned long cycles, bool startsOnFrameBoundaries);
	void setStatePtrs(SaveState &state);
	void loadState(SaveState const &state);
	void setLayers(unsigned mask) { mem_.setLayers(mask); }
	void loadSavedata(char const *data) { mem_.loadSavedata(data, cycleCounter_); }
	int saveSavedataLength() {return mem_.saveSavedataLength(); }
	void saveSavedata(char *dest) { mem_.saveSavedata(dest, cycleCounter_); }

	bool getMemoryArea(int which, unsigned char **data, int *length) { return mem_.getMemoryArea(which, data, length); }

	void setVideoBuffer(uint_least32_t *videoBuf, std::ptrdiff_t pitch) {
		mem_.setVideoBuffer(videoBuf, pitch);
	}

	void setInputGetter(unsigned (*getInput)(void *), void *context) {
		mem_.setInputGetter(getInput, context);
	}

	LoadRes load(char const *romfiledata, unsigned romfilelength, bool forceDmg, bool multicartCompat) {
		return mem_.loadROM(romfiledata, romfilelength, forceDmg, multicartCompat);
	}

	bool loaded() const { return mem_.loaded(); }
	char const * romTitle() const { return mem_.romTitle(); }
	void setSoundBuffer() { mem_.setSoundBuffer(); }
	std::size_t fillSoundBuffer() { return mem_.fillSoundBuffer(cycleCounter_); }
	bool isCgb() const { return mem_.isCgb(); }

	void setDmgPaletteColor(int palNum, int colorNum, unsigned long rgb32) {
		mem_.setDmgPaletteColor(palNum, colorNum, rgb32);
	}

	void setCgbPalette(unsigned *lut) {
		mem_.setCgbPalette(lut);
	}
	void setTimeMode(bool useCycles) { mem_.setTimeMode(useCycles, cycleCounter_); }

	void setBios(char const *buffer, std::size_t size) { mem_.setBios(buffer, size); }
	bool gbIsCgb() { return mem_.gbIsCgb(); }

	unsigned char externalRead(unsigned short addr) {return mem_.peek(addr); }

	void externalWrite(unsigned short addr, unsigned char val) {
		mem_.write_nocb(addr, val, cycleCounter_);
	}

	int linkStatus(int which) { return mem_.linkStatus(which); }

	void getRegs(int *dest);
	void setInterruptAddresses(int *addrs, int numAddrs);
	int getHitInterruptAddress();

	uint16_t getDivState() { return mem_.getDivState(cycleCounter_); }

private:
	Memory mem_;
	unsigned long cycleCounter_;
	unsigned short pc;
	unsigned short sp;
	unsigned hf1, hf2, zf, cf;
	unsigned char a, b, c, d, e, /*f,*/ h, l;
	bool skip_;

	int *interruptAddresses;
	int numInterruptAddresses;
	int hitInterruptAddress;

	void process(unsigned long cycles, bool startsOnFrameBoundaries);

public:
	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
