/***************************************************************************
 *   Copyright (C) 2010 by Sindre Aamås                                    *
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
#ifndef PPU_H
#define PPU_H

#include "video/ly_counter.h"
#include "video/sprite_mapper.h"

#include "newstate.h"

namespace gambatte {

enum { LAYER_MASK_BG = 1, LAYER_MASK_OBJ = 2, LAYER_MASK_WINDOW = 4 };

class PPUFrameBuf {
	uint32_t *buf_;
	uint32_t *fbline_;
	int32_t pitch_;

public:
	PPUFrameBuf() : buf_(0), fbline_(0), pitch_(0) {}
	uint32_t * fb() const { return buf_; }
	uint32_t * fbline() const { return fbline_; }
	int32_t pitch() const { return pitch_; }
	void setBuf(uint32_t *const buf, const int32_t pitch) { buf_ = buf; pitch_ = pitch; fbline_ = 0; }
	void setFbline(const uint32_t ly) { fbline_ = buf_ ? buf_ + static_cast<int32_t>(ly) * static_cast<int32_t>(pitch_) : 0; }
};

struct PPUState {
	void (*f)(struct PPUPriv &v);
	uint32_t (*predictCyclesUntilXpos_f)(const struct PPUPriv &v, int32_t targetxpos, uint32_t cycles);
	uint8_t id;
};

// The PPU loop accesses a lot of state at once, so it's difficult to split this up much beyond grouping stuff into smaller structs.
struct PPUPriv {
	uint32_t bgPalette[8 * 4];
	uint32_t spPalette[8 * 4];
	struct Sprite { uint8_t spx, oampos, line, attrib; } spriteList[11];
	uint16_t spwordList[11];
	uint8_t nextSprite;
	uint8_t currentSprite;
	uint8_t layersMask;

	const uint8_t *vram;
	const PPUState *nextCallPtr;

	uint32_t now;
	uint32_t lastM0Time;
	int32_t cycles;

	uint32_t tileword;
	uint32_t ntileword;

	SpriteMapper spriteMapper;
	LyCounter lyCounter;
	PPUFrameBuf framebuf;

	uint8_t lcdc;
	uint8_t scy;
	uint8_t scx;
	uint8_t wy;
	uint8_t wy2;
	uint8_t wx;
	uint8_t winDrawState;
	uint8_t wscx;
	uint8_t winYPos;
	uint8_t reg0;
	uint8_t reg1;
	uint8_t attrib;
	uint8_t nattrib;
	uint8_t xpos;
	uint8_t endx;

	bool cgb;
	bool weMaster;
	
	PPUPriv(NextM0Time &nextM0Time, const uint8_t *oamram, const uint8_t *vram);
};

class PPU {
	PPUPriv p_;
public:
	PPU(NextM0Time &nextM0Time, const uint8_t *oamram, const uint8_t *vram)
	: p_(nextM0Time, oamram, vram)
	{
	}
	
	uint32_t * bgPalette() { return p_.bgPalette; }
	bool cgb() const { return p_.cgb; }
	void doLyCountEvent() { p_.lyCounter.doEvent(); }
	uint32_t doSpriteMapEvent(uint32_t time) { return p_.spriteMapper.doEvent(time); }
	const PPUFrameBuf & frameBuf() const { return p_.framebuf; }
	bool inactivePeriodAfterDisplayEnable(uint32_t cc) const { return p_.spriteMapper.inactivePeriodAfterDisplayEnable(cc); }
	uint32_t lastM0Time() const { return p_.lastM0Time; }
	uint32_t lcdc() const { return p_.lcdc; }
	void loadState(const SaveState &state, const uint8_t *oamram);
	const LyCounter & lyCounter() const { return p_.lyCounter; }
	uint32_t now() const { return p_.now; }
	void oamChange(uint32_t cc) { p_.spriteMapper.oamChange(cc); }
	void oamChange(const uint8_t *oamram, uint32_t cc) { p_.spriteMapper.oamChange(oamram, cc); }
	uint32_t predictedNextXposTime(uint32_t xpos) const;
	void reset(const uint8_t *oamram, const uint8_t *vram, bool cgb);
	void resetCc(uint32_t oldCc, uint32_t newCc);
	void setFrameBuf(uint32_t *buf, uint32_t pitch) { p_.framebuf.setBuf(buf, pitch); }
	void setLcdc(uint32_t lcdc, uint32_t cc);
	void setScx(const uint32_t scx) { p_.scx = scx; }
	void setScy(const uint32_t scy) { p_.scy = scy; }
	void setStatePtrs(SaveState &ss) { p_.spriteMapper.setStatePtrs(ss); }
	void setWx(const uint32_t wx) { p_.wx = wx; }
	void setWy(const uint32_t wy) { p_.wy = wy; }
	void updateWy2() { p_.wy2 = p_.wy; }
	void speedChange(uint32_t cycleCounter);
	uint32_t * spPalette() { return p_.spPalette; }
	void update(uint32_t cc);
	void setLayers(uint8_t mask) { p_.layersMask = mask; }
	void setCgb(bool cgb) { p_.cgb = cgb; }

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
