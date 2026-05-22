#ifndef LIB_H
#define LIB_H

#include <stdint.h>
#include <stddef.h>

typedef struct {
    double x;
    double y;
    double z;
} Vec3;

typedef enum {
    OK = 0,
    ERROR_INVALID_INPUT = 1,
    ERROR_ALLOCATION = 2,
    ERROR_NOT_FOUND = 3,
} LibError;

void* lib_alloc(size_t size);
void  lib_free(void* ptr);

LibError lib_vec3_create(double x, double y, double z, Vec3** out);
void     lib_vec3_destroy(Vec3* ptr);
double   lib_vec3_length(const Vec3* vec);

#endif // LIB_H