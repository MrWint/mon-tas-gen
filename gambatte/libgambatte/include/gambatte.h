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
#ifndef GAMBATTE_H
#define GAMBATTE_H

#include <string>
#include <sstream>
#include <cstdint>
#include "newstate.h"

namespace gambatte {
class GB {
public:
	GB();
	~GB();
	
	enum LoadFlag {
		FORCE_DMG        = 1, /**< Treat the ROM as not having CGB support regardless of what its header advertises. */
		GBA_CGB          = 2, /**< Use GBA intial CPU register values when in CGB mode. */
		MULTICART_COMPAT = 4, /**< Use heuristics to detect and support some multicart MBCs disguised as MBC1. */
	};
	
	/** Load ROM image.
	  *
	  * @param romfile  Path to rom image file. Typically a .gbc, .gb, or .zip-file (if zip-support is compiled in).
	  * @param flags    ORed combination of LoadFlags.
	  * @return 0 on success, negative value on failure.
	  */
	int32_t load(const uint8_t *romfiledata, uint32_t romfilelength, uint32_t now, uint8_t flags, uint32_t div);
	
	void loadGBCBios(const uint8_t* biosfiledata);
	void loadDMGBios(const uint8_t* biosfiledata);

	/** Emulates until at least 'samples' stereo sound samples are produced in the supplied buffer,
	  * or until a video frame has been drawn.
	  *
	  * There are 35112 stereo sound samples in a video frame.
	  * May run for up to 2064 stereo samples too long.
	  * A stereo sample consists of two native endian 2s complement 16-bit PCM samples,
	  * with the left sample preceding the right one.
	  *
	  * Returns early when a new video frame has finished drawing in the video buffer,
	  * such that the caller may update the video output before the frame is overwritten.
	  * The return value indicates whether a new video frame has been drawn, and the
	  * exact time (in number of samples) at which it was drawn.
	  *
	  * @param samples in: number of stereo samples to produce, out: actual number of samples produced
	  * @return sample number at which the video frame was produced. -1 means no frame was produced.
	  */
	int32_t runFor(uint32_t &samples);

	void setLayers(uint8_t mask);

	/** Reset to initial state.
	  * Equivalent to reloading a ROM image, or turning a Game Boy Color off and on again.
	  */
	void reset(uint32_t now, uint32_t div);

	/** Sets the callback used for getting input state. */
	void setInputGetter(uint8_t (*getInput)(void *), void* context);
	
	void setRTCCallback(uint32_t (*callback)(void*), void* context);

	/** Writes persistent cartridge data to disk. NOT Done implicitly on ROM close. */
	void loadSavedata(const uint8_t *data);
	size_t saveSavedataLength();
	void saveSavedata(uint8_t *dest);
	
	// 0 = vram, 1 = rom, 2 = wram, 3 = cartram, 4 = oam, 5 = hram
	bool getMemoryArea(int32_t which, uint8_t **data, int32_t *length);

	uint8_t ExternalRead(uint16_t addr);
	void ExternalWrite(uint16_t addr, uint8_t val);

	void GetRegs(uint32_t *dest);

	void SetInterruptAddresses(int32_t *addrs, uint32_t numAddrs);
	int32_t GetHitInterruptAddress();

	uint16_t getDivState();
	void setVideoBuffer(uint32_t *const videoBuf, const std::size_t pitch);

	template<bool isReader>void SyncState(NewState *ns);

private:
	struct Priv;
	Priv *const p_;

	GB(const GB &);
	GB & operator=(const GB &);
};
}

#endif
