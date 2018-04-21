CXX = g++
RM = rm
CP = cp

CXXFLAGS = -Wall -Iinclude -Isrc -O3 -std=c++11 -fno-exceptions -flto $(shell sdl-config --cflags)
TARGET = libgambatte.dll
LDFLAGS = -shared $(shell sdl-config --libs)

DEST = ../output/dll

SRCS = \
	src/common/videolink/rgb32conv.cpp \
	src/cpu.cpp \
	src/gambatte.cpp \
	src/initstate.cpp \
	src/interrupter.cpp \
	src/interruptrequester.cpp \
	src/memory.cpp \
	src/mem/cartridge.cpp \
	src/mem/memptrs.cpp \
	src/mem/rtc.cpp \
	src/sdl/blitterwrapper.cpp \
	src/sdl/gambatte_sdl.cpp \
	src/sdl/sdlblitter.cpp \
	src/newstate.cpp \
	src/sound.cpp \
	src/sound/channel1.cpp \
	src/sound/channel2.cpp \
	src/sound/channel3.cpp \
	src/sound/channel4.cpp \
	src/sound/duty_unit.cpp \
	src/sound/envelope_unit.cpp \
	src/sound/length_counter.cpp \
	src/tima.cpp \
	src/video.cpp \
	src/video/lyc_irq.cpp \
	src/video/ly_counter.cpp \
	src/video/next_m0_time.cpp \
	src/video/ppu.cpp \
	src/video/sprite_mapper.cpp

OBJS = $(SRCS:.cpp=.o)

all: $(TARGET)

%.o: %.cpp
	$(CXX) -c -o $@ $< $(CXXFLAGS)

$(TARGET) : $(OBJS)
	$(CXX) -o $@ $(OBJS) $(LDFLAGS)

clean:
	$(RM) $(OBJS)
	$(RM) $(TARGET)
	
install:
	$(CP) $(TARGET) $(DEST)
