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
#ifndef SOUND_CHANNEL1_H
#define SOUND_CHANNEL1_H

#include "master_disabler.h"
#include "length_counter.h"
#include "duty_unit.h"
#include "envelope_unit.h"
#include "newstate.h"

namespace gambatte {

struct SaveState;

class Channel1 {
	class SweepUnit : public SoundUnit {
		MasterDisabler &disableMaster;
		DutyUnit &dutyUnit;
		uint16_t shadow;
		uint8_t nr0;
		bool negging;
		
		uint32_t calcFreq();
		
	public:
		SweepUnit(MasterDisabler &disabler, DutyUnit &dutyUnit);
		void event();
		void nr0Change(uint32_t newNr0);
		void nr4Init(uint32_t cycleCounter);
		void reset();
		void loadState(const SaveState &state);

		template<bool isReader>void SyncState(NewState *ns);
	};
	
	MasterDisabler disableMaster;
	LengthCounter lengthCounter;
	DutyUnit dutyUnit;
	EnvelopeUnit envelopeUnit;
	SweepUnit sweepUnit;
	
	SoundUnit *nextEventUnit;
	
	uint32_t cycleCounter;
	
	uint8_t nr4;
	bool master;
	
	void setEvent();
	
public:
	Channel1();
	void setNr0(uint32_t data);
	void setNr1(uint32_t data);
	void setNr2(uint32_t data);
	void setNr3(uint32_t data);
	void setNr4(uint32_t data);
	
	bool isActive() const { return master; }
	
	void update(uint32_t cycles);
	
	void reset();
	void loadState(const SaveState &state);

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
