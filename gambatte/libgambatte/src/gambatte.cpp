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
	unsigned loadflags;
	unsigned layersMask;

	Priv() : loadflags(0), layersMask(LAYER_MASK_BG | LAYER_MASK_OBJ)
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

long GB::runFor(unsigned &samples) {
	if (!p_->cpu.loaded()) {
		samples = 0;
		return -1;
	}
	
	p_->cpu.setSoundBuffer();
	const long cyclesSinceBlit = p_->cpu.runFor(samples * 2);
	samples = p_->cpu.fillSoundBuffer();
	
	return cyclesSinceBlit < 0 ? cyclesSinceBlit : static_cast<long>(samples) - (cyclesSinceBlit >> 1);
}

void GB::setVideoBuffer(uint_least32_t *const videoBuf, const int pitch) {
	p_->cpu.setVideoBuffer(videoBuf, pitch);
}

void GB::setLayers(unsigned mask)
{
	p_->cpu.setLayers(mask);
}

void GB::reset(const std::uint32_t now, const unsigned div) {
	if (p_->cpu.loaded()) {
		
		int length = p_->cpu.saveSavedataLength();
		char *s;
		if (length > 0)
		{
			s = (char *) std::malloc(length);
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

void GB::setInputGetter(unsigned (*getInput)(void *), void* context) {
	p_->cpu.setInputGetter(getInput, context);
}

void GB::setRTCCallback(std::uint32_t (*callback)(void*), void* context) {
	p_->cpu.setRTCCallback(callback, context);
}

int GB::load(const char *romfiledata, unsigned romfilelength, const std::uint32_t now, const unsigned flags, const unsigned div) {
	const int failed = p_->cpu.load(romfiledata, romfilelength, flags & FORCE_DMG, flags & MULTICART_COMPAT);
	
	if (!failed) {
		SaveState state;
		p_->cpu.setStatePtrs(state);
		p_->loadflags = flags;
		setInitState(state, !(flags & FORCE_DMG), flags & GBA_CGB, now, div);
		p_->cpu.loadState(state);
	}
	
	return failed;
}

int GB::loadGBCBios(const char* biosfiledata) {
	memcpy(p_->cpu.cgbBiosBuffer(), biosfiledata, 0x900);
	return 0;
}

int GB::loadDMGBios(const char* biosfiledata) {
	memcpy(p_->cpu.dmgBiosBuffer(), biosfiledata, 0x100);
	return 0;
}

void GB::saveSavedata(char *dest) {
	if (p_->cpu.loaded())
		p_->cpu.saveSavedata(dest);
}
void GB::loadSavedata(const char *data) {
	if (p_->cpu.loaded())
		p_->cpu.loadSavedata(data);
}
int GB::saveSavedataLength() {
	if (p_->cpu.loaded())
		return p_->cpu.saveSavedataLength();
	else
		return -1;
}

bool GB::getMemoryArea(int which, unsigned char **data, int *length) {
	if (p_->cpu.loaded())
		return p_->cpu.getMemoryArea(which, data, length);
	else
		return false;
}

unsigned char GB::ExternalRead(unsigned short addr) {
	if (p_->cpu.loaded())
		return p_->cpu.ExternalRead(addr);
	else
		return 0;
}

void GB::ExternalWrite(unsigned short addr, unsigned char val) {
	if (p_->cpu.loaded())
		p_->cpu.ExternalWrite(addr, val);
}

void GB::GetRegs(int *dest) {
	p_->cpu.GetRegs(dest);
}

void GB::SetInterruptAddresses(int *addrs, int numAddrs)
{
	p_->cpu.SetInterruptAddresses(addrs, numAddrs);
}

int GB::GetHitInterruptAddress()
{
	return p_->cpu.GetHitInterruptAddress();
}

std::uint16_t GB::getDivState() {
	return p_->cpu.getDivState();
}

SYNCFUNC(GB)
{
	SSS(p_->cpu);
}

}
