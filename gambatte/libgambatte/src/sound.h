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
#ifndef SOUND_H
#define SOUND_H

#include "sound/channel1.h"
#include "sound/channel2.h"
#include "sound/channel3.h"
#include "sound/channel4.h"
#include "newstate.h"

namespace gambatte {

class PSG {
	Channel1 ch1;
	Channel2 ch2;
	Channel3 ch3;
	Channel4 ch4;
	
	uint32_t lastUpdate;

	uint32_t bufferPos;
	
	bool enabled;

	void accumulate_channels(uint32_t cycles);

public:
	PSG();
	void init(bool cgb);
	void reset();
	void setStatePtrs(SaveState &state);
	void loadState(const SaveState &state);

	void generate_samples(uint32_t cycleCounter, uint32_t doubleSpeed);
	void resetCounter(uint32_t newCc, uint32_t oldCc, uint32_t doubleSpeed);
	uint32_t fillBuffer();
	void setBuffer() { bufferPos = 0; }
	
	bool isEnabled() const { return enabled; }
	void setEnabled(bool value) { enabled = value; }

	void set_nr10(uint32_t data) { ch1.setNr0(data); }
	void set_nr11(uint32_t data) { ch1.setNr1(data); }
	void set_nr12(uint32_t data) { ch1.setNr2(data); }
	void set_nr13(uint32_t data) { ch1.setNr3(data); }
	void set_nr14(uint32_t data) { ch1.setNr4(data); }

	void set_nr21(uint32_t data) { ch2.setNr1(data); }
	void set_nr22(uint32_t data) { ch2.setNr2(data); }
	void set_nr24(uint32_t data) { ch2.setNr4(data); }

	void set_nr30(uint32_t data) { ch3.setNr0(data); }
	void set_nr31(uint32_t data) { ch3.setNr1(data); }
	void set_nr33(uint8_t data) { ch3.setNr3(data); }
	void set_nr34(uint32_t data) { ch3.setNr4(data); }
	uint8_t waveRamRead(uint32_t index) const { return ch3.waveRamRead(index); }
	void waveRamWrite(uint32_t index, uint8_t data) { ch3.waveRamWrite(index, data); }

	void set_nr41(uint32_t data) { ch4.setNr1(data); }
	void set_nr42(uint32_t data) { ch4.setNr2(data); }
	void set_nr44(uint32_t data) { ch4.setNr4(data); }

	uint8_t getStatus() const;

	template<bool isReader>void SyncState(NewState *ns);
};

}

#endif
