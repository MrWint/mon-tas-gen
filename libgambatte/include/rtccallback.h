#ifndef GAMBATTE_RTCCALLBACK_H
#define GAMBATTE_RTCCALLBACK_H

#include <cstdint>

namespace gambatte {
class RtcCallback {
public:
	virtual std::uint32_t getCurrentTime() = 0;
};
}

#endif
