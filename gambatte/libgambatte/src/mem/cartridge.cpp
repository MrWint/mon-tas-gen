/***************************************************************************
 *   Copyright (C) 2007-2010 by Sindre Aamås                               *
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
#include "cartridge.h"
#include "../savestate.h"
#include <algorithm>
#include <cstdio>
#include <cstring>
#include <fstream>

namespace gambatte {

namespace {

static uint32_t toMulti64Rombank(const uint32_t rombank) {
	return (rombank >> 1 & 0x30) | (rombank & 0xF);
}

class DefaultMbc : public Mbc {
public:
	virtual bool isAddressWithinAreaRombankCanBeMappedTo(uint32_t addr, uint32_t bank) const {
		return (addr< 0x4000) == (bank == 0);
	}

	virtual void SyncState(NewState *ns, bool isReader)
	{
	}
};

class Mbc0 : public DefaultMbc {
	MemPtrs &memptrs;
	bool enableRam;

public:
	explicit Mbc0(MemPtrs &memptrs)
	: memptrs(memptrs),
	  enableRam(false)
	{
	}

	virtual void romWrite(const uint32_t P, const uint32_t data) {
		if (P < 0x2000) {
			enableRam = (data & 0xF) == 0xA;
			memptrs.setRambank(enableRam ? MemPtrs::READ_EN | MemPtrs::WRITE_EN : 0, 0);
		}
	}

	virtual void loadState(const SaveState::Mem &ss) {
		enableRam = ss.enableRam;
		memptrs.setRambank(enableRam ? MemPtrs::READ_EN | MemPtrs::WRITE_EN : 0, 0);
	}

	virtual void SyncState(NewState *ns, bool isReader)
	{
		NSS(enableRam);
	}
};

static inline uint32_t rambanks(const MemPtrs &memptrs) {
	return static_cast<std::size_t>(memptrs.rambankdataend() - memptrs.rambankdata()) / 0x2000;
}

static inline uint32_t rombanks(const MemPtrs &memptrs) {
	return static_cast<std::size_t>(memptrs.romdataend()     - memptrs.romdata()    ) / 0x4000;
}

class Mbc1 : public DefaultMbc {
	MemPtrs &memptrs;
	uint8_t rombank;
	uint8_t rambank;
	bool enableRam;
	bool rambankMode;

	static uint32_t adjustedRombank(uint32_t bank) { return bank & 0x1F ? bank : bank | 1; }
	void setRambank() const { memptrs.setRambank(enableRam ? MemPtrs::READ_EN | MemPtrs::WRITE_EN : 0, rambank & (rambanks(memptrs) - 1)); }
	void setRombank() const { memptrs.setRombank(adjustedRombank(rombank & (rombanks(memptrs) - 1))); }

public:
	explicit Mbc1(MemPtrs &memptrs)
	: memptrs(memptrs),
	  rombank(1),
	  rambank(0),
	  enableRam(false),
	  rambankMode(false)
	{
	}

	virtual void romWrite(const uint32_t P, const uint32_t data) {
		switch (P >> 13 & 3) {
		case 0:
			enableRam = (data & 0xF) == 0xA;
			setRambank();
			break;
		case 1:
			rombank = rambankMode ? data & 0x1F : (rombank & 0x60) | (data & 0x1F);
			setRombank();
			break;
		case 2:
			if (rambankMode) {
				rambank = data & 3;
				setRambank();
			} else {
				rombank = (data << 5 & 0x60) | (rombank & 0x1F);
				setRombank();
			}

			break;
		case 3:
			// Pretty sure this should take effect immediately, but I have a policy not to change old behavior
			// unless I have something (eg. a verified test or a game) that justifies it.
			rambankMode = data & 1;
			break;
		}
	}

	virtual void loadState(const SaveState::Mem &ss) {
		rombank = ss.rombank;
		rambank = ss.rambank;
		enableRam = ss.enableRam;
		rambankMode = ss.rambankMode;
		setRambank();
		setRombank();
	}

	virtual void SyncState(NewState *ns, bool isReader)
	{
		NSS(rombank);
		NSS(rambank);
		NSS(enableRam);
		NSS(rambankMode);
	}
};

class Mbc1Multi64 : public Mbc {
	MemPtrs &memptrs;
	uint8_t rombank;
	bool enableRam;
	bool rombank0Mode;

	static uint32_t adjustedRombank(uint32_t bank) { return bank & 0x1F ? bank : bank | 1; }

	void setRombank() const {
		if (rombank0Mode) {
			const uint32_t rb = toMulti64Rombank(rombank);
			memptrs.setRombank0(rb & 0x30);
			memptrs.setRombank(adjustedRombank(rb));
		} else {
			memptrs.setRombank0(0);
			memptrs.setRombank(adjustedRombank(rombank & (rombanks(memptrs) - 1)));
		}
	}

public:
	explicit Mbc1Multi64(MemPtrs &memptrs)
	: memptrs(memptrs),
	  rombank(1),
	  enableRam(false),
	  rombank0Mode(false)
	{
	}

	virtual void romWrite(const uint32_t P, const uint32_t data) {
		switch (P >> 13 & 3) {
		case 0:
			enableRam = (data & 0xF) == 0xA;
			memptrs.setRambank(enableRam ? MemPtrs::READ_EN | MemPtrs::WRITE_EN : 0, 0);
			break;
		case 1:
			rombank = (rombank   & 0x60) | (data    & 0x1F);
			memptrs.setRombank(adjustedRombank(rombank0Mode ? toMulti64Rombank(rombank) : rombank & (rombanks(memptrs) - 1)));
			break;
		case 2:
			rombank = (data << 5 & 0x60) | (rombank & 0x1F);
			setRombank();
			break;
		case 3:
			rombank0Mode = data & 1;
			setRombank();
			break;
		}
	}

	virtual void loadState(const SaveState::Mem &ss) {
		rombank = ss.rombank;
		enableRam = ss.enableRam;
		rombank0Mode = ss.rambankMode;
		memptrs.setRambank(enableRam ? MemPtrs::READ_EN | MemPtrs::WRITE_EN : 0, 0);
		setRombank();
	}
	
	virtual bool isAddressWithinAreaRombankCanBeMappedTo(uint32_t addr, uint32_t bank) const {
		return (addr < 0x4000) == ((bank & 0xF) == 0);
	}

	virtual void SyncState(NewState *ns, bool isReader)
	{
		NSS(rombank);
		NSS(enableRam);
		NSS(rombank0Mode);
	}
};

class Mbc2 : public DefaultMbc {
	MemPtrs &memptrs;
	uint8_t rombank;
	bool enableRam;

public:
	explicit Mbc2(MemPtrs &memptrs)
	: memptrs(memptrs),
	  rombank(1),
	  enableRam(false)
	{
	}

	virtual void romWrite(const uint32_t P, const uint32_t data) {
		switch (P & 0x6100) {
		case 0x0000:
			enableRam = (data & 0xF) == 0xA;
			memptrs.setRambank(enableRam ? MemPtrs::READ_EN | MemPtrs::WRITE_EN : 0, 0);
			break;
		case 0x2100:
			rombank = data & 0xF;
			memptrs.setRombank(rombank & (rombanks(memptrs) - 1));
			break;
		}
	}

	virtual void loadState(const SaveState::Mem &ss) {
		rombank = ss.rombank;
		enableRam = ss.enableRam;
		memptrs.setRambank(enableRam ? MemPtrs::READ_EN | MemPtrs::WRITE_EN : 0, 0);
		memptrs.setRombank(rombank & (rombanks(memptrs) - 1));
	}

	virtual void SyncState(NewState *ns, bool isReader)
	{
		NSS(rombank);
		NSS(enableRam);
	}
};

class Mbc3 : public DefaultMbc {
	MemPtrs &memptrs;
	Rtc *const rtc;
	uint8_t rombank;
	uint8_t rambank;
	bool enableRam;

	static uint32_t adjustedRombank(uint32_t bank) { return bank & 0x7F ? bank : bank | 1; }
	void setRambank() const {
		uint32_t flags = enableRam ? MemPtrs::READ_EN | MemPtrs::WRITE_EN : 0;
		
		if (rtc) {
			rtc->set(enableRam, rambank);
			
			if (rtc->getActive())
				flags |= MemPtrs::RTC_EN;
		}

		memptrs.setRambank(flags, rambank & (rambanks(memptrs) - 1));
	}
	// we adjust the rombank before masking with size?  this seems correct, as how would the mbc
	// know that high rom address outputs were not connected
	void setRombank() const { memptrs.setRombank(adjustedRombank(rombank) & (rombanks(memptrs) - 1)); }

public:
	Mbc3(MemPtrs &memptrs, Rtc *const rtc)
	: memptrs(memptrs),
	  rtc(rtc),
	  rombank(1),
	  rambank(0),
	  enableRam(false)
	{
	}

	virtual void romWrite(const uint32_t P, const uint32_t data) {
		switch (P >> 13 & 3) {
		case 0:
			enableRam = (data & 0xF) == 0xA;
			setRambank();
			break;
		case 1:
			rombank = data & 0x7F;
			setRombank();
			break;
		case 2:
			rambank = data;
			setRambank();
			break;
		case 3:
			if (rtc)
				rtc->latch(data);

			break;
		}
	}

	virtual void loadState(const SaveState::Mem &ss) {
		rombank = ss.rombank;
		rambank = ss.rambank;
		enableRam = ss.enableRam;
		setRambank();
		setRombank();
	}

	virtual void SyncState(NewState *ns, bool isReader)
	{
		NSS(rombank);
		NSS(rambank);
		NSS(enableRam);
	}
};

class HuC1 : public DefaultMbc {
	MemPtrs &memptrs;
	uint8_t rombank;
	uint8_t rambank;
	bool enableRam;
	bool rambankMode;

	void setRambank() const {
		memptrs.setRambank(enableRam ? MemPtrs::READ_EN | MemPtrs::WRITE_EN : MemPtrs::READ_EN,
		                   rambankMode ? rambank & (rambanks(memptrs) - 1) : 0);
	}

	void setRombank() const { memptrs.setRombank((rambankMode ? rombank : rambank << 6 | rombank) & (rombanks(memptrs) - 1)); }

public:
	explicit HuC1(MemPtrs &memptrs)
	: memptrs(memptrs),
	  rombank(1),
	  rambank(0),
	  enableRam(false),
	  rambankMode(false)
	{
	}

	virtual void romWrite(const uint32_t P, const uint32_t data) {
		switch (P >> 13 & 3) {
		case 0:
			enableRam = (data & 0xF) == 0xA;
			setRambank();
			break;
		case 1:
			rombank = data & 0x3F;
			setRombank();
			break;
		case 2:
			rambank = data & 3;
			rambankMode ? setRambank() : setRombank();
			break;
		case 3:
			rambankMode = data & 1;
			setRambank();
			setRombank();
			break;
		}
	}

	virtual void loadState(const SaveState::Mem &ss) {
		rombank = ss.rombank;
		rambank = ss.rambank;
		enableRam = ss.enableRam;
		rambankMode = ss.rambankMode;
		setRambank();
		setRombank();
	}

	virtual void SyncState(NewState *ns, bool isReader)
	{
		NSS(rombank);
		NSS(rambank);
		NSS(enableRam);
		NSS(rambankMode);
	}
};

class Mbc5 : public DefaultMbc {
	MemPtrs &memptrs;
	uint16_t rombank;
	uint8_t rambank;
	bool enableRam;

	static uint32_t adjustedRombank(const uint32_t bank) { return bank; }
	void setRambank() const { memptrs.setRambank(enableRam ? MemPtrs::READ_EN | MemPtrs::WRITE_EN : 0, rambank & (rambanks(memptrs) - 1)); }
	void setRombank() const { memptrs.setRombank(adjustedRombank(rombank & (rombanks(memptrs) - 1))); }

public:
	explicit Mbc5(MemPtrs &memptrs)
	: memptrs(memptrs),
	  rombank(1),
	  rambank(0),
	  enableRam(false)
	{
	}

	virtual void romWrite(const uint32_t P, const uint32_t data) {
		switch (P >> 13 & 3) {
		case 0:
			enableRam = (data & 0xF) == 0xA;
			setRambank();
			break;
		case 1:
			rombank = P < 0x3000 ? (rombank   & 0x100) |  data
			                     : (data << 8 & 0x100) | (rombank & 0xFF);
			setRombank();
			break;
		case 2:
			rambank = data & 0xF;
			setRambank();
			break;
		case 3:
			break;
		}
	}

	virtual void loadState(const SaveState::Mem &ss) {
		rombank = ss.rombank;
		rambank = ss.rambank;
		enableRam = ss.enableRam;
		setRambank();
		setRombank();
	}

	virtual void SyncState(NewState *ns, bool isReader)
	{
		NSS(rombank);
		NSS(rambank);
		NSS(enableRam);
	}
};

static bool hasRtc(const uint32_t headerByte0x147) {
	switch (headerByte0x147) {
	case 0x0F:
	case 0x10: return true;
	default: return false;
	}
}

}

void Cartridge::setStatePtrs(SaveState &state) {
	state.mem.vram.set(memptrs.vramdata(), memptrs.vramdataend() - memptrs.vramdata());
	state.mem.sram.set(memptrs.rambankdata(), memptrs.rambankdataend() - memptrs.rambankdata());
	state.mem.wram.set(memptrs.wramdata(0), memptrs.wramdataend() - memptrs.wramdata(0));
}

void Cartridge::loadState(const SaveState &state) {
	rtc.loadState(state);
	mbc->loadState(state.mem);
}

static uint32_t pow2ceil(uint32_t n) {
	--n;
	n |= n >> 1;
	n |= n >> 2;
	n |= n >> 4;
	n |= n >> 8;
	++n;

	return n;
}

int32_t Cartridge::loadROM(const uint8_t *romfiledata, uint32_t romfilelength, const bool forceDmg, const bool multicartCompat) {
	uint32_t rambanks = 1;
	uint32_t rombanks = 2;
	bool cgb = false;
	enum Cartridgetype { PLAIN, MBC1, MBC2, MBC3, MBC5, HUC1 } type = PLAIN;

	{
		uint8_t header[0x150];
		if (romfilelength >= sizeof header)
			std::memcpy(header, romfiledata, sizeof header);
		else
			return -1;

		switch (header[0x0147]) {
		case 0x00: type = PLAIN; break;
		case 0x01: type = MBC1; break;
		case 0x02: type = MBC1; break;
		case 0x03: type = MBC1; break;
		case 0x05: type = MBC2; break;
		case 0x06: type = MBC2; break;
		case 0x08: type = PLAIN; break;
		case 0x09: type = PLAIN; break;
		case 0x0F: type = MBC3; break;
		case 0x10: type = MBC3; break;
		case 0x11: type = MBC3; break;
		case 0x12: type = MBC3; break;
		case 0x13: type = MBC3; break;
		case 0x19: type = MBC5; break;
		case 0x1A: type = MBC5; break;
		case 0x1B: type = MBC5; break;
		case 0x1C: type = MBC5; break;
		case 0x1D: type = MBC5; break;
		case 0x1E: type = MBC5; break;
		case 0xFF: type = HUC1; break;
		default: std::puts("Wrong data-format, corrupt or unsupported ROM."); return -1;
		}

		switch (header[0x0149]) {
		case 0x00: /*std::puts("No RAM");*/ rambanks = type == MBC2; break;
		case 0x01: /*std::puts("2kB RAM");*/ /*rambankrom=1; break;*/
		case 0x02: /*std::puts("8kB RAM");*/
			rambanks = 1;
			break;
		case 0x03: /*std::puts("32kB RAM");*/
			rambanks = 4;
			break;
		case 0x04: /*std::puts("128kB RAM");*/
			rambanks = 16;
			break;
		case 0x05: /*std::puts("undocumented kB RAM");*/
			rambanks = 16;
			break;
		default: /*std::puts("Wrong data-format, corrupt or unsupported ROM loaded.");*/
			rambanks = 16;
			break;
		}
		
		cgb = !forceDmg;
	}

	const std::size_t filesize = romfilelength; //rom->size();
	rombanks = std::max(pow2ceil(filesize / 0x4000), 2u);
	
	mbc.reset();
	memptrs.reset(rombanks, rambanks, cgb ? 8 : 2);
	rtc.set(false, 0);

	std::memcpy(memptrs.romdata(), romfiledata, (filesize / 0x4000) * 0x4000ul);
	std::memset(memptrs.romdata() + (filesize / 0x4000) * 0x4000ul, 0xFF, (rombanks - filesize / 0x4000) * 0x4000ul);
	
	switch (type) {
	case PLAIN: mbc.reset(new Mbc0(memptrs)); break;
	case MBC1:
		if (!rambanks && rombanks == 64 && multicartCompat) {
			std::puts("Multi-ROM \"MBC1\" presumed");
			mbc.reset(new Mbc1Multi64(memptrs));
		} else
			mbc.reset(new Mbc1(memptrs));

		break;
	case MBC2: mbc.reset(new Mbc2(memptrs)); break;
	case MBC3: mbc.reset(new Mbc3(memptrs, hasRtc(memptrs.romdata()[0x147]) ? &rtc : 0)); break;
	case MBC5: mbc.reset(new Mbc5(memptrs)); break;
	case HUC1: mbc.reset(new HuC1(memptrs)); break;
	}

	return 0;
}

