#include "cinterface.h"
#include "gambatte.h"
#include <cstdlib>
#include <cstring>
#include "newstate.h"

using namespace gambatte;

GBEXPORT GB *gambatte_create(const uint8_t* biosfiledata, const uint8_t *romfiledata, size_t romfilelength)
{
	GB* g = new GB();
	g->loadGBCBios(biosfiledata);
	g->load(romfiledata, romfilelength, 0 /* now */, GB::GBA_CGB, 0 /* div */);
	g->setLayers(0x7);
	return g;
}

GBEXPORT void gambatte_destroy(GB *g)
{
	delete g;
}

GBEXPORT int32_t gambatte_runfor(GB *g, uint32_t *samples)
{
	uint32_t sampv = *samples;
	int32_t ret = g->runFor(sampv);
	*samples = sampv;
	return ret;
}

GBEXPORT void gambatte_setvideobuffer(GB *g, uint32_t *const videoBuf, const size_t pitch) {
	g->setVideoBuffer(videoBuf, pitch);
}

GBEXPORT void gambatte_reset(GB *g, uint32_t now)
{
	g->reset(now, 0 /* div */);
}

GBEXPORT void gambatte_setinputgetter(GB *g, uint8_t (*getInput)(void *), void* context)
{
	g->setInputGetter(getInput, context);
}

GBEXPORT void gambatte_setrtccallback(GB *g, uint32_t (*callback)(void*), void* context)
{
	g->setRTCCallback(callback, context);
}

GBEXPORT size_t gambatte_newstatelen(GB *g)
{
	NewStateDummy dummy;
	g->SyncState<false>(&dummy);
	return dummy.GetLength();
}

GBEXPORT size_t gambatte_newstatesave(GB *g, uint8_t *data, size_t len)
{
	NewStateExternalBuffer saver(data, len);
	g->SyncState<false>(&saver);
	return saver.GetLength();
}

GBEXPORT size_t gambatte_newstateload(GB *g, const uint8_t *data, size_t len)
{
	NewStateExternalBuffer loader((uint8_t*)data, len);
	g->SyncState<true>(&loader);
	return loader.GetLength();
}

GBEXPORT int32_t gambatte_getmemoryarea(GB *g, int32_t which, uint8_t **data, int32_t *length)
{
	return g->getMemoryArea(which, data, length);
}

GBEXPORT uint8_t gambatte_cpuread(GB *g, uint16_t addr)
{
	return g->ExternalRead(addr);
}

GBEXPORT void gambatte_cpuwrite(GB *g, uint16_t addr, uint8_t val)
{
	g->ExternalWrite(addr, val);
}

GBEXPORT void gambatte_getregs(GB *g, uint32_t *dest)
{
	g->GetRegs(dest);
}

GBEXPORT void gambatte_setinterruptaddresses(GB *g, int32_t *addrs, size_t numAddrs)
{
	g->SetInterruptAddresses(addrs, numAddrs);
}
GBEXPORT void gambatte_clearinterruptaddresses(GB *g)
{
	g->SetInterruptAddresses(0, 0);
}
GBEXPORT int32_t gambatte_gethitinterruptaddress(GB *g)
{
	return g->GetHitInterruptAddress();
}

GBEXPORT uint16_t gambatte_getdivstate(GB *g) {
  return g->getDivState();
}