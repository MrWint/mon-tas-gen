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
#ifndef TIMA_H
#define TIMA_H

#include "interruptrequester.h"

namespace gambatte {

class TimaInterruptRequester {
	InterruptRequester &intreq;
	
public:
	explicit TimaInterruptRequester(InterruptRequester &intreq) : intreq(intreq) {}
	void flagIrq() const { intreq.flagIrq(4); }
	uint32_t nextIrqEventTime() const { return intreq.eventTime(TIMA); }
	void setNextIrqEventTime(const uint32_t time) const { intreq.setEventTime<TIMA>(time); }
};

class Tima {
	uint32_t basetime_;
	uint32_t lastUpdate_;
	uint32_t tmatime_;
	
	uint8_t tima_;
	uint8_t tma_;
	uint8_t tac_;
	
	void updateIrq(const uint32_t cc, const TimaInterruptRequester timaIrq) {
		while (cc >= timaIrq.nextIrqEventTime())
			doIrqEvent(timaIrq);
	}
	
	void updateTima(uint32_t cc);
	
public:
	Tima();
	void loadState(const SaveState &, TimaInterruptRequester timaIrq);
	void resetCc(uint32_t oldCc, uint32_t newCc, TimaInterruptRequester timaIrq);
	
	void setTima(uint32_t tima, uint32_t cc, TimaInterruptRequester timaIrq);
	void setTma(uint32_t tma, uint32_t cc, TimaInterruptRequester timaIrq);
	void setTac(uint32_t tac, uint32_t cc, TimaInterruptRequester timaIrq, bool gbIsCgb);
	void resTac(uint32_t cc, TimaInterruptRequester timaIrq);
	uint8_t tima(uint32_t cc);
	
	void doIrqEvent(TimaInterruptRequester timaIrq);

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
