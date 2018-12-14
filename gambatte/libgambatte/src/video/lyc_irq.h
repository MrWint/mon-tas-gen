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
#ifndef VIDEO_LYC_IRQ_H
#define VIDEO_LYC_IRQ_H

#include "newstate.h"

namespace gambatte {

struct SaveState;
class LyCounter;

class LycIrq {
	uint32_t time_;
 	uint8_t lycRegSrc_;
 	uint8_t statRegSrc_;
	uint8_t lycReg_;
	uint8_t statReg_;
	bool cgb_;
	
	void regChange(uint32_t statReg, uint32_t lycReg, const LyCounter &lyCounter, uint32_t cc);
	
public:
	LycIrq();
	void doEvent(uint8_t *ifreg, const LyCounter &lyCounter);
	uint32_t lycReg() const { return lycRegSrc_; }
	void loadState(const SaveState &state);
	uint32_t time() const { return time_; }
	void setCgb(const bool cgb) { cgb_ = cgb; }
	void lcdReset();
	void reschedule(const LyCounter & lyCounter, uint32_t cc);
	
	void statRegChange(uint32_t statReg, const LyCounter &lyCounter, uint32_t cc) {
		regChange(statReg, lycRegSrc_, lyCounter, cc);
	}
	
	void lycRegChange(uint32_t lycReg, const LyCounter &lyCounter, uint32_t cc) {
		regChange(statRegSrc_, lycReg, lyCounter, cc);
	}

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
