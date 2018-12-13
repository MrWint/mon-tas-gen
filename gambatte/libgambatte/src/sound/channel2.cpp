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
#include "channel2.h"
#include "../savestate.h"

namespace gambatte {

Channel2::Channel2() :
	disableMaster(master),
	lengthCounter(disableMaster, 0x3F),
	envelopeUnit(),
	cycleCounter(0),
	nr4(0),
	master(false) {}

void Channel2::setNr1(const uint32_t data) {
	lengthCounter.nr1Change(data, nr4, cycleCounter);
}

void Channel2::setNr2(const uint32_t data) {
	if (envelopeUnit.nr2Change(data)) master = false;
}

void Channel2::setNr4(const uint32_t data) {
	lengthCounter.nr4Change(nr4, data, cycleCounter);
		
	nr4 = data;
	
	if (data & 0x80) { //init-bit
		nr4 &= 0x7F;
		master = !envelopeUnit.nr4Init();
	}
}

void Channel2::reset() {
	cycleCounter = 0x1000 | (cycleCounter & 0xFFF); // cycleCounter >> 12 & 7 represents the frame sequencer position.
}

void Channel2::loadState(const SaveState &state) {
	envelopeUnit.loadState(state.mem.ioamhram.get()[0x117]);
	lengthCounter.loadState(state.spu.ch2.lcounter, state.spu.cycleCounter);
	
	cycleCounter = state.spu.cycleCounter;
	nr4 = state.spu.ch2.nr4;
	master = state.spu.ch2.master;
}

void Channel2::update(uint32_t cycles) {
	cycleCounter += cycles;
	if (lengthCounter.getCounter() < cycleCounter)
		lengthCounter.event(); // lengthCounter can only trigger once, and disables adterwards.
	
	if (cycleCounter & SoundUnit::COUNTER_MAX) {
		lengthCounter.resetCounters();

		cycleCounter -= SoundUnit::COUNTER_MAX;
	}
}

SYNCFUNC(Channel2)
{
	SSS(lengthCounter);
	SSS(envelopeUnit);

	NSS(cycleCounter);

	NSS(nr4);
	NSS(master);
}

}
