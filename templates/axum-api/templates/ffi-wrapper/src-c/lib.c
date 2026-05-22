#include "lib.h"
#include <stdlib.h>
#include <math.h>

struct Vec3 {
    float x;
    float y;
    float z;
};

Vec3* lib_vec3_create(float x, float y, float z) {
    Vec3* v = (Vec3*)malloc(sizeof(Vec3));
    if (v) {
        v->x = x;
        v->y = y;
        v->z = z;
    }
    return v;
}

void lib_vec3_destroy(Vec3* v) {
    free(v);
}

float lib_vec3_get_x(const Vec3* v) { return v->x; }
float lib_vec3_get_y(const Vec3* v) { return v->y; }
float lib_vec3_get_z(const Vec3* v) { return v->z; }

float lib_vec3_dot(const Vec3* a, const Vec3* b) {
    return a->x * b->x + a->y * b->y + a->z * b->z;
}

float lib_vec3_length(const Vec3* v) {
    return sqrtf(v->x * v->x + v->y * v->y + v->z * v->z);
}

void lib_vec3_normalize(Vec3* v) {
    float len = lib_vec3_length(v);
    if (len > 0.0001f) {
        v->x /= len;
        v->y /= len;
        v->z /= len;
    }
}

void lib_vec3_array_sum(const float* values, size_t count, float* out) {
    float sum = 0.0f;
    for (size_t i = 0; i < count; i++) {
        sum += values[i];
    }
    *out = sum;
}
