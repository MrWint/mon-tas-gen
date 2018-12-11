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
#ifndef ENVELOPE_UNIT_H
#define ENVELOPE_UNIT_H

#include "../savestate.h"
#include "newstate.h"

namespace gambatte {

class EnvelopeUnit {
private:
	unsigned char nr2;

public:
	EnvelopeUnit() : nr2(0) {}
	bool nr2Change(unsigned newNr2);
	bool nr4Init() { return !(nr2 & 0xF8); }
	void loadState(unsigned nr2)  { this->nr2 = nr2; }

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
