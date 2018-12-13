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
#ifndef SOUND_CHANNEL2_H
#define SOUND_CHANNEL2_H

#include "length_counter.h"
#include "master_disabler.h"
#include "envelope_unit.h"
#include "newstate.h"

namespace gambatte {

struct SaveState;

class Channel2 {
	MasterDisabler disableMaster;
	LengthCounter lengthCounter;
	EnvelopeUnit envelopeUnit;
	
	uint32_t cycleCounter;
	
	uint8_t nr4;
	bool master;

public:
	Channel2();
	void setNr1(uint32_t data);
	void setNr2(uint32_t data);
	void setNr4(uint32_t data);
	
	bool isActive() const { return master; }
	
	void update(uint32_t cycles);
	
	void reset();
	void loadState(const SaveState &state);

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
