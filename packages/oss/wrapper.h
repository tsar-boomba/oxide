#include "../../build/sysroot/usr/include/linux/soundcard.h"
#include <stddef.h>

#define MACRO_TO_CONST(M) const unsigned int Fix753_##M = (M);

MACRO_TO_CONST(SNDCTL_DSP_RESET)
MACRO_TO_CONST(SNDCTL_DSP_SYNC)
MACRO_TO_CONST(SNDCTL_DSP_SPEED)
MACRO_TO_CONST(SNDCTL_DSP_STEREO)
MACRO_TO_CONST(SNDCTL_DSP_GETBLKSIZE)
MACRO_TO_CONST(SNDCTL_DSP_SAMPLESIZE)
MACRO_TO_CONST(SNDCTL_DSP_CHANNELS)
MACRO_TO_CONST(SNDCTL_DSP_GETOSPACE)
MACRO_TO_CONST(SNDCTL_DSP_GETISPACE)
MACRO_TO_CONST(SNDCTL_DSP_NONBLOCK)
MACRO_TO_CONST(SNDCTL_DSP_GETCAPS)
MACRO_TO_CONST(SNDCTL_DSP_GETTRIGGER)
MACRO_TO_CONST(SNDCTL_DSP_SETTRIGGER)
MACRO_TO_CONST(SNDCTL_DSP_POST)
MACRO_TO_CONST(SNDCTL_DSP_SUBDIVIDE)
MACRO_TO_CONST(SNDCTL_DSP_SETFRAGMENT)
MACRO_TO_CONST(SNDCTL_DSP_GETFMTS)
MACRO_TO_CONST(SNDCTL_DSP_SETFMT)
MACRO_TO_CONST(SNDCTL_DSP_GETIPTR)
MACRO_TO_CONST(SNDCTL_DSP_GETOPTR)
MACRO_TO_CONST(SNDCTL_DSP_MAPINBUF)
MACRO_TO_CONST(SNDCTL_DSP_MAPOUTBUF)
MACRO_TO_CONST(SNDCTL_DSP_SETSYNCRO)
MACRO_TO_CONST(SNDCTL_DSP_SETDUPLEX)
MACRO_TO_CONST(SNDCTL_DSP_GETODELAY)
MACRO_TO_CONST(SNDCTL_DSP_GETCHANNELMASK)
MACRO_TO_CONST(SNDCTL_DSP_BIND_CHANNEL)
MACRO_TO_CONST(SNDCTL_DSP_SETSPDIF)
MACRO_TO_CONST(SNDCTL_DSP_GETSPDIF)
MACRO_TO_CONST(SNDCTL_DSP_PROFILE)