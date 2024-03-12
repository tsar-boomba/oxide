#include "../../build/sysroot/usr/include/linux/soundcard.h"
#include <stddef.h>

const size_t Fix753_SNDCTL_DSP_RESET = ((((0U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((0)) << 0) | ((0) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_SYNC = ((((0U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((1)) << 0) | ((0) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_SPEED = ((((2U|1U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((2)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_STEREO = ((((2U|1U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((3)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_GETBLKSIZE = ((((2U|1U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((4)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_SAMPLESIZE = ((((2U|1U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((5)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_CHANNELS = ((((2U|1U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((6)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_GETOSPACE = ((((2U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((12)) << 0) | ((((sizeof(audio_buf_info)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_GETISPACE = ((((2U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((13)) << 0) | ((((sizeof(audio_buf_info)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_NONBLOCK = ((((0U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((14)) << 0) | ((0) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_GETCAPS = ((((2U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((15)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_GETTRIGGER = ((((2U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((16)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_SETTRIGGER = ((((1U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((16)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_POST = ((((0U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((8)) << 0) | ((0) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_SUBDIVIDE = ((((2U|1U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((9)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_SETFRAGMENT = ((((2U|1U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((10)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_GETFMTS = ((((2U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((11)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_SETFMT = ((((2U|1U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((5)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_GETIPTR = ((((2U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((17)) << 0) | ((((sizeof(count_info)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_GETOPTR = ((((2U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((18)) << 0) | ((((sizeof(count_info)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_MAPINBUF = ((((2U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((19)) << 0) | ((((sizeof(buffmem_desc)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_MAPOUTBUF = ((((2U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((20)) << 0) | ((((sizeof(buffmem_desc)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_SETSYNCRO = ((((0U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((21)) << 0) | ((0) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_SETDUPLEX = ((((0U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((22)) << 0) | ((0) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_GETODELAY = ((((2U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((23)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_GETCHANNELMASK = ((((2U|1U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((64)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_BIND_CHANNEL = ((((2U|1U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((65)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_SETSPDIF = ((((1U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((66)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_GETSPDIF = ((((2U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((67)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));
const size_t Fix753_SNDCTL_DSP_PROFILE = ((((1U) << (((0 +8)+8)+14)) | ((('P')) << (0 +8)) | (((23)) << 0) | ((((sizeof(int)))) << ((0 +8)+8))));

