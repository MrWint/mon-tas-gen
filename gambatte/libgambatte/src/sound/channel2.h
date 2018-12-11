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

#include "gbint.h"
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
	
	unsigned long cycleCounter;
	unsigned long soMask;
	
	unsigned char nr4;
	bool master;
	
	void setEvent();
	
public:
	Channel2();
	void setNr1(unsigned data);
	void setNr2(unsigned data);
	void setNr4(unsigned data);
	
	bool isActive() const { return master; }
	
	void update(unsigned long cycles);
	
	void reset();
	void init(bool cgb);
	void loadState(const SaveState &state);

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
