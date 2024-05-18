#include <stdio.h>

float bad_add(float v1, float v2)
{
    printf("Idk how to sum %f and %f", v1, v2);
    return v1 + v2 + 10.0;
}