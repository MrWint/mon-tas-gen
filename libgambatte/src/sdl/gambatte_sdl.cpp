/***************************************************************************
 *   Copyright (C) 2007 by Sindre Aamås                                    *
 *   sinamas@users.sourceforge.net                                         *
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
#include <gambatte.h>
#include "common/array.h"
#include <SDL.h>
#include <cstdlib>
#include <cstring>
#include <cstdio>
#include <string>
#include <sstream>
#include <iostream>
#include <algorithm>
#include <memory>
#include <vector>
#include <map>

#include "blitterwrapper.h"
#include "rtccallback.h"

namespace {

using namespace gambatte;

class GetInput : public InputGetter {
public:
	unsigned is;
	
	GetInput() : is(0) {}
	unsigned operator()() { return is; }
};

class FrameTracker : public RtcCallback {
public:
  std::uint32_t numFrames;
	bool onFrameBoundaries;

	FrameTracker() : numFrames(0),onFrameBoundaries(true) {}

  std::uint32_t getCurrentTime() {
		std::uint64_t fn = (std::uint64_t)numFrames;

		// as we're exactly tracking cpu cycles, this can be pretty accurate
		fn *= 4389;
		fn /= 262144;
		return (std::uint32_t)fn;
	}
};

class SdlIniter {
	bool failed;
public:
	BlitterWrapper blitter;

	SdlIniter() : failed(false) {}
    void init(unsigned numScreens, unsigned scaleFactor) {

		if (SDL_Init(SDL_INIT_VIDEO) < 0) {
			std::fprintf(stderr, "Unable to init SDL: %s\n", SDL_GetError());
			failed = true;
		}

		blitter.setScale(scaleFactor);
		blitter.setYuv(false);

		blitter.init(numScreens);
		// SDL_ShowCursor(SDL_DISABLE);
		SDL_WM_SetCaption("Gambatte SDL", NULL);
	}
	~SdlIniter() { SDL_Quit(); }
	bool isFailed() const { return failed; }
};

SdlIniter sdl;

class GambatteSdl {
public:
	Array<Sint16> inBuf;
	GB gambatte;
	
	GetInput inputGetter;
	FrameTracker frameTracker;
	uint64_t cycleCount;
	
	unsigned overflowSamples;
	bool loadRom(const char *romfiledata, unsigned romfilelength);
	int screen;
	bool equalLengthFrames;
	
	GambatteSdl(int screen_, bool equalLengthFrames_) : inBuf((35112 + 2064) * 2),overflowSamples(0),screen(screen_),equalLengthFrames(equalLengthFrames_),cycleCount(0) {}
	void step();
	void saveState(std::vector<char>& data);
	void loadState(const std::vector<char>& data);
};

bool GambatteSdl::loadRom(const char *romfiledata, unsigned romfilelength) {
	{
		const bool gbaCgbOption = true;
		const bool forceDmgOption = false;
		const bool multicartCompatOption = false;
		
		if (int const loadError =
				gambatte.load(romfiledata, romfilelength, 0 /* now */,
				                gbaCgbOption          * GB::GBA_CGB
				              + forceDmgOption        * GB::FORCE_DMG
				              + multicartCompatOption * GB::MULTICART_COMPAT, 0 /* div */)) {
			std::printf("failed to load ROM data\n");
			return loadError;
		}

	}

	gambatte.setRTCCallback(&frameTracker);
	gambatte.setInputGetter(&inputGetter);
	gambatte.setLayers(0x7);

	return 0;
}

const unsigned SAMPLES_PER_FRAME = 35112;


void GambatteSdl::step() {

	if (frameTracker.onFrameBoundaries) frameTracker.numFrames++;

	BlitterWrapper::Buf vbuf;
	if (screen >= 0) vbuf = sdl.blitter.inBuf(screen);

	while (true) {
		unsigned emusamples = SAMPLES_PER_FRAME - overflowSamples;
		if (gambatte.runFor(
				reinterpret_cast<gambatte::uint_least32_t*>(inBuf.get()), emusamples) >= 0) {

			if (screen >= 0) {
				SDL_Event e;
				gambatte.blitTo(vbuf.pixels, vbuf.pitch);
				sdl.blitter.draw();
				sdl.blitter.present();
				while (SDL_PollEvent(&e)); // work through events to keep window responsive
			}
		}

		overflowSamples += emusamples;
		cycleCount += emusamples;

		if(gambatte.p_->cpu.hitInterruptAddress != 0) { // go into frame
			break;
		}

		if(!equalLengthFrames) { // old frame timing
			overflowSamples = 0; // completed frame
			break;
		}

		if (overflowSamples >= SAMPLES_PER_FRAME) { // new frame timing
			overflowSamples -= SAMPLES_PER_FRAME;
			break;
		}
	}
	frameTracker.onFrameBoundaries = (gambatte.p_->cpu.hitInterruptAddress == 0);
}
} // anon namespace



extern "C" void initSdlOutput(int numScreens, int scaleFactor) {
  sdl.init(numScreens, scaleFactor);
}

