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
#ifndef LY_COUNTER_H
#define LY_COUNTER_H

#include "newstate.h"

namespace gambatte {

struct SaveState;

class LyCounter {
	uint32_t time_;
	uint16_t lineTime_;
	uint8_t ly_;
	bool ds;
	
public:
	LyCounter();
	void doEvent();
	bool isDoubleSpeed() const { return ds; }
	
	uint32_t frameCycles(const uint32_t cc) const {
		return ly_ * 456ul + lineCycles(cc);
	}
	
	uint32_t lineCycles(const uint32_t cc) const {
		return 456u - ((time_ - cc) >> isDoubleSpeed());
	}
	
	uint32_t lineTime() const { return lineTime_; }
	uint32_t ly() const { return ly_; }
	uint32_t nextLineCycle(uint32_t lineCycle, uint32_t cycleCounter) const;
	uint32_t nextFrameCycle(uint32_t frameCycle, uint32_t cycleCounter) const;
	void reset(uint32_t videoCycles, uint32_t lastUpdate);
	void setDoubleSpeed(bool ds_in);
	uint32_t time() const { return time_; }

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
