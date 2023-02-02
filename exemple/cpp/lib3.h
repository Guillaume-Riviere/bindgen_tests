#ifndef VECTOR2_H
#define VECTOR2_H

#include <math.h>

struct Vector2{
    double x;
    double y;
};

Vector2 Vector2_new(double ix, double iy);
double Vector2_distanceTo(Vector2 self, Vector2 pos);
void Vector2_vectorMovement(Vector2 *self, double plusx, double plusy);
Vector2 Vector2_midpoint(Vector2 self, Vector2 pos);
double Vector2_percentDistance(Vector2 self, Vector2 pos, double percentOfDistance);

#endif
