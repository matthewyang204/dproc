// Copyright 2025 Matthew Yang
// This file is part of libmysolvers.
// libmysolvers is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

#include "libmysolvers.h"
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

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
 // This uses cramer's rule to solve a system of linear equations
 // This can only solve a system of two linear equations with two variables
double linear_solver(int val, double a1, double b1, double c1, double a2, double b2, double c2) {
    double x;
    double y;

    double d = a1 * b2 - a2 * b1;
    x = (c1 * b2 - c2 * b1) / d;
    y = (a1 * c2 - a2 * c1) / d;

    if (d == 0) {
        if (x == 0 && y == 0) {
            fprintf(stderr, "ERROR: The system has infinitely many solutions.\n");
            exit(EXIT_FAILURE);
        } else {
            fprintf(stderr, "ERROR: The system has no solution.\n");
            exit(EXIT_FAILURE);
        }
    } else {
        if (val == 0) {
            return x;
        } else if (val == 1) {
            return y;
        } else {
            fprintf(stderr, "ERROR: Invalid variable name. Use 0 to get x or 1 to get y.\n");
            exit(EXIT_FAILURE);
        }
    }
}