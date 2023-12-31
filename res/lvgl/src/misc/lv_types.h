/**
 * @file lv_types.h
 *
 */

#ifndef LV_TYPES_H
#define LV_TYPES_H

#ifdef __cplusplus
extern "C" {
#endif

/*********************
 *      INCLUDES
 *********************/
#include "../lv_conf_internal.h"

#ifndef __ASSEMBLY__
#include <stdint.h>
#endif

/*********************
 *      DEFINES
 *********************/

/*If __UINTPTR_MAX__ or UINTPTR_MAX are available, use them to determine arch size*/
#if defined(__UINTPTR_MAX__) && __UINTPTR_MAX__ > 0xFFFFFFFF
#define LV_ARCH_64

#elif defined(UINTPTR_MAX) && UINTPTR_MAX > 0xFFFFFFFF
#define LV_ARCH_64

/*Otherwise use compiler-dependent means to determine arch size*/
#elif defined(_WIN64) || defined(__x86_64__) || defined(__ppc64__) || defined (__aarch64__)
#define LV_ARCH_64

#endif

/**********************
 *      TYPEDEFS
 **********************/

/* Exclude C enum and struct definitions when included by assembly code */
#ifndef __ASSEMBLY__

/**
 * LVGL error codes.
 */
enum _lv_result_t {
    LV_RESULT_INVALID = 0, /*Typically indicates that the object is deleted (become invalid) in the action
                      function or an operation was failed*/
    LV_RESULT_OK,      /*The object is valid (no deleted) after the action*/
};

#ifdef DOXYGEN
typedef _lv_result_t lv_result_t;
#else
typedef uint8_t lv_result_t;
#endif /*DOXYGEN*/

#if defined(__cplusplus) || __STDC_VERSION__ >= 199901L
/*If c99 or newer,  use the definition of uintptr_t directly from <stdint.h>*/
typedef uintptr_t lv_uintptr_t;
typedef intptr_t lv_intptr_t;

#else

/*Otherwise, use the arch size determination*/
#ifdef LV_ARCH_64
typedef uint64_t lv_uintptr_t;
typedef int64_t lv_intptr_t;
#else
typedef uint32_t lv_uintptr_t;
typedef int32_t lv_intptr_t;
#endif

#endif

#if LV_USE_FLOAT
typedef float lv_value_precise_t;
#else
typedef int32_t lv_value_precise_t;
#endif

#endif /*__ASSEMBLY__*/

/**********************
 * GLOBAL PROTOTYPES
 **********************/

/**********************
 *      MACROS
 **********************/

#define LV_UNUSED(x) ((void)x)

#define _LV_CONCAT(x, y) x ## y
#define LV_CONCAT(x, y) _LV_CONCAT(x, y)

#define _LV_CONCAT3(x, y, z) x ## y ## z
#define LV_CONCAT3(x, y, z) _LV_CONCAT3(x, y, z)

#if defined(PYCPARSER) || defined(__CC_ARM)
#define LV_FORMAT_ATTRIBUTE(fmtstr, vararg)
#elif defined(__GNUC__) && ((__GNUC__ == 4 && __GNUC_MINOR__ >= 4) || __GNUC__ > 4)
#define LV_FORMAT_ATTRIBUTE(fmtstr, vararg) __attribute__((format(gnu_printf, fmtstr, vararg)))
#elif (defined(__clang__) || defined(__GNUC__) || defined(__GNUG__))
#define LV_FORMAT_ATTRIBUTE(fmtstr, vararg) __attribute__((format(printf, fmtstr, vararg)))
#else
#define LV_FORMAT_ATTRIBUTE(fmtstr, vararg)
#endif

#ifdef __cplusplus
} /*extern "C"*/
#endif

#endif /*LV_TYPES_H*/
