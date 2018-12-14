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
#ifndef SOUND_UNIT_H
#define SOUND_UNIT_H

#include <cstdint>

namespace gambatte {

class SoundUnit {
protected:
	uint32_t counter;
public:
  static const uint32_t COUNTER_MAX = 0x80000000u;
  static const uint32_t COUNTER_DISABLED = 0xFFFFFFFFu;

	SoundUnit() : counter(COUNTER_DISABLED) {}
	virtual ~SoundUnit() {}
	virtual void event() = 0;
	uint32_t getCounter() const { return counter; }
	virtual void resetCounters() { if (counter != COUNTER_DISABLED) counter -= COUNTER_MAX; }
};

}

#endif
