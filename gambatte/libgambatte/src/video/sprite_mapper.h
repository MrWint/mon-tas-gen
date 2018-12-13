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
#ifndef SPRITE_MAPPER_H
#define SPRITE_MAPPER_H

#include "ly_counter.h"
#include "../savestate.h"
#include "newstate.h"

namespace gambatte {
class NextM0Time;

class SpriteMapper {
	class OamReader {
		uint8_t buf[80];
		bool szbuf[40];

	public:
		const LyCounter &lyCounter;

	private:
		const uint8_t *oamram;
		uint32_t lu;
		uint8_t lastChange;
		bool largeSpritesSrc;
		bool cgb_;

	public:
		OamReader(const LyCounter &lyCounter, const uint8_t *oamram);
		void reset(const uint8_t *oamram, bool cgb);
		void change(uint32_t cc);
		void change(const uint8_t *_oamram, uint32_t cc) { change(cc); oamram = _oamram; }
		bool changed() const { return lastChange != 0xFF; }
		bool largeSprites(uint32_t spNr) const { return szbuf[spNr]; }
		const uint8_t *oam() const { return oamram; }
		void resetCycleCounter(const uint32_t oldCc, const uint32_t newCc) { lu -= oldCc - newCc; }
		void setLargeSpritesSrc(const bool src) { largeSpritesSrc = src; }
		void update(uint32_t cc);
		const uint8_t *spritePosBuf() const { return buf; }
		void setStatePtrs(SaveState &state);
		void enableDisplay(uint32_t cc);
		void loadState(const SaveState &ss, const uint8_t *oamram);
		bool inactivePeriodAfterDisplayEnable(const uint32_t cc) const { return cc < lu; }

		template<bool isReader>void SyncState(NewState *ns);
	};

	enum { NEED_SORTING_MASK = 0x80 };

public:
	class SpxLess {
		const uint8_t *const posbuf_plus1;

	public:
		explicit SpxLess(const uint8_t *const posbuf) : posbuf_plus1(posbuf + 1) {}

		bool operator()(const uint8_t l, const uint8_t r) const {
			return posbuf_plus1[l] < posbuf_plus1[r];
		}
	};

private:
	mutable uint8_t spritemap[144*10];
	mutable uint8_t num[144];
	
	NextM0Time &nextM0Time_;
	OamReader oamReader;

	void clearMap();
	void mapSprites();
	void sortLine(uint32_t ly) const;

public:
	SpriteMapper(NextM0Time &nextM0Time,
	             const LyCounter &lyCounter,
	             const uint8_t *oamram_in);
	void reset(const uint8_t *oamram, bool cgb);
	uint32_t doEvent(uint32_t time);
	bool largeSprites(uint32_t spNr) const { return oamReader.largeSprites(spNr); }
	uint8_t numSprites(const uint32_t ly) const { return num[ly] & ~NEED_SORTING_MASK; }
	void oamChange(uint32_t cc) { oamReader.change(cc); }
	void oamChange(const uint8_t *oamram, uint32_t cc) { oamReader.change(oamram, cc); }
	const uint8_t *oamram() const { return oamReader.oam(); }
	const uint8_t *posbuf() const { return oamReader.spritePosBuf(); }
	void  preSpeedChange(const uint32_t cc) { oamReader.update(cc); }
	void postSpeedChange(const uint32_t cc) { oamReader.change(cc); }

	void resetCycleCounter(const uint32_t oldCc, const uint32_t newCc) {
		oamReader.update(oldCc);
		oamReader.resetCycleCounter(oldCc, newCc);
	}

	static uint32_t schedule(const LyCounter &lyCounter, const uint32_t cycleCounter) {
		return lyCounter.nextLineCycle(80, cycleCounter);
	}

	void setLargeSpritesSource(bool src) { oamReader.setLargeSpritesSrc(src); }

	const uint8_t* sprites(const uint32_t ly) const {
		if (num[ly] & NEED_SORTING_MASK)
			sortLine(ly);

		return spritemap + ly * 10;
	}

	void setStatePtrs(SaveState &state) { oamReader.setStatePtrs(state); }
	void enableDisplay(uint32_t cc) { oamReader.enableDisplay(cc); }
	void loadState(const SaveState &state, const uint8_t *const oamram) { oamReader.loadState(state, oamram); mapSprites(); }
	bool inactivePeriodAfterDisplayEnable(uint32_t cc) const { return oamReader.inactivePeriodAfterDisplayEnable(cc); }

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
