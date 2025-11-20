// Copyright 2025 Matthew Yang
// This file is part of libcharge, which is itself a part of libmysolvers.
// libmysolvers is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

#ifndef LIBCHARGE_H
#define LIBCHARGE_H

double get_volt(double I, double R);
double get_R(double V, double I);
double get_I(double V, double R);
double get_watt(double V, double IR, char* IorR);

#endif