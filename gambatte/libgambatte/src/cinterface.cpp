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

#include "cinterface.h"
#include "gambatte.h"
#include <cstdlib>
#include <cstring>
#include "newstate.h"

// new is actually called in a few different places, so replace all of them for determinism guarantees
void *operator new(std::size_t n) {
	void *p = std::malloc(n);
	std::memset(p, 0, n);
	return p;
}

void operator delete(void *p) {
	std::free(p);
}

namespace {

using namespace gambatte;

	GBEXPORT GB * gambatte_create() {
	return new GB();
}

GBEXPORT void gambatte_destroy(GB *g) {
	delete g;
}

GBEXPORT int gambatte_load(GB *g, char const *romfiledata, unsigned romfilelength, unsigned flags) {
	return g->load(romfiledata, romfilelength, flags);
}

GBEXPORT int gambatte_loadbios(GB *g, char const *biosfiledata, unsigned size) {
	return g->loadBios(biosfiledata, size);
}

GBEXPORT int gambatte_runfor(GB *g, unsigned *samples, bool startsOnFrameBoundaries) {
	std::size_t sampv = *samples;
	int ret = g->runFor(sampv, startsOnFrameBoundaries);
	*samples = sampv;
	return ret;
}

GBEXPORT void gambatte_setvideobuffer(GB *g, uint_least32_t *const videoBuf, const int pitch) {
	g->setVideoBuffer(videoBuf, pitch);
}

GBEXPORT void gambatte_setlayers(GB *g, unsigned mask) {
	g->setLayers(mask);
}

GBEXPORT void gambatte_setTimeMode(GB *g, bool useCycles) {
	g->setTimeMode(useCycles);
}

GBEXPORT void gambatte_reset(GB *g) {
	g->reset();
}

GBEXPORT void gambatte_setdmgpalettecolor(GB *g, unsigned palnum, unsigned colornum, unsigned rgb32) {
	g->setDmgPaletteColor(palnum, colornum, rgb32);
}

GBEXPORT void gambatte_setcgbpalette(GB *g, unsigned *lut) {
	g->setCgbPalette(lut);
}

GBEXPORT void gambatte_setinputgetter(GB *g, unsigned (*getinput)(void *), void *context) {
	g->setInputGetter(getinput, context);
}

GBEXPORT int gambatte_iscgb(GB *g) {
	return g->isCgb();
}

GBEXPORT int gambatte_isloaded(GB *g) {
	return g->isLoaded();
}

GBEXPORT void gambatte_savesavedata(GB *g, char *dest) {
	g->saveSavedata(dest);
}

GBEXPORT void gambatte_loadsavedata(GB *g, char const *data) {
	g->loadSavedata(data);
}

GBEXPORT int gambatte_savesavedatalength(GB *g) {
	return g->saveSavedataLength();
}

GBEXPORT int gambatte_newstatelen(GB *g) {
	NewStateDummy dummy;
	g->SyncState<false>(&dummy);
	return dummy.GetLength();
}

GBEXPORT int gambatte_newstatesave(GB *g, char *data, int len) {
	NewStateExternalBuffer saver(data, len);
	g->SyncState<false>(&saver);
	return !saver.Overflow() && saver.GetLength() == len;
}

GBEXPORT int gambatte_newstateload(GB *g, char const *data, int len) {
	NewStateExternalBuffer loader((char *)data, len);
	g->SyncState<true>(&loader);
	return !loader.Overflow() && loader.GetLength() == len;
}

GBEXPORT void gambatte_newstatesave_ex(GB *g, FPtrs *ff) {
	NewStateExternalFunctions saver(ff);
	g->SyncState<false>(&saver);
}

GBEXPORT void gambatte_newstateload_ex(GB *g, FPtrs *ff) {
	NewStateExternalFunctions loader(ff);
	g->SyncState<true>(&loader);
}

GBEXPORT void gambatte_romtitle(GB *g, char *dest) {
	std::strcpy(dest, g->romTitle().c_str());
}

GBEXPORT int gambatte_getmemoryarea(GB *g, int which, unsigned char **data, int *length) {
	return g->getMemoryArea(which, data, length);
}

GBEXPORT unsigned char gambatte_cpuread(GB *g, unsigned short addr) {
	return g->externalRead(addr);
}

GBEXPORT void gambatte_cpuwrite(GB *g, unsigned short addr, unsigned char val) {
	g->externalWrite(addr, val);
}

GBEXPORT int gambatte_linkstatus(GB *g, int which)
{
	return g->linkStatus(which);
}

GBEXPORT void gambatte_getregs(GB *g, int *dest) {
	g->getRegs(dest);
}

GBEXPORT void gambatte_setinterruptaddresses(GB *g, int *addrs, int numAddrs) {
	g->setInterruptAddresses(addrs, numAddrs);
}
GBEXPORT void gambatte_clearinterruptaddresses(GB *g)
{
	g->setInterruptAddresses(0, 0);
}
GBEXPORT int gambatte_gethitinterruptaddress(GB *g) {
	return g->getHitInterruptAddress();
}
GBEXPORT uint16_t gambatte_getdivstate(GB *g) {
  return g->getDivState();
}

}
