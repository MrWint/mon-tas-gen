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
#ifndef MEMPTRS_H
#define MEMPTRS_H

#include "newstate.h"

namespace gambatte {

enum OamDmaSrc { OAM_DMA_SRC_ROM, OAM_DMA_SRC_SRAM, OAM_DMA_SRC_VRAM,
                 OAM_DMA_SRC_WRAM, OAM_DMA_SRC_INVALID, OAM_DMA_SRC_OFF };

class MemPtrs {
	const uint8_t *rmem_[0x10];
	      uint8_t *wmem_[0x10];
	
	uint8_t *romdata_[2];
	uint8_t *wramdata_[2];
	uint8_t *vrambankptr_;
	uint8_t *rsrambankptr_;
	uint8_t *wsrambankptr_;
	uint8_t *memchunk_;
	uint8_t *rambankdata_;
	uint8_t *wramdataend_;
	bool *interruptmemchunk_; // Somewhat wasteful but allows faster lookups
	
	OamDmaSrc oamDmaSrc_;

	uint32_t curRomBank_;

	int32_t memchunk_len;
	int32_t memchunk_saveoffs;
	int32_t memchunk_savelen;

	MemPtrs(const MemPtrs &);
	MemPtrs & operator=(const MemPtrs &);
	void disconnectOamDmaAreas();
	uint8_t * rdisabledRamw() const { return wramdataend_         ; }
	uint8_t * wdisabledRam()  const { return wramdataend_ + 0x2000; }
public:
	enum RamFlag { READ_EN = 1, WRITE_EN = 2, RTC_EN = 4 };
	
	MemPtrs();
	~MemPtrs();
	void reset(uint32_t rombanks, uint32_t rambanks, uint32_t wrambanks);
	
	const uint8_t * rmem(uint32_t area) const { return rmem_[area]; }
	uint8_t * wmem(uint32_t area) const { return wmem_[area]; }
	uint8_t * vramdata() const { return rambankdata_ - 0x4000; }
	uint8_t * vramdataend() const { return rambankdata_; }
	uint8_t * romdata() const { return memchunk_ + 0x4000; }
	uint8_t * romdata(uint32_t area) const { return romdata_[area]; }
	uint8_t * romdataend() const { return rambankdata_ - 0x4000; }
	uint8_t * wramdata(uint32_t area) const { return wramdata_[area]; }
	uint8_t * wramdataend() const { return wramdataend_; }
	uint8_t * rambankdata() const { return rambankdata_; }
	uint8_t * rambankdataend() const { return wramdata_[0]; }
	const uint8_t * rdisabledRam() const { return rdisabledRamw(); }
	const uint8_t * rsrambankptr() const { return rsrambankptr_; }
	uint8_t * wsrambankptr() const { return wsrambankptr_; }
	uint8_t * vrambankptr() const { return vrambankptr_; }
	OamDmaSrc oamDmaSrc() const { return oamDmaSrc_; }
	uint32_t curRomBank() const { return curRomBank_; }

	void setRombank0(uint32_t bank);
	void setRombank(uint32_t bank);
	void setRambank(uint32_t ramFlags, uint32_t rambank);
	void setVrambank(uint32_t bank) { vrambankptr_ = vramdata() + bank * 0x2000ul - 0x8000; }
	void setWrambank(uint32_t bank);
	void setOamDmaSrc(OamDmaSrc oamDmaSrc);

	inline bool isInterrupt(uint32_t romAddress) { return interruptmemchunk_[romAddress]; }
	inline void setInterrupt(uint32_t romAddress) { interruptmemchunk_[romAddress] = true; }
	inline void clearInterrupt(uint32_t romAddress) { interruptmemchunk_[romAddress] = false; }

	template<bool isReader>void SyncState(NewState *ns);
};

inline bool isCgb(const MemPtrs &memptrs) {
	return memptrs.wramdataend() - memptrs.wramdata(0) == 0x8000;
}

}

#endif
