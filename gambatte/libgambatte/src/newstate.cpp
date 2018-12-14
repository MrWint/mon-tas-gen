#include "newstate.h"
#include <cstring>
#include <algorithm>

namespace gambatte {

NewStateDummy::NewStateDummy()
	:length(0)
{
}
void NewStateDummy::Save(const void *, size_t size, const char *)
{
	length += size;
}
void NewStateDummy::Load(void *, size_t, const char *)
{
}

NewStateExternalBuffer::NewStateExternalBuffer(uint8_t *buffer, std::size_t maxlength)
	:buffer(buffer), length(0), maxlength(maxlength)
{
}

void NewStateExternalBuffer::Save(const void *ptr, size_t size, const char *name)
{
	if (maxlength - length >= size)
	{
		std::memcpy(buffer + length, ptr, size);
	}
	length += size;
}

void NewStateExternalBuffer::Load(void *ptr, size_t size, const char *name)
{
	char *dst = static_cast<char *>(ptr);
	if (maxlength - length >= size)
	{
		std::memcpy(dst, buffer + length, size);
	}
	length += size;
}


}
