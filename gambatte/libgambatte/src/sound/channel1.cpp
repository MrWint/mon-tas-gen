/***************************************************************************
 *   Copyright (C) 2007 by Sindre Aam�s                                    *
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
#include "channel1.h"
#include "../savestate.h"
#include <algorithm>


namespace gambatte {

Channel1::SweepUnit::SweepUnit(MasterDisabler &disabler, DutyUnit &dutyUnit) :
	disableMaster(disabler),
	dutyUnit(dutyUnit),
	shadow(0),
	nr0(0),
	negging(false)
{}

uint32_t Channel1::SweepUnit::calcFreq() {
	uint32_t freq = shadow >> (nr0 & 0x07);
	
	if (nr0 & 0x08) {
		freq = shadow - freq;
		negging = true;
	} else
		freq = shadow + freq;
	
	if (freq & 2048)
		disableMaster();
	
	return freq;
}

void Channel1::SweepUnit::event() {
	const uint32_t period = nr0 >> 4 & 0x07;
	
	if (period) {
		const uint32_t freq = calcFreq();
		
		if (!(freq & 2048) && (nr0 & 0x07)) {
			shadow = freq;
			dutyUnit.setFreq(freq);
			calcFreq();
		}
		
		counter += period << 14;
	} else
		counter += 8ul << 14;
}

void Channel1::SweepUnit::nr0Change(const uint32_t newNr0) {
	if (negging && !(newNr0 & 0x08))
		disableMaster();
	
	nr0 = newNr0;
}

void Channel1::SweepUnit::nr4Init(const uint32_t cc) {
	negging = false;
	shadow = dutyUnit.getFreq();
	
	const uint32_t period = nr0 >> 4 & 0x07;
	const uint32_t shift = nr0 & 0x07;
	
	if (period | shift)
		counter = ((cc >> 14) + (period ? period : 8)) << 14;
	else
		counter = COUNTER_DISABLED;
	
	if (shift)
		calcFreq();
}

void Channel1::SweepUnit::reset() {
	counter = COUNTER_DISABLED;
}

void Channel1::SweepUnit::loadState(const SaveState &state) {
	counter = std::max(state.spu.ch1.sweep.counter, state.spu.cycleCounter);
	shadow = state.spu.ch1.sweep.shadow;
	nr0 = state.spu.ch1.sweep.nr0;
	negging = state.spu.ch1.sweep.negging;
}

template<bool isReader>
void Channel1::SweepUnit::SyncState(NewState *ns)
{
	NSS(counter);
	NSS(shadow);
	NSS(nr0);
	NSS(negging);
}

Channel1::Channel1() :
	disableMaster(master),
	lengthCounter(disableMaster, 0x3F),
	envelopeUnit(),
	sweepUnit(disableMaster, dutyUnit),
	cycleCounter(0),
	nr4(0),
	master(false)
{
	setEvent();
}

void Channel1::setEvent() {
	nextEventUnit = &sweepUnit;
	if (lengthCounter.getCounter() < nextEventUnit->getCounter())
		nextEventUnit = &lengthCounter;
}

void Channel1::setNr0(const uint32_t data) {
	sweepUnit.nr0Change(data);
	setEvent();
}

void Channel1::setNr1(const uint32_t data) {
	lengthCounter.nr1Change(data, nr4, cycleCounter);

	setEvent();
}

void Channel1::setNr2(const uint32_t data) {
	if (envelopeUnit.nr2Change(data))
		disableMaster();
	
	setEvent();
}

void Channel1::setNr3(const uint32_t data) {
	dutyUnit.nr3Change(data);
	setEvent();
}

void Channel1::setNr4(const uint32_t data) {
	lengthCounter.nr4Change(nr4, data, cycleCounter);
		
	nr4 = data;
	
	dutyUnit.nr4Change(data);
	
	if (data & 0x80) { //init-bit
		nr4 &= 0x7F;
		master = !envelopeUnit.nr4Init();
		sweepUnit.nr4Init(cycleCounter);
	}
	
	setEvent();
}

void Channel1::reset() {
	cycleCounter = 0x1000 | (cycleCounter & 0xFFF); // cycleCounter >> 12 & 7 represents the frame sequencer position.

	sweepUnit.reset();
	
	setEvent();
}

void Channel1::loadState(const SaveState &state) {
	sweepUnit.loadState(state);
	dutyUnit.loadState(state.spu.ch1.duty, state.spu.ch1.nr4);
	envelopeUnit.loadState(state.mem.ioamhram.get()[0x112]);
	lengthCounter.loadState(state.spu.ch1.lcounter, state.spu.cycleCounter);
	
	cycleCounter = state.spu.cycleCounter;
	nr4 = state.spu.ch1.nr4;
	master = state.spu.ch1.master;
}

void Channel1::update(uint32_t cycles) {
	const uint32_t endCycles = cycleCounter + cycles;
	
	for (;;) {
		const uint32_t nextMajorEvent = nextEventUnit->getCounter() < endCycles ? nextEventUnit->getCounter() : endCycles;

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
		lengthCounter.resetCounters();
		sweepUnit.resetCounters();
		
		cycleCounter -= SoundUnit::COUNTER_MAX;
	}
}

SYNCFUNC(Channel1)
{
	SSS(lengthCounter);
	SSS(dutyUnit);
	SSS(envelopeUnit);
	SSS(sweepUnit);

	EBS(nextEventUnit, 0);
	EVS(nextEventUnit, &sweepUnit, 1);
	EVS(nextEventUnit, &lengthCounter, 2);
	EES(nextEventUnit, NULL);

	NSS(cycleCounter);

	NSS(nr4);
	NSS(master);
}

}
