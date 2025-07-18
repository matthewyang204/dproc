// Copyright 2025 Matthew Yang
// This file is part of libmysolvers.
// libmysolvers is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

#ifndef LIBMYSOLVERS_H
#define LIBMYSOLVERS_H

double quadratic_single_neg_solver(double a, double b, double c);
double quadratic_single_pos_solver(double a, double b, double c);
double linear_solver(double a1, double b1, double c1, double a2, double b2, double c2);

#endif