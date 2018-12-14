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
#ifndef RTC_H
#define RTC_H

#include <cstdint>
#include "newstate.h"

namespace gambatte {

struct SaveState;

class Rtc {
private:
	uint8_t *activeData;
	void (Rtc::*activeSet)(uint8_t);
	uint32_t baseTime;
	uint32_t haltTime;
	uint8_t index;
	uint8_t dataDh;
	uint8_t dataDl;
	uint8_t dataH;
	uint8_t dataM;
	uint8_t dataS;
	bool enabled;
	uint8_t lastLatchData;
	uint32_t (*timeCB)(void*);
	void* timeCBcontext;
	
	void doLatch();
	void doSwapActive();
	void setDh(uint8_t new_dh);
	void setDl(uint8_t new_lowdays);
	void setH(uint8_t new_hours);
	void setM(uint8_t new_minutes);
	void setS(uint8_t new_seconds);
	
public:
	Rtc();
	
	const uint8_t* getActive() const { return activeData; }
	uint32_t getBaseTime() const { return baseTime; }
	
	void setBaseTime(const uint32_t _baseTime) {
		baseTime = _baseTime;
	}
	
	void latch(const uint8_t data) {
		if (!lastLatchData && data == 1)
			doLatch();
		
		lastLatchData = data;
	}
	
	void loadState(const SaveState &state);
	
	void set(const bool _enabled, uint8_t bank) {
		bank &= 0xF;
		bank -= 8;
		
		enabled = _enabled;
		index = bank;
		
		doSwapActive();
	}
	
	void write(const uint8_t data) {
		(this->*activeSet)(data);
		*activeData = data;
	}

	void setRTCCallback(uint32_t (*callback)(void*), void* context) {
		timeCB = callback;
		timeCBcontext = context;
	}

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
