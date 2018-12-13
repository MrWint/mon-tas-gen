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
#ifndef VIDEO_H
#define VIDEO_H

#include "video/ppu.h"
#include "video/lyc_irq.h"
#include "video/next_m0_time.h"
#include "interruptrequester.h"
#include "minkeeper.h"
#include <memory>
#include "newstate.h"

namespace gambatte {

class VideoInterruptRequester {
	InterruptRequester * intreq;
	
public:
	explicit VideoInterruptRequester(InterruptRequester * intreq) : intreq(intreq) {}
	void flagHdmaReq() const { gambatte::flagHdmaReq(intreq); }
	void flagIrq(const uint32_t bit) const { intreq->flagIrq(bit); }
	void setNextEventTime(const uint32_t time) const { intreq->setEventTime<VIDEO>(time); }
};

class M0Irq {
	uint8_t statReg_;
	uint8_t lycReg_;
	
public:
	M0Irq() : statReg_(0), lycReg_(0) {}
	
	void lcdReset(const uint8_t statReg, const uint8_t lycReg) {
		statReg_ = statReg;
		 lycReg_ =  lycReg;
	}
	
	void statRegChange(const uint8_t statReg,
			const uint32_t nextM0IrqTime, const uint32_t cc, const bool cgb) {
		if (nextM0IrqTime - cc > cgb * 2U)
			statReg_ = statReg;
	}
	
	void lycRegChange(const uint8_t lycReg,
			const uint32_t nextM0IrqTime, const uint32_t cc, const bool ds, const bool cgb) {
		if (nextM0IrqTime - cc > cgb * 5 + 1U - ds)
			lycReg_ = lycReg;
	}
	
	void doEvent(uint8_t *const ifreg, const uint32_t ly, const uint8_t statReg, const uint8_t lycReg) {
		if (((statReg_ | statReg) & 0x08) && (!(statReg_ & 0x40) || ly != lycReg_))
			*ifreg |= 2;
		
		statReg_ = statReg;
		 lycReg_ =  lycReg;
	}
	
	void loadState(const SaveState &state) {
		 lycReg_ = state.ppu.m0lyc;
		statReg_ = state.mem.ioamhram.get()[0x141];
	}
	
	uint32_t statReg() const { return statReg_; }

	template<bool isReader>
	void SyncState(NewState *ns)
	{
		NSS(statReg_);
		NSS(lycReg_);
	}
};

class LCD {
	enum Event { MEM_EVENT, LY_COUNT }; enum { NUM_EVENTS = LY_COUNT + 1 };
	enum MemEvent { ONESHOT_LCDSTATIRQ, ONESHOT_UPDATEWY2, MODE1_IRQ, LYC_IRQ, SPRITE_MAP,
	                HDMA_REQ, MODE2_IRQ, MODE0_IRQ }; enum { NUM_MEM_EVENTS = MODE0_IRQ + 1 };
	
	class EventTimes {
		MinKeeper<NUM_EVENTS> eventMin_;
		MinKeeper<NUM_MEM_EVENTS> memEventMin_;
		VideoInterruptRequester memEventRequester_;
		
		void setMemEvent() {
			const uint32_t nmet = nextMemEventTime();
			eventMin_.setValue<MEM_EVENT>(nmet);
			memEventRequester_.setNextEventTime(nmet);
		}
		
	public:
		explicit EventTimes(const VideoInterruptRequester memEventRequester) : memEventRequester_(memEventRequester) {}
		
		Event nextEvent() const { return static_cast<Event>(eventMin_.min()); }
		uint32_t nextEventTime() const { return eventMin_.minValue(); }
		uint32_t operator()(const Event e) const { return eventMin_.value(e); }
		template<Event e> void set(const uint32_t time) { eventMin_.setValue<e>(time); }
		void set(const Event e, const uint32_t time) { eventMin_.setValue(e, time); }
		
		MemEvent nextMemEvent() const { return static_cast<MemEvent>(memEventMin_.min()); }
		uint32_t nextMemEventTime() const { return memEventMin_.minValue(); }
		uint32_t operator()(const MemEvent e) const { return memEventMin_.value(e); }
		template<MemEvent e> void setm(const uint32_t time) { memEventMin_.setValue<e>(time); setMemEvent(); }
		void set(const MemEvent e, const uint32_t time) { memEventMin_.setValue(e, time); setMemEvent(); }
		
		void flagIrq(const uint32_t bit) { memEventRequester_.flagIrq(bit); }
		void flagHdmaReq() { memEventRequester_.flagHdmaReq(); }

		template<bool isReader>
		void SyncState(NewState *ns)
		{
			SSS(eventMin_);
			SSS(memEventMin_);
			//SSS(memEventRequester_); // not needed
		}
	};
	
	PPU ppu;
	uint32_t dmgColorsRgb32[3 * 4];
	uint8_t  bgpData[8 * 8];
	uint8_t objpData[8 * 8];

	EventTimes eventTimes_;
	M0Irq m0Irq_;
	LycIrq lycIrq;
	NextM0Time nextM0Time_;

	uint8_t statReg;
	uint8_t m2IrqStatReg_;
	uint8_t m1IrqStatReg_;

	static void setDmgPalette(uint32_t *palette, const uint32_t *dmgColors, uint32_t data);
	void setDmgPaletteColor(uint32_t index, uint32_t rgb32);

