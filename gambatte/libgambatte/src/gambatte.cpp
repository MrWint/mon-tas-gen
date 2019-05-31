//
//   Copyright (C) 2007 by sinamas <sinamas at users.sourceforge.net>
//
//   This program is free software; you can redistribute it and/or modify
//   it under the terms of the GNU General Public License version 2 as
//   published by the Free Software Foundation.
//
//   This program is distributed in the hope that it will be useful,
//   but WITHOUT ANY WARRANTY; without even the implied warranty of
//   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//   GNU General Public License version 2 for more details.
//
//   You should have received a copy of the GNU General Public License
//   version 2 along with this program; if not, write to the
//   Free Software Foundation, Inc.,
//   51 Franklin St, Fifth Floor, Boston, MA  02110-1301, USA.
//

#include "gambatte.h"
#include "cpu.h"
#include "initstate.h"
#include "savestate.h"
#include <cstring>
#include <sstream>

namespace gambatte {

struct GB::Priv {
	CPU cpu;
	unsigned loadflags;
	unsigned layersMask;

	Priv() : loadflags(0), layersMask(layer_mask_bg | layer_mask_window | layer_mask_obj)
	{
	}
};

GB::GB() : p_(new Priv) {}

GB::~GB() {
	delete p_;
}

std::ptrdiff_t GB::runFor(std::size_t &samples, bool startsOnFrameBoundaries) {
	if (!p_->cpu.loaded()) {
		samples = 0;
		return -1;
	}

	p_->cpu.setSoundBuffer();

	long const cyclesSinceBlit = p_->cpu.runFor(samples * 2, startsOnFrameBoundaries);
	samples = p_->cpu.fillSoundBuffer();
	return cyclesSinceBlit >= 0
	     ? static_cast<std::ptrdiff_t>(samples) - (cyclesSinceBlit >> 1)
	     : cyclesSinceBlit;
}

void GB::setLayers(unsigned mask) {
	p_->cpu.setLayers(mask);
}

void GB::setVideoBuffer(uint_least32_t *const videoBuf, int const pitch) {
	p_->cpu.setVideoBuffer(videoBuf, pitch);
}

void GB::reset() {
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
		setInitState(state, !(p_->loadflags & FORCE_DMG), p_->loadflags & GBA_CGB);
		p_->cpu.loadState(state);
		if (length > 0)
		{
			p_->cpu.loadSavedata(s);
			std::free(s);
		}
	}
}

void GB::setInputGetter(unsigned (*getInput)(void *), void *context) {
	p_->cpu.setInputGetter(getInput, context);
}

void GB::setTimeMode(bool useCycles) {
	p_->cpu.setTimeMode(useCycles);
}

LoadRes GB::load(char const *romfiledata, unsigned romfilelength, unsigned const flags) {
	LoadRes const loadres = p_->cpu.load(romfiledata, romfilelength, flags & FORCE_DMG, flags & MULTICART_COMPAT);

	if (loadres == LOADRES_OK) {
		SaveState state;
		p_->cpu.setStatePtrs(state);
		p_->loadflags = flags;
		setInitState(state, !(flags & FORCE_DMG), flags & GBA_CGB);
		p_->cpu.loadState(state);
	}

	return loadres;
}

int GB::loadBios(char const* biosfiledata, std::size_t size) {
	p_->cpu.setBios(biosfiledata, size);
	return 0;
}

bool GB::isCgb() const {
	return p_->cpu.isCgb();
}

bool GB::isLoaded() const {
	return p_->cpu.loaded();
}

void GB::saveSavedata(char *dest) {
	if (p_->cpu.loaded())
		p_->cpu.saveSavedata(dest);
}
void GB::loadSavedata(char const *data) {
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

unsigned char GB::externalRead(unsigned short addr) {
	if (p_->cpu.loaded())
		return p_->cpu.externalRead(addr);
	else
		return 0;
}

void GB::externalWrite(unsigned short addr, unsigned char val) {
	if (p_->cpu.loaded())
		p_->cpu.externalWrite(addr, val);
}


void GB::setDmgPaletteColor(int palNum, int colorNum, unsigned long rgb32) {
	p_->cpu.setDmgPaletteColor(palNum, colorNum, rgb32);
}

void GB::setCgbPalette(unsigned *lut) {
	p_->cpu.setCgbPalette(lut);
}

std::string const GB::romTitle() const {
	if (p_->cpu.loaded()) {
		char title[0x11];
		std::memcpy(title, p_->cpu.romTitle(), 0x10);
		title[title[0xF] & 0x80 ? 0xF : 0x10] = '\0';
		return std::string(title);
	}

	return std::string();
}

int GB::linkStatus(int which) {
	return p_->cpu.linkStatus(which);
}

void GB::getRegs(int *dest) {
	p_->cpu.getRegs(dest);
}

void GB::setInterruptAddresses(int *addrs, int numAddrs) {
	p_->cpu.setInterruptAddresses(addrs, numAddrs);
}

int GB::getHitInterruptAddress() {
	return p_->cpu.getHitInterruptAddress();
}

uint16_t GB::getDivState() {
	return p_->cpu.getDivState();
}

SYNCFUNC(GB)
{
	SSS(p_->cpu);
	NSS(p_->loadflags);
}

}
