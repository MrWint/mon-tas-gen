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
#include "duty_unit.h"

static inline uint32_t toPeriod(const uint32_t freq) {
	return (2048 - freq) << 1;
}

namespace gambatte {

void DutyUnit::setFreq(const uint32_t newFreq) {
	period = toPeriod(newFreq);
}

void DutyUnit::nr3Change(const uint32_t newNr3) {
	setFreq((getFreq() & 0x700) | newNr3);
}

void DutyUnit::nr4Change(const uint32_t newNr4) {
	setFreq((newNr4 << 8 & 0x700) | (getFreq() & 0xFF));
}

DutyUnit::DutyUnit() : period(4096) {}

void DutyUnit::loadState(const SaveState::SPU::Duty &dstate, const uint32_t nr4) {
	period = toPeriod((nr4 << 8 & 0x700) | dstate.nr3);
}

SYNCFUNC(DutyUnit)
{
	NSS(period);
}

}
