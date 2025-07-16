#include "libmysolvers.h"
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

double quadratic_single_neg_solver(double a, double b, double c) {
    if (a == 0) {
        fprintf(stderr, "ERROR: Coefficient 'a' cannot be zero in a quadratic equation, or else it will not be an equation at all.\n");
        exit(EXIT_FAILURE);
    }

    double discriminant = (b * b) - (4 * a * c);
    if (discriminant < 0) {
        fprintf(stderr, "ERROR: The equation has no real roots.\n");
        exit(EXIT_FAILURE);
    }

    double x = (-b - sqrt(discriminant)) / (2 * a);
    return x;
}

double quadratic_single_pos_solver(double a, double b, double c) {
    if (a == 0) {
        fprintf(stderr, "ERROR: Coefficient 'a' cannot be zero in a quadratic equation, or else it will not be an equation at all.\n");
        exit(EXIT_FAILURE);
    }

    double discriminant = (b * b) - (4 * a * c);
    if (discriminant < 0) {
        fprintf(stderr, "ERROR: The equation has no real roots.\n");
        exit(EXIT_FAILURE);
    }

    double x = (-b + sqrt(discriminant)) / (2 * a);
    return x;
}