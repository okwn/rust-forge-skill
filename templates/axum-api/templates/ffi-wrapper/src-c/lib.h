#ifndef LIB_H
#define LIB_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Vec3 Vec3;

Vec3* lib_vec3_create(float x, float y, float z);
void lib_vec3_destroy(Vec3* v);

float lib_vec3_get_x(const Vec3* v);
float lib_vec3_get_y(const Vec3* v);
float lib_vec3_get_z(const Vec3* v);

float lib_vec3_dot(const Vec3* a, const Vec3* b);
float lib_vec3_length(const Vec3* v);
void lib_vec3_normalize(Vec3* v);

void lib_vec3_array_sum(const float* values, size_t count, float* out);

#ifdef __cplusplus
}
#endif

#endif
