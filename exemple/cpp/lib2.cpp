#include "lib2.h"
#include <math.h>

Vector2::Vector2(double ix , double iy ){
    x = ix;
    y = iy;
}
double Vector2::distanceTo(Vector2 pos){
    return sqrt((pos.y - y) * (pos.y - y) + (pos.x - x) * (pos.x - x));
}
void Vector2::vectorMovement(double plusx, double plusy){
    x += plusx;
    y += plusy;
    return;
}
Vector2 Vector2::midpoint(Vector2 pos){
    double mx = (x + pos.x) / 2;
    double my = (y + pos.y) / 2;
    Vector2 mid(mx, my);
    return mid;
}
double Vector2::percentDistance(Vector2 pos, double percentOfDistance){
    return distanceTo(pos) / (100 / percentOfDistance);
}