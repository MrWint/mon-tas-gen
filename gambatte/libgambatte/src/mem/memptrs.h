//
//   Copyright (C) 2007-2010 by sinamas <sinamas at users.sourceforge.net>
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

#ifndef MEMPTRS_H
#define MEMPTRS_H

// #define TRACE_LOGGING_ENABLED

#include <cstdint>
#include "newstate.h"

#ifdef TRACE_LOGGING_ENABLED
#include <fstream>
#endif


namespace gambatte {

enum OamDmaSrc { oam_dma_src_rom,
                 oam_dma_src_sram,
                 oam_dma_src_vram,
                 oam_dma_src_wram,
                 oam_dma_src_invalid,
                 oam_dma_src_off, };

class MemPtrs {
public:
	enum RamFlag { read_en = 1, write_en = 2, rtc_en = 4 };

	MemPtrs();
	~MemPtrs();
	void reset(unsigned rombanks, unsigned rambanks, unsigned wrambanks);

	unsigned char const * rmem(unsigned area) const { return rmem_[area]; }
	unsigned char * wmem(unsigned area) const { return wmem_[area]; }
	unsigned char * vramdata() const { return rambankdata_ - 0x4000; }
	unsigned char * vramdataend() const { return rambankdata_; }
	unsigned char * romdata() const { return memchunk_ + 0x4000; }
	unsigned char * romdata(unsigned area) const { return romdata_[area]; }
	unsigned char * romdataend() const { return rambankdata_ - 0x4000; }
	unsigned char * wramdata(unsigned area) const { return wramdata_[area]; }
	unsigned char * wramdataend() const { return wramdataend_; }
	unsigned char * rambankdata() const { return rambankdata_; }
	unsigned char * rambankdataend() const { return wramdata_[0]; }
	unsigned char const * rdisabledRam() const { return rdisabledRamw(); }
	unsigned char const * rsrambankptr() const { return rsrambankptr_; }
	unsigned char * wsrambankptr() const { return wsrambankptr_; }
	unsigned char * vrambankptr() const { return vrambankptr_; }
	OamDmaSrc oamDmaSrc() const { return oamDmaSrc_; }
	unsigned curRomBank() const { return curRomBank_; }

	void setRombank0(unsigned bank);
	void setRombank(unsigned bank);
	void setRambank(unsigned ramFlags, unsigned rambank);
	void setVrambank(unsigned bank) { vrambankptr_ = vramdata() + bank * 0x2000ul - 0x8000; }
	void setWrambank(unsigned bank);
	void setOamDmaSrc(OamDmaSrc oamDmaSrc);

private:
	unsigned char const *rmem_[0x10];
	unsigned char       *wmem_[0x10];
	unsigned char *romdata_[2];
	unsigned char *wramdata_[2];
	unsigned char *vrambankptr_;
	unsigned char *rsrambankptr_;
	unsigned char *wsrambankptr_;
	unsigned char *memchunk_;
	unsigned char *rambankdata_;
	unsigned char *wramdataend_;
	bool *interruptmemchunk_; // Somewhat wasteful but allows faster lookups
#ifdef TRACE_LOGGING_ENABLED
	bool *codetracememchunk_;
	std::ofstream logout;
#endif
	OamDmaSrc oamDmaSrc_;

	unsigned curRomBank_;

	int memchunk_len;
	int memchunk_saveoffs;
	int memchunk_savelen;

	MemPtrs(MemPtrs const &);
	MemPtrs & operator=(MemPtrs const &);
	void disconnectOamDmaAreas();
	unsigned char * rdisabledRamw() const { return wramdataend_         ; }
	unsigned char * wdisabledRam()  const { return wramdataend_ + 0x2000; }

public:
	inline bool isInterrupt(std::uint32_t romAddress) {
#ifdef TRACE_LOGGING_ENABLED
		if (!codetracememchunk_[romAddress]) {
			if (!logout.is_open()) logout.open("codeTrace.bin", std::ios::binary);
			logout.write((char*)&romAddress, 4);
			logout.flush();
			codetracememchunk_[romAddress] = true;
		}
#endif
		return interruptmemchunk_[romAddress];
	}
	inline void setInterrupt(std::uint32_t romAddress) { interruptmemchunk_[romAddress] = true; }
	inline void clearInterrupt(std::uint32_t romAddress) { interruptmemchunk_[romAddress] = false; }

	template<bool isReader>void SyncState(NewState *ns);
};

inline bool isCgb(MemPtrs const &memptrs) {
	return memptrs.wramdataend() - memptrs.wramdata(0) == 0x8000;
}

}

#endif
