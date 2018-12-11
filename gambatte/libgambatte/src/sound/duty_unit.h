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
#ifndef DUTY_UNIT_H
#define DUTY_UNIT_H

#include "../savestate.h"
#include "newstate.h"

namespace gambatte {

class DutyUnit {
	unsigned long nextPosUpdate;
	unsigned short period;
	unsigned char pos;

public:
	DutyUnit();
	void nr3Change(unsigned newNr3);
	void nr4Change(unsigned newNr4);
	void loadState(const SaveState::SPU::Duty &dstate, unsigned nr4);

	//intended for use by SweepUnit only.
	unsigned getFreq() const { return 2048 - (period >> 1); }
	void setFreq(unsigned newFreq);

	template<bool isReader>void SyncState(NewState *ns);
};
}

#endif