extern "C" GambatteSdl* createGb(int screen, bool equalLengthFrames) {
  GambatteSdl* gb = new GambatteSdl(screen, equalLengthFrames);
	return gb;
}
extern "C" void destroyGb(GambatteSdl* gb) {
  delete gb;
}

extern "C" void loadGbcBios(GambatteSdl* gb, const char *biosdata) {
  gb->gambatte.loadGBCBios(biosdata);
}

extern "C" void loadRom(GambatteSdl* gb, const char *romfiledata, unsigned romfilelength) {
  gb->loadRom(romfiledata, romfilelength);
}

extern "C" void setInput(GambatteSdl* gb, unsigned keymask) {
	gb->inputGetter.is = keymask;
}
extern "C" void step(GambatteSdl* gb) {
	gb->gambatte.SetInterruptAddresses(0, 0); // no interrupts
  gb->step();
}
extern "C" int stepUntil(GambatteSdl* gb, int *interruptAddresses, int numInterruptAddresses) {
	gb->gambatte.SetInterruptAddresses(interruptAddresses, numInterruptAddresses);
  gb->step();
	return gb->gambatte.GetHitInterruptAddress();
}

extern "C" void reset(GambatteSdl* gb) {
	if (!gb->frameTracker.onFrameBoundaries) { // forward to next frame boundary
		gb->gambatte.SetInterruptAddresses(0, 0); // no interrupts
		gb->step();
	}
	{
		gb->frameTracker.numFrames++; // temporarily add a frame since BizHawk increases the frame before checking for resets, so current time is accurate.
		gb->gambatte.reset(gb->frameTracker.getCurrentTime(), 0 /* div */);
		gb->frameTracker.numFrames--;
	}
	gb->inputGetter.is = 0;
  gb->step(); // BizHawk completes a frame on the reset input
}

extern "C" int saveState(GambatteSdl *gb, char *data, int len) {
	NewStateExternalBuffer saver(data, len);
	gb->gambatte.SyncState<false>(&saver);
	saver.Save(&gb->overflowSamples, sizeof(gb->overflowSamples), "overflowSamples");
	saver.Save(&gb->inputGetter.is, sizeof(gb->inputGetter.is), "inputGetter.is");
	saver.Save(&gb->frameTracker.numFrames, sizeof(gb->frameTracker.numFrames), "numFrames");
	saver.Save(&gb->frameTracker.onFrameBoundaries, sizeof(gb->frameTracker.onFrameBoundaries), "onFrameBoundaries");
	saver.Save(&gb->cycleCount, sizeof(gb->cycleCount), "cycleCount");
	return saver.GetLength();
}
extern "C" int loadState(GambatteSdl *gb, const char *data, int len) {
	NewStateExternalBuffer loader((char *)data, len);
	gb->gambatte.SyncState<true>(&loader);
	loader.Load(&gb->overflowSamples, sizeof(gb->overflowSamples), "overflowSamples");
	loader.Load(&gb->inputGetter.is, sizeof(gb->inputGetter.is), "inputGetter.is");
	loader.Load(&gb->frameTracker.numFrames, sizeof(gb->frameTracker.numFrames), "numFrames");
	loader.Load(&gb->frameTracker.onFrameBoundaries, sizeof(gb->frameTracker.onFrameBoundaries), "onFrameBoundaries");
	loader.Load(&gb->cycleCount, sizeof(gb->cycleCount), "cycleCount");
	return loader.GetLength();
}

extern "C" std::uint32_t getNumFrames(GambatteSdl *gb) {
	return gb->frameTracker.numFrames;
}
extern "C" bool isOnFrameBoundaries(GambatteSdl *gb) {
	return gb->frameTracker.onFrameBoundaries;
}
extern "C" std::uint64_t getCycleCount(GambatteSdl *gb) {
	return gb->cycleCount;
}

extern "C" std::uint8_t readMemory(GambatteSdl *gb, std::uint16_t address) {
	return gb->gambatte.p_->cpu.memory.peek(address);
}
extern "C" void writeMemory(GambatteSdl *gb, std::uint16_t address, std::uint8_t value) {
	gb->gambatte.ExternalWrite(address, value);
}

extern "C" void readRegisters(GambatteSdl *gb, int *dest) {
	gb->gambatte.GetRegs(dest);
}

extern "C" int getMemoryArea(GambatteSdl *gb, int which, unsigned char **data, int *length) {
	return gb->gambatte.getMemoryArea(which, data, length);
}

extern "C" std::uint16_t readDivState(GambatteSdl *gb) {

  const std::uint32_t cycleCounter = gb->gambatte.p_->cpu.cycleCounter_;
  const std::uint32_t divLastUpdate = gb->gambatte.p_->cpu.memory.divLastUpdate;
  const std::uint32_t div = gb->gambatte.p_->cpu.memory.ioamhram[0x104];

  return (((div << 8) + cycleCounter - divLastUpdate) >> 2) & 0x3FFF;
}