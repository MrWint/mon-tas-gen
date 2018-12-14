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
#ifndef LENGTH_COUNTER_H
#define LENGTH_COUNTER_H

#include "sound_unit.h"
#include "../savestate.h"
#include "newstate.h"

namespace gambatte {

class MasterDisabler;

class LengthCounter : public SoundUnit {
	MasterDisabler &disableMaster;
	uint16_t lengthCounter;
	const uint8_t lengthMask;

public:
	LengthCounter(MasterDisabler &disabler, uint32_t lengthMask);
	void event();
	void nr1Change(uint32_t newNr1, uint32_t nr4, uint32_t cc);
	void nr4Change(uint32_t oldNr4, uint32_t newNr4, uint32_t cc);
	void loadState(const SaveState::SPU::LCounter &lstate, uint32_t cc);

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
