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
#include "cpu.h"
#include "savestate.h"
#include "initstate.h"
#include <sstream>
#include <cstring>

namespace gambatte {
struct GB::Priv {
	CPU cpu;
	uint8_t loadflags;

	Priv() : loadflags(0)
	{
	}

	~Priv()
	{
	}
};
	
GB::GB() : p_(new Priv) {}

GB::~GB() {
	delete p_;
}

int32_t GB::runFor(uint32_t &samples) {
	if (!p_->cpu.loaded()) {
		samples = 0;
		return -1;
	}
	
	p_->cpu.setSoundBuffer();
	const int32_t cyclesSinceBlit = p_->cpu.runFor(samples * 2);
	samples = p_->cpu.fillSoundBuffer();
	
	return cyclesSinceBlit < 0 ? cyclesSinceBlit : static_cast<int32_t>(samples) - (cyclesSinceBlit >> 1);
}

void GB::setVideoBuffer(uint32_t *const videoBuf, const std::size_t pitch) {
	p_->cpu.setVideoBuffer(videoBuf, pitch);
}

void GB::setLayers(uint8_t mask)
{
	p_->cpu.setLayers(mask);
}

void GB::reset(const uint32_t now, const uint32_t div) {
	if (p_->cpu.loaded()) {
		
		size_t length = p_->cpu.saveSavedataLength();
		uint8_t *s;
		if (length > 0)
		{
			s = (uint8_t *) std::malloc(length);
			p_->cpu.saveSavedata(s);
		}
		
		SaveState state;
		p_->cpu.setStatePtrs(state);
		setInitState(state, !(p_->loadflags & FORCE_DMG), p_->loadflags & GBA_CGB, now, div);
		p_->cpu.loadState(state);
		if (length > 0)
		{
			p_->cpu.loadSavedata(s);
			std::free(s);
		}
	}
}

void GB::setInputGetter(uint8_t (*getInput)(void *), void* context) {
	p_->cpu.setInputGetter(getInput, context);
}

void GB::setRTCCallback(uint32_t (*callback)(void*), void* context) {
	p_->cpu.setRTCCallback(callback, context);
}

int32_t GB::load(const uint8_t *romfiledata, uint32_t romfilelength, const uint32_t now, const uint8_t flags, const uint32_t div) {
	const int32_t failed = p_->cpu.load(romfiledata, romfilelength, flags & FORCE_DMG, flags & MULTICART_COMPAT);
	
	if (!failed) {
		SaveState state;
		p_->cpu.setStatePtrs(state);
		p_->loadflags = flags;
		setInitState(state, !(flags & FORCE_DMG), flags & GBA_CGB, now, div);
		p_->cpu.loadState(state);
	}
	
	return failed;
}

void GB::loadGBCBios(const uint8_t* biosfiledata) {
	memcpy(p_->cpu.cgbBiosBuffer(), biosfiledata, 0x900);
}

void GB::loadDMGBios(const uint8_t* biosfiledata) {
	memcpy(p_->cpu.dmgBiosBuffer(), biosfiledata, 0x100);
}

void GB::saveSavedata(uint8_t *dest) {
	if (p_->cpu.loaded())
		p_->cpu.saveSavedata(dest);
}
void GB::loadSavedata(const uint8_t *data) {
	if (p_->cpu.loaded())
		p_->cpu.loadSavedata(data);
}
size_t GB::saveSavedataLength() {
	if (p_->cpu.loaded())
		return p_->cpu.saveSavedataLength();
	else
		return 0;
}

bool GB::getMemoryArea(int32_t which, uint8_t **data, int32_t *length) {
	if (p_->cpu.loaded())
		return p_->cpu.getMemoryArea(which, data, length);
	else
		return false;
}

uint8_t GB::ExternalRead(uint16_t addr) {
	if (p_->cpu.loaded())
		return p_->cpu.ExternalRead(addr);
	else
		return 0;
}

void GB::ExternalWrite(uint16_t addr, uint8_t val) {
	if (p_->cpu.loaded())
		p_->cpu.ExternalWrite(addr, val);
}

void GB::GetRegs(uint32_t *dest) {
	p_->cpu.GetRegs(dest);
}

void GB::SetInterruptAddresses(int32_t *addrs, uint32_t numAddrs)
{
	p_->cpu.SetInterruptAddresses(addrs, numAddrs);
}

int32_t GB::GetHitInterruptAddress()
{
	return p_->cpu.GetHitInterruptAddress();
}

uint16_t GB::getDivState() {
	return p_->cpu.getDivState();
}

SYNCFUNC(GB)
{
	SSS(p_->cpu);
}

}
