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
void operator delete(void *p, std::size_t)
{
	std::free(p);
}

GBEXPORT GB *gambatte_create()
{
	return new GB();
}

GBEXPORT void gambatte_destroy(GB *g)
{
	delete g;
}

GBEXPORT int gambatte_load(GB *g, const char *romfiledata, unsigned romfilelength)
{
	int ret = g->load(romfiledata, romfilelength, 0 /* now */, GB::GBA_CGB, 0 /* div */);
	g->setLayers(0x7);
	return ret;
}

GBEXPORT int gambatte_loadgbcbios(GB* g, const char* biosfiledata)
{
	int ret = g->loadGBCBios(biosfiledata);
	return ret;
}

GBEXPORT int gambatte_runfor(GB *g, unsigned *samples)
{
	unsigned sampv = *samples;
	int ret = g->runFor(sampv);
	*samples = sampv;
	return ret;
}

GBEXPORT void gambatte_setvideobuffer(GB *g, std::uint32_t *const videoBuf, const int pitch) {
	g->setVideoBuffer(videoBuf, pitch);
}

GBEXPORT void gambatte_reset(GB *g, std::uint32_t now)
{
	g->reset(now, 0 /* div */);
}

GBEXPORT void gambatte_setinputgetter(GB *g, unsigned (*getInput)(void *), void* context)
{
	g->setInputGetter(getInput, context);
}

GBEXPORT void gambatte_setrtccallback(GB *g, unsigned int (*callback)(void*), void* context)
{
	g->setRTCCallback(callback, context);
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
	return saver.GetLength();
}

GBEXPORT int gambatte_newstateload(GB *g, const char *data, int len)
{
	NewStateExternalBuffer loader((char *)data, len);
	g->SyncState<true>(&loader);
	return loader.GetLength();
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

GBEXPORT void gambatte_getregs(GB *g, int *dest)
{
	g->GetRegs(dest);
}

GBEXPORT void gambatte_setinterruptaddresses(GB *g, int *addrs, int numAddrs)
{
	g->SetInterruptAddresses(addrs, numAddrs);
}

GBEXPORT int gambatte_gethitinterruptaddress(GB *g)
{
	return g->GetHitInterruptAddress();
}

GBEXPORT std::uint16_t gambatte_getdivstate(GB *g) {
  return g->getDivState();
}