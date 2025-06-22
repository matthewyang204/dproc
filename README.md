# `dproc`
**This project is under heavy development, and is therefore insuitable for daily use.**

`dproc` is a basic CLI data processor, designed to be fed data and output data directly from the commandline.

See the [todo list](TODO.md) for more details.

# Usage
`dproc [SUBCMD1] [SUBCMD2] {yourdata}`

#### `SUBCMD1` can be:
- `round` functions for rounding, getting averages, etc
- `deviate` functions for getting deviations and values related to deviations, such as the variance
- `organize` functions for organizing the dataset
- `enumerate` functions for enumerating (getting data about the dataset), such as getting their sum or counting how many values

#### `SUBCMD2` can be:

*When `SUBCMD1` is `round`:*
- `mean` get the mean of the dataset
- `median` get the median of the dataset
- `mode` get the mode of the dataset
- `decimal` round to a decimal place, with the value to be rounded supplied as the first value, and the number of decimal places maintained specified as the second
- `integer` round the number to the nearest integer

*When `SUBCMD1` is `deviate`:*
- `range` get the range of the data
- `variance` get the variance of the data
- `standard` get the standard deviation of the data; the program will ask you if the data given is a sample or population
- `meanAbsolute` get the mean absolute deviation
- `medianAbsolute` get the median absolute deviation

*When `SUBCMD1` is `organize`:*
- `sort` sort the data from smallest to largest

*When `SUBCMD1` is `enumerate`:*
- `sum` get the sum of the data
- `count` get the number of values in the data

Place your data, values separated by spaces, in the place of `{yourdata}`.
