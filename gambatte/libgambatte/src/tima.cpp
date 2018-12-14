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
#include "tima.h"
#include "savestate.h"

static const uint8_t timaClock[4] = { 10, 4, 6, 8 };

namespace gambatte {

Tima::Tima() :
lastUpdate_(0),
tmatime_(DISABLED_TIME),
tima_(0),
tma_(0),
tac_(0)
{}

void Tima::loadState(const SaveState &state, const TimaInterruptRequester timaIrq) {
	lastUpdate_ = state.mem.timaLastUpdate;
	tmatime_ = state.mem.tmatime;
	
	tima_ = state.mem.ioamhram.get()[0x105];
	tma_  = state.mem.ioamhram.get()[0x106];
	tac_  = state.mem.ioamhram.get()[0x107];
	
	timaIrq.setNextIrqEventTime((tac_ & 4)
		?
			(tmatime_ != DISABLED_TIME && tmatime_ > state.cpu.cycleCounter
			          ? tmatime_
			          : lastUpdate_ + ((256u - tima_) << timaClock[tac_ & 3]) + 3)
		:
			static_cast<uint32_t>(DISABLED_TIME)
	);
}

void Tima::resetCc(const uint32_t oldCc, const uint32_t newCc, const TimaInterruptRequester timaIrq) {
	const uint32_t dec = oldCc - newCc;
	
	if (tac_ & 0x04) {
		updateIrq(oldCc, timaIrq);
		updateTima(oldCc);
		
		lastUpdate_ -= dec;
		timaIrq.setNextIrqEventTime(timaIrq.nextIrqEventTime() - dec);
		
		if (tmatime_ != DISABLED_TIME)
			tmatime_ -= dec;
	}
}

void Tima::updateTima(const uint32_t cycleCounter) {
	const uint32_t ticks = (cycleCounter - lastUpdate_) >> timaClock[tac_ & 3];

	lastUpdate_ += ticks << timaClock[tac_ & 3];

	if (cycleCounter >= tmatime_) {
		if (cycleCounter >= tmatime_ + 4)
			tmatime_ = DISABLED_TIME;

		tima_ = tma_;
	}

	uint32_t tmp = tima_ + ticks;

	while (tmp > 0x100)
		tmp -= 0x100 - tma_;

	if (tmp == 0x100) {
		tmp = 0;
		tmatime_ = lastUpdate_ + 3;

		if (cycleCounter >= tmatime_) {
			if (cycleCounter >= tmatime_ + 4)
				tmatime_ = DISABLED_TIME;

			tmp = tma_;
		}
	}

	tima_ = tmp;
}

void Tima::setTima(const uint32_t data, const uint32_t cycleCounter, const TimaInterruptRequester timaIrq) {
	if (tac_ & 0x04) {
		updateIrq(cycleCounter, timaIrq);
		updateTima(cycleCounter);

		if (tmatime_ - cycleCounter < 4)
			tmatime_ = DISABLED_TIME;

		timaIrq.setNextIrqEventTime(lastUpdate_ + ((256u - data) << timaClock[tac_ & 3]) + 3);
	}
	
	tima_ = data;
}

void Tima::setTma(const uint32_t data, const uint32_t cycleCounter, const TimaInterruptRequester timaIrq) {
	if (tac_ & 0x04) {
		updateIrq(cycleCounter, timaIrq);
		updateTima(cycleCounter);
	}
	
	tma_ = data;
}

void Tima::setTac(const uint32_t data, const uint32_t cycleCounter, const TimaInterruptRequester timaIrq, bool gbIsCgb) {
	if (tac_ ^ data) {
		uint32_t nextIrqEventTime = timaIrq.nextIrqEventTime();
		
		if (tac_ & 0x04) {
			updateIrq(cycleCounter, timaIrq);
			updateTima(cycleCounter);

			lastUpdate_ -= (1u << (timaClock[tac_ & 3] - 1)) + 3;
			tmatime_ -= (1u << (timaClock[tac_ & 3] - 1)) + 3;
			nextIrqEventTime -= (1u << (timaClock[tac_ & 3] - 1)) + 3;
			
			if (cycleCounter >= nextIrqEventTime)
				timaIrq.flagIrq();
			
			updateTima(cycleCounter);

			tmatime_ = DISABLED_TIME;
			nextIrqEventTime = DISABLED_TIME;
		}

		if (data & 4) {
			lastUpdate_ = (cycleCounter >> timaClock[data & 3]) << timaClock[data & 3];
			uint32_t diff = cycleCounter - basetime_;
			
			if (gbIsCgb) {
				if (((diff >> (timaClock[tac_ & 3] - 1)) & 1) == 1 && ((diff >> (timaClock[data & 3] - 1)) & 1) == 0)
					tima_++;
			}		
			lastUpdate_ = basetime_ + ((diff >> timaClock[data & 3]) << timaClock[data & 3]);
			nextIrqEventTime = lastUpdate_ + ((256u - tima_) << timaClock[data & 3]) + 3;
		}
		
		timaIrq.setNextIrqEventTime(nextIrqEventTime);
	}
	
	tac_ = data;
}

void Tima::resTac(uint32_t const cycleCounter, TimaInterruptRequester timaIrq) {
	basetime_ = cycleCounter;
	if (tac_ & 0x04) {
		setTac(tac_ & ~0x04, cycleCounter, timaIrq, false);
		setTac(tac_ | 0x04, cycleCounter, timaIrq, false);
	}
}

uint8_t Tima::tima(uint32_t cycleCounter) {
	if (tac_ & 0x04)
		updateTima(cycleCounter);

	return tima_;
}

void Tima::doIrqEvent(const TimaInterruptRequester timaIrq) {
	timaIrq.flagIrq();
	timaIrq.setNextIrqEventTime(timaIrq.nextIrqEventTime() + ((256u - tma_) << timaClock[tac_ & 3]));
}

SYNCFUNC(Tima)
{
	NSS(lastUpdate_);
	NSS(basetime_);
	NSS(tmatime_);
	NSS(tima_);
	NSS(tma_);
	NSS(tac_);
}

}
