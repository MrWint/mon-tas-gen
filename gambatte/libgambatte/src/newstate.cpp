#include "newstate.h"
#include <cstring>
#include <algorithm>

namespace gambatte {

NewStateDummy::NewStateDummy()
	:length(0)
{
}
void NewStateDummy::Save(const void *ptr, size_t size, const char *name)
{
	length += size;
}
void NewStateDummy::Load(void *ptr, size_t size, const char *name)
{
}

NewStateExternalBuffer::NewStateExternalBuffer(char *buffer, long maxlength)
	:buffer(buffer), length(0), maxlength(maxlength)
{
}

void NewStateExternalBuffer::Save(const void *ptr, size_t size, const char *name)
{
	if (maxlength - length >= (long)size)
	{
		std::memcpy(buffer + length, ptr, size);
	}
	length += size;
}

void NewStateExternalBuffer::Load(void *ptr, size_t size, const char *name)
{
	char *dst = static_cast<char *>(ptr);
	if (maxlength - length >= (long)size)
	{
		std::memcpy(dst, buffer + length, size);
	}
	length += size;
}


}
