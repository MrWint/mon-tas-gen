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
#include "channel3.h"
#include "../savestate.h"
#include <cstring>
#include <algorithm>

static inline unsigned toPeriod(const unsigned nr3, const unsigned nr4) {
	return 0x800 - ((nr4 << 8 & 0x700) | nr3);
}

namespace gambatte {

Channel3::Channel3() :
	disableMaster(master, waveCounter),
	lengthCounter(disableMaster, 0xFF),
	cycleCounter(0),
	waveCounter(SoundUnit::COUNTER_DISABLED),
	lastReadTime(0),
	nr0(0),
	nr3(0),
	nr4(0),
	wavePos(0),
	master(false),
	cgb(false)
{}

void Channel3::setNr0(const unsigned data) {
	nr0 = data & 0x80;
	
	if (!(data & 0x80))
		disableMaster();
}

void Channel3::setNr4(const unsigned data) {
	lengthCounter.nr4Change(nr4, data, cycleCounter);
		
	nr4 = data & 0x7F;
	
	if (data & nr0/* & 0x80*/) {
		if (!cgb && waveCounter == cycleCounter + 1) {
			const unsigned pos = ((wavePos + 1) & 0x1F) >> 1;
			
			if (pos < 4)
				waveRam[0] = waveRam[pos];
			else
				std::memcpy(waveRam, waveRam + (pos & ~3), 4);
		}
		
		master = true;
		wavePos = 0;
		lastReadTime = waveCounter = cycleCounter + toPeriod(nr3, data) + 3;
	}
}

void Channel3::reset() {
	cycleCounter = 0x1000 | (cycleCounter & 0xFFF); // cycleCounter >> 12 & 7 represents the frame sequencer position.
}

void Channel3::init(const bool cgb) {
	this->cgb = cgb;
}

void Channel3::setStatePtrs(SaveState &state) {
	state.spu.ch3.waveRam.set(waveRam, sizeof waveRam);
}

void Channel3::loadState(const SaveState &state) {
	lengthCounter.loadState(state.spu.ch3.lcounter, state.spu.cycleCounter);
	
	cycleCounter = state.spu.cycleCounter;
	waveCounter = std::max(state.spu.ch3.waveCounter, state.spu.cycleCounter);
	lastReadTime = state.spu.ch3.lastReadTime;
	nr3 = state.spu.ch3.nr3;
	nr4 = state.spu.ch3.nr4;
	wavePos = state.spu.ch3.wavePos & 0x1F;
	master = state.spu.ch3.master;
	
	nr0 = state.mem.ioamhram.get()[0x11A] & 0x80;
}

void Channel3::updateWaveCounter(const unsigned long cc) {
	if (cc >= waveCounter) {
		const unsigned period = toPeriod(nr3, nr4);
		const unsigned long periods = (cc - waveCounter) / period;

		lastReadTime = waveCounter + periods * period;
		waveCounter = lastReadTime + period;

		wavePos += periods + 1;
		wavePos &= 0x1F;
	}
}

void Channel3::update(unsigned long cycles) {
	cycleCounter += cycles;
	
	if (lengthCounter.getCounter() <= cycleCounter) {
		updateWaveCounter(lengthCounter.getCounter());
		lengthCounter.event(); // Can only trigger once, disables afterwards.
	}
	updateWaveCounter(cycleCounter);
	
	if (cycleCounter & SoundUnit::COUNTER_MAX) {
		lengthCounter.resetCounters(cycleCounter);
		
		if (waveCounter != SoundUnit::COUNTER_DISABLED)
			waveCounter -= SoundUnit::COUNTER_MAX;
		
		lastReadTime -= SoundUnit::COUNTER_MAX;
		cycleCounter -= SoundUnit::COUNTER_MAX;
	}
}

SYNCFUNC(Channel3)
{
	NSS(waveRam);
	
	SSS(lengthCounter);

	NSS(cycleCounter);
	NSS(waveCounter);
	NSS(lastReadTime);

	NSS(nr0);
	NSS(nr3);
	NSS(nr4);
	NSS(wavePos);

	NSS(master);
}

}
