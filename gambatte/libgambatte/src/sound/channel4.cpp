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
#include "channel4.h"
#include "../savestate.h"

namespace gambatte {

Channel4::Channel4() :
	disableMaster(master),
	lengthCounter(disableMaster, 0x3F),
	envelopeUnit(),
	cycleCounter(0),
	nr4(0),
	master(false) {}

void Channel4::setNr1(const unsigned data) {
	lengthCounter.nr1Change(data, nr4, cycleCounter);
}

void Channel4::setNr2(const unsigned data) {
	if (envelopeUnit.nr2Change(data)) master = false;
}

void Channel4::setNr4(const unsigned data) {
	lengthCounter.nr4Change(nr4, data, cycleCounter);
		
	nr4 = data;
	
	if (data & 0x80) { //init-bit
		nr4 &= 0x7F;
		master = !envelopeUnit.nr4Init();
	}
}

void Channel4::reset() {
	cycleCounter = 0x1000 | (cycleCounter & 0xFFF); // cycleCounter >> 12 & 7 represents the frame sequencer position.
}

void Channel4::loadState(const SaveState &state) {
	envelopeUnit.loadState(state.mem.ioamhram.get()[0x121]);
	lengthCounter.loadState(state.spu.ch4.lcounter, state.spu.cycleCounter);
	
	cycleCounter = state.spu.cycleCounter;
	nr4 = state.spu.ch4.nr4;
	master = state.spu.ch4.master;
}

void Channel4::update(unsigned long cycles) {
	cycleCounter += cycles;
	if (lengthCounter.getCounter() < cycleCounter)
		lengthCounter.event(); // lengthCounter can only trigger once, and disables adterwards.
	
	if (cycleCounter & SoundUnit::COUNTER_MAX) {
		lengthCounter.resetCounters(cycleCounter);

		cycleCounter -= SoundUnit::COUNTER_MAX;
	}
}

SYNCFUNC(Channel4)
{
	SSS(lengthCounter);
	SSS(envelopeUnit);

	NSS(cycleCounter);
	
	NSS(nr4);
	NSS(master);
}

}
