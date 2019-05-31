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

#ifndef MEMORY_H
#define MEMORY_H

static unsigned char const agbOverride[0xD] = { 0xFF, 0x00, 0xCD, 0x03, 0x35, 0xAA, 0x31, 0x90, 0x94, 0x00, 0x00, 0x00, 0x00 };

#include "mem/cartridge.h"
#include "sound.h"
#include "tima.h"
#include "video.h"
#include "newstate.h"
#include "gambatte.h"

namespace gambatte {

class FilterInfo;

class Memory {
public:
	explicit Memory(unsigned short &sp, unsigned short &pc);
	~Memory();

	bool loaded() const { return cart_.loaded(); }
	unsigned curRomBank() const { return cart_.curRomBank(); }
	char const * romTitle() const { return cart_.romTitle(); }
	int debugGetLY() const { return lcd_.debugGetLY(); }
	void setStatePtrs(SaveState &state);
	void loadState(SaveState const &state);
	void loadSavedata(char const *data, unsigned long const cc) { cart_.loadSavedata(data, cc); }
	int saveSavedataLength() {return cart_.saveSavedataLength(); }
	void saveSavedata(char *dest, unsigned long const cc) { cart_.saveSavedata(dest, cc); }
	void updateInput();

	void setBios(char const *buffer, std::size_t size) {
		delete []bios_;
		bios_ = new unsigned char[size];
		memcpy(bios_, buffer, size);
		biosSize_ = size;
	}
	bool gbIsCgb() { return gbIsCgb_; }

	bool getMemoryArea(int which, unsigned char **data, int *length);

	unsigned long stop(unsigned long cycleCounter);
	bool isCgb() const { return lcd_.isCgb(); }
	bool ime() const { return intreq_.ime(); }
	bool halted() const { return intreq_.halted(); }
	unsigned long nextEventTime() const { return intreq_.minEventTime(); }
	void setLayers(unsigned mask) { lcd_.setLayers(mask); }
	bool isActive() const { return intreq_.eventTime(intevent_end) != disabled_time; }

	long cyclesSinceBlit(unsigned long cc) const {
		if (cc < intreq_.eventTime(intevent_blit))
			return -1;

		return (cc - intreq_.eventTime(intevent_blit)) >> isDoubleSpeed();
	}

	void halt(unsigned long cycleCounter) { halttime_ = cycleCounter; intreq_.halt(); }
	void ei(unsigned long cycleCounter) { if (!ime()) { intreq_.ei(cycleCounter); } }
	void di() { intreq_.di(); }

	unsigned readBios(unsigned p) {
		if(agbMode_ && p >= 0xF3 && p < 0x100) {
			return (agbOverride[p-0xF3] + bios_[p]) & 0xFF;
		}
		return bios_[p];
	}

	unsigned ff_read(unsigned p, unsigned long cc) {
		return p < 0x80 ? nontrivial_ff_read(p, cc) : ioamhram_[p + 0x100];
	}

	unsigned read(unsigned p, unsigned long cc) {
		if(biosMode_) {
			if (p < biosSize_ && !(p >= 0x100 && p < 0x200))
				return readBios(p);
		}
		return cart_.rmem(p >> 12) ? cart_.rmem(p >> 12)[p] : nontrivial_read(p, cc);
	}

	unsigned read_excb(unsigned p, unsigned long cc, bool first) {
		if (biosMode_) {
			if(p < biosSize_ && !(p >= 0x100 && p < 0x200))
				return readBios(p);
		}
		return cart_.rmem(p >> 12) ? cart_.rmem(p >> 12)[p] : nontrivial_read(p, cc);
	}

	unsigned peek(unsigned p) {
		if (biosMode_ && p < biosSize_ && !(p >= 0x100 && p < 0x200)) {
			return readBios(p);
		}
		return cart_.rmem(p >> 12) ? cart_.rmem(p >> 12)[p] : nontrivial_peek(p);
	}

	void write_nocb(unsigned p, unsigned data, unsigned long cc) {
		if (cart_.wmem(p >> 12)) {
			cart_.wmem(p >> 12)[p] = data;
		} else
			nontrivial_write(p, data, cc);
	}

