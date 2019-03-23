#include "cinterface.h"
#include "gambatte.h"
#include <cstdlib>
#include <cstring>
#include "newstate.h"

using namespace gambatte;

// new is actually called in a few different places, so replace all of them for determinism guarantees
void *operator new(std::size_t n)
{
	void *p = std::malloc(n);
	std::memset(p, 0, n);
	return p;
}

void operator delete(void *p)
{
	std::free(p);
}
void operator delete(void *p, std::size_t) { std::free(p); } // fixes warning.

GBEXPORT GB *gambatte_create()
{
	return new GB();
}

GBEXPORT void gambatte_destroy(GB *g)
{
	delete g;
}

GBEXPORT int gambatte_load(GB *g, const char *romfiledata, unsigned romfilelength, long long now, unsigned flags, unsigned div)
{
	int ret = g->load(romfiledata, romfilelength, now, flags, div);
	return ret;
}

GBEXPORT int gambatte_loadgbcbios(GB* g, const char* biosfiledata)
{
	int ret = g->loadGBCBios(biosfiledata);
	return ret;
}

GBEXPORT int gambatte_loaddmgbios(GB* g, const char* biosfiledata)
{
	int ret = g->loadDMGBios(biosfiledata);
	return ret;
}

GBEXPORT int gambatte_runfor(GB *g, unsigned *samples, bool startsOnFrameBoundaries)
{
	uint32_t sampv = *samples;
	int32_t ret = g->runFor(sampv, startsOnFrameBoundaries);
	*samples = sampv;
	return ret;
}

GBEXPORT void gambatte_setvideobuffer(GB *g, uint_least32_t *const videoBuf, const int pitch) {
	g->setVideoBuffer(videoBuf, pitch);
}

GBEXPORT void gambatte_setlayers(GB *g, unsigned mask)
{
	g->setLayers(mask);
}

GBEXPORT void gambatte_reset(GB *g, long long now, unsigned div)
{
	g->reset(now, div);
}

GBEXPORT void gambatte_setdmgpalettecolor(GB *g, unsigned palnum, unsigned colornum, unsigned rgb32)
{
	g->setDmgPaletteColor(palnum, colornum, rgb32);
}

GBEXPORT void gambatte_setcgbpalette(GB *g, unsigned *lut)
{
	g->setCgbPalette(lut);
}

GBEXPORT void gambatte_setinputgetter(GB *g, unsigned (*getInput)(void *), void* context)
{
	g->setInputGetter(getInput, context);
}

GBEXPORT void gambatte_setrtccallback(GB *g, unsigned int (*callback)(void*), void* context)
{
	g->setRTCCallback(callback, context);
}

GBEXPORT int gambatte_iscgb(GB *g)
{
	return g->isCgb();
}

GBEXPORT int gambatte_isloaded(GB *g)
{
	return g->isLoaded();
}

GBEXPORT void gambatte_savesavedata(GB *g, char *dest)
{
	g->saveSavedata(dest);
}

GBEXPORT void gambatte_loadsavedata(GB *g, const char *data)
{
	g->loadSavedata(data);
}

GBEXPORT int gambatte_savesavedatalength(GB *g)
{
	return g->saveSavedataLength();
}

GBEXPORT int gambatte_newstatelen(GB *g)
{
	NewStateDummy dummy;
	g->SyncState<false>(&dummy);
	return dummy.GetLength();
}

GBEXPORT int gambatte_newstatesave(GB *g, char *data, int len)
{
	NewStateExternalBuffer saver(data, len);
	g->SyncState<false>(&saver);
	return !saver.Overflow() && saver.GetLength() == len;
}

GBEXPORT int gambatte_newstateload(GB *g, const char *data, int len)
{
	NewStateExternalBuffer loader((char *)data, len);
	g->SyncState<true>(&loader);
	return !loader.Overflow() && loader.GetLength() == len;
}

GBEXPORT void gambatte_newstatesave_ex(GB *g, FPtrs *ff)
{
	NewStateExternalFunctions saver(ff);
	g->SyncState<false>(&saver);
}

GBEXPORT void gambatte_newstateload_ex(GB *g, FPtrs *ff)
{
	NewStateExternalFunctions loader(ff);
	g->SyncState<true>(&loader);
}

GBEXPORT void gambatte_romtitle(GB *g, char *dest)
{
	std::strcpy(dest, g->romTitle().c_str());
}

GBEXPORT int gambatte_getmemoryarea(GB *g, int which, unsigned char **data, int *length)
{
	return g->getMemoryArea(which, data, length);
}

GBEXPORT unsigned char gambatte_cpuread(GB *g, unsigned short addr)
{
	return g->ExternalRead(addr);
}

GBEXPORT void gambatte_cpuwrite(GB *g, unsigned short addr, unsigned char val)
{
	g->ExternalWrite(addr, val);
}

GBEXPORT int gambatte_linkstatus(GB *g, int which)
{
	return g->LinkStatus(which);
}

GBEXPORT void gambatte_getregs(GB *g, int *dest)
{
	g->GetRegs(dest);
}

GBEXPORT void gambatte_setinterruptaddresses(GB *g, int *addrs, int numAddrs)
{
	g->SetInterruptAddresses(addrs, numAddrs);
}
GBEXPORT void gambatte_clearinterruptaddresses(GB *g)
{
	g->SetInterruptAddresses(0, 0);
}
GBEXPORT int gambatte_gethitinterruptaddress(GB *g)
{
	return g->GetHitInterruptAddress();
}

GBEXPORT uint16_t gambatte_getdivstate(GB *g) {
  return g->getDivState();
}