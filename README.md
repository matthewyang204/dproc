# `dproc`
`dproc` is a basic CLI data processor, designed to be fed data and output data directly from the commandline.

See the [todo list](TODO.md) for more details.

# Usage
`dproc [SUBCMD1] [SUBCMD2] {yourdata}`

#### `SUBCMD1` can be:
- `round` functions for rounding, getting averages, etc
- `deviate` functions for getting deviations and values related to deviations, such as the variance
- `organize` functions for organizing the dataset
- `enumerate` functions for enumerating (getting data about the dataset), such as getting their sum or counting how many values
- `math` miscellaneous functions for arbitrary things about integers like gcd and lcm
- `solve` functions for solving equations

#### `SUBCMD2` can be:

*When `SUBCMD1` is `round`:*
- `mean` get the mean of the dataset
- `geo-mean` get the geoemtric mean of the dataset
- `harmonic-mean` get the harmonic mean of the dataset
- `median` get the median of the dataset
- `decimal` round to a decimal place, with the value to be rounded supplied as the first value, and the number of decimal places maintained specified as the second
- `integer` round the number to the nearest integer

*When `SUBCMD1` is `freq`:*
- `mode` get the mode of the dataset

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

*When `SUBCMD1` is `math`:*
- `lcm` get the LCM of a dataset
- `gcd`, `gcf` get the GCD/GCF of a dataset
- `prime-check` check whether each number in the dataset is prime

*When `SUBCMD1` is `solve`:*
- `quadratic-single {{a}} {{b}} {{c}}` solve a quadratic equation with the quadratic formula, where a, b, and c are the coefficients of the quadratic equation ax^2 + bx + c = 0. Note that you may have to convert your equation; this does not accept != 0 on the other side of the equation.
- `linear-dual {{a1}} {{b1}} {{c1}} {{a2}} {{b2}} {{c2}}` solve a system of two linear equations with the substitution method, where a1, b1, c1 are the coefficients of the first linear equation a1x + b1y = c1 and a2, b2, c2 are the coefficients of the second linear equation a2x + b2y = c1. Note that this is standard form and you may have to convert your equations to it.

Place your data, values separated by spaces, in the place of `{yourdata}`. Alternatively, you may put `stdin` or `-` in the place of `{yourdata}` to read from stdin.

# Building
Requirements:
- `rustc`
- `cargo`
- `make`
- `gcc`, `clang`, or some other C99-compliant C compiler
- `ar` (usually comes with `binutils`, or more rarely, `coreutils`)

Additionally, if building the GUI program:
- `python` 3.6 or later
- If on Linux, the `python-is-python3` package that links the `python` command to `python3`
- the contents of `requirements.txt` installed in a fresh `venv` or directly in your python installation, noting that `pyinstaller` should be in PATH

To build:
1. Download the latest xzipped src tarball and unpack it, or clone the repo, and then `cd` into the resulting folder
2. Run `./configure`.
3. Run `make`. Alternatively, to make it run a specified number of jobs, run `make -j{yournumberofjobs}`. Replace `{yournumberofjobs}` with the number of jobs you want to run. You can also specify your build target in the `BUILDTARGET` variable (optional) via `make BUILDTAGET={yourtarget}`, replacing `{yourtarget}` with your target, such as `x86_64-apple-darwin`. You can also specify the linker with `make BUILDLINKER={yourlinker}`, where `{yourlinker}` is your linker. These options can all be combined or used separately, and they are both completely optional.

To build the GUI:
1. Enter the `gui` directory
2. Install the stuff in `requirements.txt` if applicable
3. Run `build.py`
4. Re-build the main program with GUI awareness

# Installing
Requirements:
- `make`
- A build from the previous section or a prebuilt binary

To install a custom build, simply run `make install` in the root of the built binary's folder (root folder of a downloaded-and-built source folder). You can specify where you want to install with the `PREFIX` var. To do this, run `make install PREFIX={yourprefix}`. Replace `{yourprefix}` with your prefix. Use `sudo` where appropriate.

If you want to install from a binary downloaded from the releases, simply move the binary to a folder of your choice in your PATH.

On macOS, you can also use Homebrew. To install using Homebrew, first tap my self-hosted tap via `brew tap matthewyang204/homebrew-formulae-casks`. Then, simply install with `brew install dproc`.

# GUI Frontends
It's very easy to build a GUI frontend for this program. You can build your own or use the one that comes with it. This processor can be integrated with multiple different types of graphical frontends or even middlemen. Some examples:
- Manual user interaction frontends - i,e  in similar nature to Git GUI, very close to bare stuff, basic entry boxes, buttons, etc
- AI middleman - training an AI to use the data processor and run it as subprocess, enhancing the AI's capabilities

However, for typical use, it is recommended to get familiar with the terminal and use it in the CLI where it is the most powerful.

## dproc GUI
**This software is still in heavy development and is not ready for production use; use at your own risk.**

This is the GUI frontend that is included in the source code and disabled by default. It is only enabled if the user chooses to enable it with the `--enable-gui` flag during configuration.

# License
This project is licensed under the GNU General Public License v3.0 (GPLv3).  
All past and future versions of `dproc` are covered by this license.  
See the LICENSE file for full details.
