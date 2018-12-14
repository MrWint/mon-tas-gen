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
#ifndef CARTRIDGE_H
#define CARTRIDGE_H

#include "memptrs.h"
#include "rtc.h"
#include "savestate.h"
#include <memory>
#include <string>
#include <vector>
#include "newstate.h"

namespace gambatte {

class Mbc {
public:
	virtual ~Mbc() {}
	virtual void romWrite(uint32_t P, uint32_t data) = 0;
	virtual void loadState(const SaveState::Mem &ss) = 0;
	virtual bool isAddressWithinAreaRombankCanBeMappedTo(uint32_t address, uint32_t rombank) const = 0;
	//virtual void mapAddress(AddressMapping* mapping, uint32_t address) const = 0; //DOOM

	template<bool isReader>void SyncState(NewState *ns)
	{
		// can't have virtual templates, so..
		SyncState(ns, isReader);
	}
	virtual void SyncState(NewState *ns, bool isReader) = 0;
};

class Cartridge {
	MemPtrs memptrs;
	Rtc rtc;
	std::unique_ptr<Mbc> mbc; // auto_ptr is deprecated.
	
public:
	void setStatePtrs(SaveState &);
	void loadState(const SaveState &);
	
	bool loaded() const { return mbc.get(); }
	
	const uint8_t * rmem(uint32_t area) const { return memptrs.rmem(area); }
	uint8_t * wmem(uint32_t area) const { return memptrs.wmem(area); }
	uint8_t * vramdata() const { return memptrs.vramdata(); }
	uint8_t * romdata(uint32_t area) const { return memptrs.romdata(area); }
	uint8_t * wramdata(uint32_t area) const { return memptrs.wramdata(area); }
	const uint8_t * rdisabledRam() const { return memptrs.rdisabledRam(); }
	const uint8_t * rsrambankptr() const { return memptrs.rsrambankptr(); }
	uint8_t * wsrambankptr() const { return memptrs.wsrambankptr(); }
	uint8_t * vrambankptr() const { return memptrs.vrambankptr(); }
	OamDmaSrc oamDmaSrc() const { return memptrs.oamDmaSrc(); }
	uint32_t curRomBank() const { return memptrs.curRomBank(); }

	void setVrambank(uint32_t bank) { memptrs.setVrambank(bank); }
	void setWrambank(uint32_t bank) { memptrs.setWrambank(bank); }
	void setOamDmaSrc(OamDmaSrc oamDmaSrc) { memptrs.setOamDmaSrc(oamDmaSrc); }
	
	void mbcWrite(uint32_t addr, uint32_t data) { mbc->romWrite(addr, data); }

	bool isCgb() const { return gambatte::isCgb(memptrs); }
	
	void rtcWrite(uint8_t data) { rtc.write(data); }
	uint8_t rtcRead() const { return *rtc.getActive(); }
	
	void loadSavedata(const uint8_t *data);
	size_t saveSavedataLength();
	void saveSavedata(uint8_t *dest);

	bool getMemoryArea(int32_t which, uint8_t **data, int32_t *length) const;

	int32_t loadROM(const uint8_t *romfiledata, uint32_t romfilelength, bool forceDmg, bool multicartCompat);

	void setRTCCallback(uint32_t (*callback)(void*), void* context) {
		rtc.setRTCCallback(callback, context);
	}

	inline bool isInterrupt(uint32_t romAddress) { return memptrs.isInterrupt(romAddress); }
	inline void setInterrupt(uint32_t romAddress) { memptrs.setInterrupt(romAddress); }
	inline void clearInterrupt(uint32_t romAddress) { memptrs.clearInterrupt(romAddress); }

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
