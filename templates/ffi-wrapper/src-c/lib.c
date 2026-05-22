#include "lib.h"
#include <stdlib.h>
#include <math.h>

void* lib_alloc(size_t size) {
    return malloc(size);
}

void lib_free(void* ptr) {
    free(ptr);
}

LibError lib_vec3_create(double x, double y, double z, Vec3** out) {
    if (out == NULL) {
        return ERROR_INVALID_INPUT;
    }
    Vec3* vec = (Vec3*)malloc(sizeof(Vec3));
    if (vec == NULL) {
        return ERROR_ALLOCATION;
    }
    vec->x = x;
    vec->y = y;
    vec->z = z;
    *out = vec;
    return OK;
}

void lib_vec3_destroy(Vec3* ptr) {
    if (ptr != NULL) {
        free(ptr);
    }
}

double lib_vec3_length(const Vec3* vec) {
    if (vec == NULL) {
        return 0.0;
    }
    return sqrt(vec->x * vec->x + vec->y * vec->y + vec->z * vec->z);
}