static bool hasBattery(const uint8_t headerByte0x147) {
	switch (headerByte0x147) {
	case 0x03:
	case 0x06:
	case 0x09:
	case 0x0F:
	case 0x10:
	case 0x13:
	case 0x1B:
	case 0x1E:
	case 0xFF: return true;
	default: return false;
	}
}

void Cartridge::loadSavedata(const uint8_t *data) {
	if (hasBattery(memptrs.romdata()[0x147])) {
		int32_t length = memptrs.rambankdataend() - memptrs.rambankdata();
		std::memcpy(memptrs.rambankdata(), data, length);
		data += length;
	}

	if (hasRtc(memptrs.romdata()[0x147])) {
		uint32_t basetime;
		std::memcpy(&basetime, data, 4);
		rtc.setBaseTime(basetime);
	}
}

size_t Cartridge::saveSavedataLength() {
	size_t ret = 0;
	if (hasBattery(memptrs.romdata()[0x147])) {
		ret = memptrs.rambankdataend() - memptrs.rambankdata();
	}
	if (hasRtc(memptrs.romdata()[0x147])) {
		ret += 4;
	}
	return ret;
}

void Cartridge::saveSavedata(uint8_t *dest) {
	if (hasBattery(memptrs.romdata()[0x147])) {
		int32_t length = memptrs.rambankdataend() - memptrs.rambankdata();
		std::memcpy(dest, memptrs.rambankdata(), length);
		dest += length;
	}

	if (hasRtc(memptrs.romdata()[0x147])) {
		const uint32_t basetime = rtc.getBaseTime();
		std::memcpy(dest, &basetime, 4);
	}
}

bool Cartridge::getMemoryArea(int32_t which, uint8_t **data, int32_t *length) const {
	if (!data || !length)
		return false;

	switch (which)
	{
	case 0:
		*data = memptrs.vramdata();
		*length = memptrs.vramdataend() - memptrs.vramdata();
		return true;
	case 1:
		*data = memptrs.romdata();
		*length = memptrs.romdataend() - memptrs.romdata();
		return true;
	case 2:
		*data = memptrs.wramdata(0);
		*length = memptrs.wramdataend() - memptrs.wramdata(0);
		return true;
	case 3:
		*data = memptrs.rambankdata();
		*length = memptrs.rambankdataend() - memptrs.rambankdata();
		return true;

	default:
		return false;
	}
	return false;
}

SYNCFUNC(Cartridge)
{
	SSS(memptrs);
	SSS(rtc);
	TSS(mbc);
}

}
