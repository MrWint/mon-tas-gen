/***************************************************************************
 *   Copyright (C) 2008 by Sindre Aamås                                    *
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
#ifndef SAVESTATE_H
#define SAVESTATE_H

#include <cstdint>

namespace gambatte {

class SaverList;

struct SaveState {
	template<typename T>
	class Ptr {
		T *ptr;
		size_t sz;
		
	public:
		Ptr() : ptr(0), sz(0) {}
		const T* get() const { return ptr; }
		size_t getSz() const { return sz; }
		void set(T *ptr, const size_t sz) { this->ptr = ptr; this->sz = sz; }
		
		friend class SaverList;
		friend void setInitState(SaveState &, bool, bool, uint32_t, unsigned);
	};

	struct CPU {
		uint32_t cycleCounter;
		uint16_t PC;
		uint16_t SP;
		uint8_t A;
		uint8_t B;
		uint8_t C;
		uint8_t D;
		uint8_t E;
		uint8_t F;
		uint8_t H;
		uint8_t L;
		bool skip;
	} cpu;
	
	struct Mem {
		Ptr<uint8_t> vram;
		Ptr<uint8_t> sram;
		Ptr<uint8_t> wram;
		Ptr<uint8_t> ioamhram;
		uint32_t divLastUpdate;
		uint32_t timaLastUpdate;
		uint32_t tmatime;
		uint32_t nextSerialtime;
		uint32_t lastOamDmaUpdate;
		uint32_t minIntTime;
		uint32_t unhaltTime;
		uint16_t rombank;
		uint16_t dmaSource;
		uint16_t dmaDestination;
		uint8_t rambank;
		uint8_t oamDmaPos;
		bool IME;
		bool halted;
		bool enableRam;
		bool rambankMode;
		bool hdmaTransfer;
		bool biosMode;
		bool cgbSwitching;
		bool agbMode;
		bool gbIsCgb;
	} mem;
	
	struct PPU {
		Ptr<uint8_t> bgpData;
		Ptr<uint8_t> objpData;
		//SpriteMapper::OamReader
		Ptr<uint8_t> oamReaderBuf;
		Ptr<bool> oamReaderSzbuf;
		
		uint32_t videoCycles;
		uint32_t enableDisplayM0Time;
		uint16_t lastM0Time;
		uint16_t nextM0Irq;
		uint16_t tileword;
		uint16_t ntileword;
		uint8_t spAttribList[10];
		uint8_t spByte0List[10];
		uint8_t spByte1List[10];
		uint8_t winYPos;
		uint8_t xpos;
		uint8_t endx;
		uint8_t reg0;
		uint8_t reg1;
		uint8_t attrib;
		uint8_t nattrib;
		uint8_t state;
		uint8_t nextSprite;
		uint8_t currentSprite;
		uint8_t lyc;
		uint8_t m0lyc;
		uint8_t oldWy;
		uint8_t winDrawState;
		uint8_t wscx;
		bool weMaster;
		bool pendingLcdstatIrq;
		bool isCgb;
	} ppu;
	
	struct SPU {
		struct Duty {
			uint8_t nr3;
		};
		
		struct LCounter {
			uint32_t counter;
			uint16_t lengthCounter;
		};
		
		struct {
			struct {
				uint32_t counter;
				uint16_t shadow;
				uint8_t nr0;
				bool negging;
			} sweep;
			Duty duty;
			LCounter lcounter;
			uint8_t nr4;
			bool master;
		} ch1;
		
		struct {
			LCounter lcounter;
			uint8_t nr4;
			bool master;
		} ch2;
		
		struct {
			Ptr<uint8_t> waveRam;
			LCounter lcounter;
			uint32_t waveCounter;
			uint32_t lastReadTime;
			uint8_t nr3;
			uint8_t nr4;
			uint8_t wavePos;
			bool master;
		} ch3;
		
		struct {
			LCounter lcounter;
			uint8_t nr4;
			bool master;
		} ch4;
		
		uint32_t cycleCounter;
	} spu;
	
	struct RTC {
		uint32_t baseTime;
		uint32_t haltTime;
		uint8_t dataDh;
		uint8_t dataDl;
		uint8_t dataH;
		uint8_t dataM;
		uint8_t dataS;
		bool lastLatchData;
	} rtc;
};

}

#endif
