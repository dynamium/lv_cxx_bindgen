
#if defined(LV_LVGL_H_INCLUDE_SIMPLE)
#include "lvgl.h"
#elif defined(LV_BUILD_TEST)
#include "../lvgl.h"
#else
#include "lvgl/lvgl.h"
#endif


#ifndef LV_ATTRIBUTE_MEM_ALIGN
#define LV_ATTRIBUTE_MEM_ALIGN
#endif

#ifndef LV_ATTRIBUTE_IMG_DUST
#define LV_ATTRIBUTE_IMG_DUST
#endif

static const
LV_ATTRIBUTE_MEM_ALIGN LV_ATTRIBUTE_LARGE_CONST LV_ATTRIBUTE_IMG_DUST
uint8_t test_A2_NONE_align64_map[] = {

    0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x03,0xff,0xff,0xff,0xff,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x00,0x02,0xff,0x80,0x00,0x00,0x00,0x03,0x00,0x00,0x00,0x03,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x00,0x2f,0xff,0xf8,0x00,0x07,0xd0,0x03,0x3f,0xff,0xff,0xf3,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x00,0xbf,0xff,0xfe,0x00,0x0f,0xf0,0x03,0x30,0x00,0x00,0x33,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x02,0xff,0xff,0xff,0x80,0x0f,0xf0,0x03,0x33,0xff,0xff,0x33,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x03,0xff,0xff,0xff,0xc0,0x07,0xd0,0x03,0x33,0x00,0x03,0x33,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x0b,0xff,0xff,0xff,0xe0,0x00,0x00,0x03,0x33,0x3f,0xf3,0x33,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x0f,0xff,0xff,0xff,0xf0,0x00,0x00,0x03,0x33,0x30,0x33,0x33,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x0f,0xff,0xff,0xff,0xf0,0x1b,0xe4,0x03,0x33,0x33,0x33,0x33,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x0f,0xff,0xff,0xff,0xf0,0x7f,0xfd,0x03,0x33,0x30,0x33,0x33,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x0f,0xff,0xff,0xff,0xf0,0xbf,0xfe,0x03,0x33,0x3f,0xf3,0x33,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x0b,0xff,0xff,0xff,0xe0,0xff,0xff,0x03,0x33,0x00,0x03,0x33,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x03,0xff,0xff,0xff,0xc0,0xff,0xff,0x03,0x33,0xff,0xff,0x33,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x02,0xff,0xff,0xff,0x80,0xbf,0xfe,0x03,0x30,0x00,0x00,0x33,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x00,0xbf,0xff,0xfe,0x00,0x7f,0xfd,0x03,0x3f,0xff,0xff,0xf3,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xfc,0x00,0x2f,0xff,0xf8,0x00,0x1b,0xe4,0x03,0x00,0x00,0x00,0x03,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x00,0x00,0x00,0x00,0x00,0x02,0xff,0x80,0x00,0x00,0x00,0x03,0xff,0xff,0xff,0xff,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x00,0x00,0x00,0x00,0x3f,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x0b,0xff,0xf4,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0xbf,0xff,0xfe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0xff,0xff,0xff,0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0xff,0x40,0xff,0xc0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0xfe,0x00,0x7f,0xd0,0x00,0x01,0xbf,0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0xbd,0x00,0x3f,0xe0,0x00,0x1f,0xff,0xf4,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x00,0x00,0x3f,0xe0,0x00,0x1f,0xeb,0xfc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x06,0xff,0xff,0xe0,0x00,0x1f,0x40,0xfe,0x00,0x00,0x00,0x00,0x00,0x33,0x0c,0x0c,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x3f,0xff,0xff,0xe0,0x00,0x1a,0x00,0xfe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0xff,0xfe,0xbf,0xe0,0x00,0x00,0x5a,0xfe,0x00,0x00,0x15,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc1,0xff,0x40,0x3f,0xe0,0x00,0x0b,0xff,0xfe,0x00,0x00,0xff,0xd0,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc2,0xff,0x00,0x3f,0xe0,0x00,0x2f,0xea,0xfe,0x00,0x00,0xd1,0xe0,0x00,0x33,0x0c,0x0c,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0x00,0x3f,0xe0,0x00,0x3f,0x80,0xfe,0x00,0x00,0x16,0xf0,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc2,0xff,0x41,0xff,0xe0,0x00,0x3f,0x40,0xfe,0x00,0x00,0xff,0xf0,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc1,0xff,0xff,0xff,0xfd,0x00,0x3f,0xd7,0xff,0x00,0x01,0xe0,0xf0,0x00,0x33,0x0c,0x0c,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0xff,0xff,0xbf,0xfd,0x00,0x2f,0xff,0xff,0xc0,0x01,0xf6,0xf0,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x2f,0xfe,0x1f,0xfd,0x00,0x0b,0xfd,0x7f,0xc0,0x00,0xbe,0xf8,0x00,0x33,0x0c,0x0c,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x00,0x40,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfe,0xaa,0xa9,0x55,0x50,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfe,0xaa,0xa9,0x55,0x50,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfe,0xaa,0xa9,0x55,0x50,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfe,0xaa,0xa9,0x55,0x50,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfe,0xaa,0xa9,0x55,0x50,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfe,0xaa,0xa9,0x55,0x50,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfe,0xaa,0xa9,0x55,0x50,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfe,0xaa,0xa9,0x55,0x50,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfe,0xaa,0xa9,0x55,0x50,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfe,0xaa,0xa9,0x55,0x50,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfe,0xaa,0xa9,0x55,0x50,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfe,0xaa,0xa9,0x55,0x50,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfe,0xaa,0xa9,0x55,0x50,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc3,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfe,0xaa,0xa9,0x55,0x50,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xc0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xfc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,

};

const lv_img_dsc_t test_A2_NONE_align64 = {
  .header.cf = LV_COLOR_FORMAT_A2,
  .header.flags = 0,
  .header.w = 71,
  .header.h = 60,
  .header.stride = 32,
  .data_size = 1920,
  .data = test_A2_NONE_align64_map,
};

