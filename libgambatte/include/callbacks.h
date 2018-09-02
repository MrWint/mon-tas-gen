#ifndef CALLBACKS_H
#define CALLBACKS_H

#include <cstdint>

namespace gambatte {
enum { BG_PALETTE = 0, SP1_PALETTE = 1, SP2_PALETTE = 2 };

typedef void (*MemoryCallback)(int32_t address, int64_t cycleOffset);
typedef void (*CDCallback)(int32_t addr, int32_t addrtype, int32_t flags);

enum eCDLog_AddrType
{
	eCDLog_AddrType_ROM, eCDLog_AddrType_HRAM, eCDLog_AddrType_WRAM, eCDLog_AddrType_CartRAM,
	eCDLog_AddrType_None
};

enum eCDLog_Flags
{
	eCDLog_Flags_ExecFirst = 1,
	eCDLog_Flags_ExecOperand = 2,
	eCDLog_Flags_Data = 4,
};
}

#endif