	uint32_t gbcToRgb32(const uint32_t bgr15);
	void doCgbColorChange(uint8_t *const pdata, uint32_t *const palette, uint32_t index, const uint32_t data);

	void refreshPalettes();
	void setDBuffer();

	void doMode2IrqEvent();
	void event();

	uint32_t m0TimeOfCurrentLine(uint32_t cc);
	bool cgbpAccessible(uint32_t cycleCounter);
	
	void mode3CyclesChange();
	void doCgbBgColorChange(uint32_t index, uint32_t data, uint32_t cycleCounter);
	void doCgbSpColorChange(uint32_t index, uint32_t data, uint32_t cycleCounter);

public:
	LCD(const uint8_t *oamram, const uint8_t *vram_in, VideoInterruptRequester memEventRequester);
	void reset(const uint8_t *oamram, const uint8_t *vram, bool cgb);
	void setStatePtrs(SaveState &state);
	void loadState(const SaveState &state, const uint8_t *oamram);
	void setVideoBuffer(uint32_t *videoBuf, std::size_t pitch);
	void setLayers(uint8_t mask) { ppu.setLayers(mask); }
	void setCgb(bool cgb);
	void copyCgbPalettesToDmg();
	void blackScreen();

	void dmgBgPaletteChange(const uint8_t data, const uint32_t cycleCounter) {
		update(cycleCounter);
		bgpData[0] = data;
		setDmgPalette(ppu.bgPalette(), dmgColorsRgb32, data);
	}

	void dmgSpPalette1Change(const uint8_t data, const uint32_t cycleCounter) {
		update(cycleCounter);
		objpData[0] = data;
		setDmgPalette(ppu.spPalette(), dmgColorsRgb32 + 4, data);
	}

	void dmgSpPalette2Change(const uint8_t data, const uint32_t cycleCounter) {
		update(cycleCounter);
		objpData[1] = data;
		setDmgPalette(ppu.spPalette() + 4, dmgColorsRgb32 + 8, data);
	}

	void cgbBgColorChange(uint32_t index, const uint32_t data, const uint32_t cycleCounter) {
		if (bgpData[index] != data)
			doCgbBgColorChange(index, data, cycleCounter);
	}

	void cgbSpColorChange(uint32_t index, const uint32_t data, const uint32_t cycleCounter) {
		if (objpData[index] != data)
			doCgbSpColorChange(index, data, cycleCounter);
	}

	uint8_t cgbBgColorRead(const uint32_t index, const uint32_t cycleCounter) {
		return ppu.cgb() & cgbpAccessible(cycleCounter) ? bgpData[index] : 0xFF;
	}

	uint8_t cgbSpColorRead(const uint32_t index, const uint32_t cycleCounter) {
		return ppu.cgb() & cgbpAccessible(cycleCounter) ? objpData[index] : 0xFF;
	}

	void updateScreen(bool blanklcd, uint32_t cc);
	void resetCc(uint32_t oldCC, uint32_t newCc);
	void speedChange(uint32_t cycleCounter);
	bool vramAccessible(uint32_t cycleCounter);
	bool oamReadable(uint32_t cycleCounter);
	bool oamWritable(uint32_t cycleCounter);
	void wxChange(uint32_t newValue, uint32_t cycleCounter);
	void wyChange(uint32_t newValue, uint32_t cycleCounter);
	void oamChange(uint32_t cycleCounter);
	void oamChange(const uint8_t *oamram, uint32_t cycleCounter);
	void scxChange(uint32_t newScx, uint32_t cycleCounter);
	void scyChange(uint32_t newValue, uint32_t cycleCounter);

	void vramChange(const uint32_t cycleCounter) { update(cycleCounter); }

	uint8_t getStat(uint32_t lycReg, uint32_t cycleCounter);

	uint8_t getLyReg(const uint32_t cycleCounter) {
		uint8_t lyReg = 0;

		if (ppu.lcdc() & 0x80) {
			if (cycleCounter >= ppu.lyCounter().time())
				update(cycleCounter);

			lyReg = ppu.lyCounter().ly();
			
			if (lyReg == 153) {
				if (isDoubleSpeed()) {
					if (ppu.lyCounter().time() - cycleCounter <= 456 * 2 - 8)
						lyReg = 0;
				} else
					lyReg = 0;
			} else if (ppu.lyCounter().time() - cycleCounter <= 4)
				++lyReg;
		}

		return lyReg;
	}

	uint32_t nextMode1IrqTime() const { return eventTimes_(MODE1_IRQ); }

	void lcdcChange(uint32_t data, uint32_t cycleCounter);
	void lcdstatChange(uint32_t data, uint32_t cycleCounter);
	void lycRegChange(uint32_t data, uint32_t cycleCounter);
	
	void enableHdma(uint32_t cycleCounter);
	void disableHdma(uint32_t cycleCounter);
	bool hdmaIsEnabled() const { return eventTimes_(HDMA_REQ) != DISABLED_TIME; }
	
	void update(uint32_t cycleCounter);
	
	bool isCgb() const { return ppu.cgb(); }
	bool isDoubleSpeed() const { return ppu.lyCounter().isDoubleSpeed(); }

	uint32_t *bgPalette() { return ppu.bgPalette(); }
	uint32_t *spPalette() { return ppu.spPalette(); }

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
