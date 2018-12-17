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
#include "gambatte.h"
#include "savestate.h"
#include "initstate.h"
#include <sstream>
#include <cstring>

namespace gambatte {
	
GB::GB() : loadflags(0) {}

int32_t GB::runFor(uint32_t &samples) {
	if (!cpu.loaded()) {
		samples = 0;
		return -1;
	}
	
	cpu.setSoundBuffer();
	const int32_t cyclesSinceBlit = cpu.runFor(samples * 2);
	samples = cpu.fillSoundBuffer();
	
	return cyclesSinceBlit < 0 ? cyclesSinceBlit : static_cast<int32_t>(samples) - (cyclesSinceBlit >> 1);
}

void GB::setVideoBuffer(uint32_t *const videoBuf, const size_t pitch) {
	cpu.setVideoBuffer(videoBuf, pitch);
}

void GB::setLayers(uint8_t mask)
{
	cpu.setLayers(mask);
}

void GB::reset(const uint32_t now, const uint32_t div) {
	if (cpu.loaded()) {
		
		size_t length = cpu.saveSavedataLength();
		uint8_t *s;
		if (length > 0)
		{
			s = (uint8_t *) std::malloc(length);
			cpu.saveSavedata(s);
		}
		
		SaveState state;
		cpu.setStatePtrs(state);
		setInitState(state, !(loadflags & FORCE_DMG), loadflags & GBA_CGB, now, div);
		cpu.loadState(state);
		if (length > 0)
		{
			cpu.loadSavedata(s);
			std::free(s);
		}
	}
}

void GB::setInputGetter(uint8_t (*getInput)(void *), void* context) {
	cpu.setInputGetter(getInput, context);
}

void GB::setRTCCallback(uint32_t (*callback)(void*), void* context) {
	cpu.setRTCCallback(callback, context);
}

int32_t GB::load(const uint8_t *romfiledata, size_t romfilelength, const uint32_t now, const uint8_t flags, const uint32_t div) {
	const int32_t failed = cpu.load(romfiledata, romfilelength, flags & FORCE_DMG, flags & MULTICART_COMPAT);
	
	if (!failed) {
		SaveState state;
		cpu.setStatePtrs(state);
		loadflags = flags;
		setInitState(state, !(flags & FORCE_DMG), flags & GBA_CGB, now, div);
		cpu.loadState(state);
	}
	
	return failed;
}

void GB::loadGBCBios(const uint8_t* biosfiledata) {
	memcpy(cpu.cgbBiosBuffer(), biosfiledata, 0x900);
}

void GB::loadDMGBios(const uint8_t* biosfiledata) {
	memcpy(cpu.dmgBiosBuffer(), biosfiledata, 0x100);
}

void GB::saveSavedata(uint8_t *dest) {
	if (cpu.loaded())
		cpu.saveSavedata(dest);
}
void GB::loadSavedata(const uint8_t *data) {
	if (cpu.loaded())
		cpu.loadSavedata(data);
}
size_t GB::saveSavedataLength() {
	if (cpu.loaded())
		return cpu.saveSavedataLength();
	else
		return 0;
}

bool GB::getMemoryArea(int32_t which, uint8_t **data, int32_t *length) {
	if (cpu.loaded())
		return cpu.getMemoryArea(which, data, length);
	else
		return false;
}

uint8_t GB::ExternalRead(uint16_t addr) {
	if (cpu.loaded())
		return cpu.ExternalRead(addr);
	else
		return 0;
}

void GB::ExternalWrite(uint16_t addr, uint8_t val) {
	if (cpu.loaded())
		cpu.ExternalWrite(addr, val);
}

void GB::GetRegs(uint32_t *dest) {
	cpu.GetRegs(dest);
}

void GB::SetInterruptAddresses(int32_t *addrs, size_t numAddrs)
{
	cpu.SetInterruptAddresses(addrs, numAddrs);
}

int32_t GB::GetHitInterruptAddress()
{
	return cpu.GetHitInterruptAddress();
}

uint16_t GB::getDivState() {
	return cpu.getDivState();
}

SYNCFUNC(GB)
{
	SSS(cpu);
}

}
