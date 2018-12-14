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
#include "sound.h"
#include "savestate.h"
#include <cstring>
#include <algorithm>

/*
	Frame Sequencer

	Step    Length Ctr  Vol Env     Sweep
	- - - - - - - - - - - - - - - - - - - -
	0       Clock       -           Clock
S	1       -           Clock       -
	2       Clock       -           -
	3       -           -           -
	4       Clock       -           Clock
	5       -           -           -
	6       Clock       -           -
	7       -           -           -
	- - - - - - - - - - - - - - - - - - - -
	Rate    256 Hz      64 Hz       128 Hz

S) start step on sound power on.
*/

namespace gambatte {

PSG::PSG()
: lastUpdate(0),
  bufferPos(0),
  enabled(false)
{
}

void PSG::init(const bool cgb) {
	ch3.init(cgb);
}

void PSG::reset() {
	ch1.reset();
	ch2.reset();
	ch3.reset();
	ch4.reset();
}

void PSG::setStatePtrs(SaveState &state) {
	ch3.setStatePtrs(state);
}

void PSG::loadState(const SaveState &state) {
	ch1.loadState(state);
	ch2.loadState(state);
	ch3.loadState(state);
	ch4.loadState(state);
	
	lastUpdate = state.cpu.cycleCounter;
	enabled = state.mem.ioamhram.get()[0x126] >> 7 & 1;
}

void PSG::accumulate_channels(const uint32_t cycles) {
	ch1.update(cycles);
	ch2.update(cycles);
	ch3.update(cycles);
	ch4.update(cycles);
}

void PSG::generate_samples(const uint32_t cycleCounter, const uint32_t doubleSpeed) {
	const uint32_t cycles = (cycleCounter - lastUpdate) >> (1 + doubleSpeed);
	lastUpdate += cycles << (1 + doubleSpeed);

	if (cycles)
		accumulate_channels(cycles);
	
	bufferPos += cycles;
}

void PSG::resetCounter(const uint32_t newCc, const uint32_t oldCc, const uint32_t doubleSpeed) {
	generate_samples(oldCc, doubleSpeed);
	lastUpdate = newCc - (oldCc - lastUpdate);
}

uint32_t PSG::fillBuffer() {
	return bufferPos;
}

uint8_t PSG::getStatus() const {
	return (uint8_t)ch1.isActive() | (uint8_t)ch2.isActive() << 1 | (uint8_t)ch3.isActive() << 2 | (uint8_t)ch4.isActive() << 3;
}

// the buffer and position are not saved, as they're set and flushed on each runfor() call
SYNCFUNC(PSG)
{
	SSS(ch1);
	SSS(ch2);
	SSS(ch3);
	SSS(ch4);
	NSS(lastUpdate);
	NSS(enabled);
}

}
