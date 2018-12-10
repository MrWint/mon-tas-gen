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
	master(false)
{
	setEvent();
}

void Channel2::setEvent() {
	nextEventUnit = &envelopeUnit;
	if (lengthCounter.getCounter() < nextEventUnit->getCounter())
		nextEventUnit = &lengthCounter;
}

void Channel2::setNr1(const unsigned data) {
	lengthCounter.nr1Change(data, nr4, cycleCounter);
	setEvent();
}

void Channel2::setNr2(const unsigned data) {
	if (envelopeUnit.nr2Change(data))
		master = false;
	
	setEvent();
}

void Channel2::setNr3(const unsigned data) {
	setEvent();
}

void Channel2::setNr4(const unsigned data) {
	lengthCounter.nr4Change(nr4, data, cycleCounter);
		
	nr4 = data;
	
	if (data & 0x80) { //init-bit
		nr4 &= 0x7F;
		master = !envelopeUnit.nr4Init(cycleCounter);
	}
	
	setEvent();
}

void Channel2::setSo() {
	setEvent();
}

void Channel2::reset() {
	cycleCounter = 0x1000 | (cycleCounter & 0xFFF); // cycleCounter >> 12 & 7 represents the frame sequencer position.
	
// 	lengthCounter.reset();
	envelopeUnit.reset();
	
	setEvent();
}

void Channel2::init(const bool cgb) {
	lengthCounter.init(cgb);
}

void Channel2::loadState(const SaveState &state) {
	envelopeUnit.loadState(state.spu.ch2.env, state.mem.ioamhram.get()[0x117], state.spu.cycleCounter);
	lengthCounter.loadState(state.spu.ch2.lcounter, state.spu.cycleCounter);
	
	cycleCounter = state.spu.cycleCounter;
	nr4 = state.spu.ch2.nr4;
	master = state.spu.ch2.master;
}

void Channel2::update(unsigned long cycles) {
	const unsigned long endCycles = cycleCounter + cycles;
	
	for (;;) {
		const unsigned long nextMajorEvent = nextEventUnit->getCounter() < endCycles ? nextEventUnit->getCounter() : endCycles;
		
		if (cycleCounter < nextMajorEvent) {
			cycleCounter = nextMajorEvent;
		}
		
		if (nextEventUnit->getCounter() == nextMajorEvent) {
			nextEventUnit->event();
			setEvent();
		} else
			break;
	}
	
	if (cycleCounter & SoundUnit::COUNTER_MAX) {
		lengthCounter.resetCounters(cycleCounter);
		envelopeUnit.resetCounters(cycleCounter);
		
		cycleCounter -= SoundUnit::COUNTER_MAX;
	}
}

SYNCFUNC(Channel2)
{
	SSS(lengthCounter);
	SSS(envelopeUnit);

	EBS(nextEventUnit, 0);
	EVS(nextEventUnit, &envelopeUnit, 1);
	EVS(nextEventUnit, &lengthCounter, 2);
	EES(nextEventUnit, NULL);

	NSS(cycleCounter);

	NSS(nr4);
	NSS(master);
}

}
