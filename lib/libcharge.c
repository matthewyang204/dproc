// Copyright 2025 Matthew Yang
// This file is part of libcharge, which is itself a part of libmysolvers.
// libmysolvers is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

#include "libcharge.h"
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

double get_volt(double I, double R){
    double volt = I * R;
    return volt;
}

double get_R(double V, double I){
    double resistance = V / I;
    return resistance;
}

double get_I(double V, double R){
    double current = V / R;
    return current;
}

double get_watt(double V, double IR, char* IorR){
    if (strcmp(IorR, "I") == 0){
        double watt = V * IR;
        return watt;
    }
    else if (strcmp(IorR, "R") == 0){
        double I = get_I(V, IR);
        double watt = V * I;
        return watt;
    }
    else {
        fprintf(stderr, "Error: Invalid parameter for IorR. Use 'I' for current or 'R' for resistance.\n");
        exit(EXIT_FAILURE);
    }
}