	void write(unsigned p, unsigned data, unsigned long cc) {
		if (cart_.wmem(p >> 12)) {
			cart_.wmem(p >> 12)[p] = data;
		} else
			nontrivial_write(p, data, cc);
	}

	void ff_write(unsigned p, unsigned data, unsigned long cc) {
		if (p - 0x80u < 0x7Fu) {
			ioamhram_[p + 0x100] = data;
		} else
			nontrivial_ff_write(p, data, cc);
	}

	unsigned long event(unsigned long cycleCounter);
	unsigned long resetCounters(unsigned long cycleCounter);
	LoadRes loadROM(char const *romfiledata, unsigned romfilelength, bool forceDmg, bool multicartCompat);

	void setInputGetter(unsigned (*getInput)(void *), void *context) {
		getInput_ = getInput;
		getInputContext_ = context;
	}

	void setEndtime(unsigned long cc, unsigned long inc);
	void setBasetime(unsigned long cc) { basetime_ = cc; }

	void setSoundBuffer() { psg_.setBuffer(); }
	std::size_t fillSoundBuffer(unsigned long cc);

	void setVideoBuffer(uint_least32_t *videoBuf, std::ptrdiff_t pitch) {
		lcd_.setVideoBuffer(videoBuf, pitch);
	}

	void setDmgPaletteColor(int palNum, int colorNum, unsigned long rgb32) {
		lcd_.setDmgPaletteColor(palNum, colorNum, rgb32);
	}

	void setCgbPalette(unsigned *lut);
	void setTimeMode(bool useCycles, unsigned long const cc) {
		cart_.setTimeMode(useCycles, cc);
	}

	int linkStatus(int which);

	uint16_t getDivState(uint32_t cc) {
		const uint32_t div = ioamhram_[0x104];
		return (((div << 8) + cc - divLastUpdate_) >> 2) & 0x3FFF;
	}

	inline bool isInterrupt(uint32_t romAddress) { if (biosMode_) return false; else return cart_.isInterrupt(romAddress); }
	inline void setInterrupt(uint32_t romAddress) { cart_.setInterrupt(romAddress); }
	inline void clearInterrupt(uint32_t romAddress) { cart_.clearInterrupt(romAddress); }

private:
	Cartridge cart_;
	unsigned char ioamhram_[0x200];
	unsigned char *bios_;
	std::size_t biosSize_;
	unsigned (*getInput_)(void *);
	void *getInputContext_;
	unsigned long divLastUpdate_;
	unsigned long lastOamDmaUpdate_;
	InterruptRequester intreq_;
	Tima tima_;
	LCD lcd_;
	PSG psg_;
	unsigned short dmaSource_;
	unsigned short dmaDestination_;
	unsigned char oamDmaPos_;
	unsigned char serialCnt_;
	bool blanklcd_;
	bool biosMode_;
	bool cgbSwitching_;
	bool agbMode_;
	bool gbIsCgb_;
	unsigned short &sp_;
	unsigned short &pc_;
	unsigned long basetime_;
	unsigned long halttime_;
	bool stopped_;

	bool LINKCABLE_;
	bool linkClockTrigger_;

	void decEventCycles(IntEventId eventId, unsigned long dec);
	void oamDmaInitSetup();
	void updateOamDma(unsigned long cycleCounter);
	void startOamDma(unsigned long cycleCounter);
	void endOamDma(unsigned long cycleCounter);
	unsigned char const * oamDmaSrcPtr() const;
	unsigned nontrivial_ff_read(unsigned p, unsigned long cycleCounter);
	unsigned nontrivial_read(unsigned p, unsigned long cycleCounter);
	void nontrivial_ff_write(unsigned p, unsigned data, unsigned long cycleCounter);
	void nontrivial_write(unsigned p, unsigned data, unsigned long cycleCounter);
	unsigned nontrivial_peek(unsigned p);
	unsigned nontrivial_ff_peek(unsigned p);
	void updateSerial(unsigned long cc);
	void updateTimaIrq(unsigned long cc);
	void updateIrqs(unsigned long cc);
	bool isDoubleSpeed() const { return lcd_.isDoubleSpeed(); }

public:
	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
