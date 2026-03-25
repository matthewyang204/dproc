// Regular imports
use std::env;
use std::process::exit;
use std::io::{self, BufRead};
use std::ffi::CString;
use std::os::raw::c_char;
use std::f64::consts::{PI, E};
use regex::Regex;
use std::convert::TryInto;

// Load modules
pub mod enumerate;
pub mod round;
pub mod freq;
pub mod sort;
pub use sort as crateSort;
pub mod deviate;
pub mod coreFuncs;
pub mod primes;
pub mod math;
pub mod utils;

// Use functions from modules
use round::mean;
use round::geoMean;
use round::harMean;
use round::median;
use freq::mode;
use freq::num;
use deviate::range;
use deviate::variance;
use deviate::sd;
use deviate::meanAD;
use deviate::medianAD;
use deviate::iqr;
use deviate::skewness;
use crateSort::sort;
use crateSort::keepUnique;
use enumerate::sum;
use enumerate::count;
use enumerate::min;
use enumerate::max;
#[allow(unused_imports)]
use primes::generate_primes;
use primes::is_prime;
use math::factorial;

// Load crates
use num_integer;
use num_bigint::BigUint;
use rhai::{Engine, Dynamic, Module};

// Include C functions from libmysolvers
unsafe extern "C" {
    fn quadratic_single_neg_solver(a: f64, b: f64, c: f64) -> f64;
    fn quadratic_single_pos_solver(a: f64, b: f64, c: f64) -> f64;

	fn linear_solver(val: i64, a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> f64;
	
	fn triangle_centroid(val: i64, x1: f64, y1: f64, x2: f64, y2:f64, x3: f64, y2:f64) -> f64;
}
// Include C functions from libcharge
unsafe extern "C" {
	fn get_volt(I: f64, R: f64) -> f64;
	fn get_R(V: f64, I: f64) -> f64;
	fn get_I(V: f64, R: f64) -> f64;

	fn get_watt(V: f64, IR: f64, IorR: c_char) -> f64;
}

// mod main;
// use main::{getArgs, floatExpr};

pub fn getArgs() -> Vec<String> {
	let args: Vec<String> = env::args().collect();
	return args;
}

pub fn floatExpr(expr: &str) -> String {
    let re = Regex::new(r"\b\d+(\.\d+)?\b").unwrap();
    let mut result = String::new();
    let mut last_end = 0;

    for mat in re.find_iter(expr) {
        let matched = mat.as_str();
        result.push_str(&expr[last_end..mat.start()]);

        if matched.contains('.') {
            result.push_str(matched);
        } else {
            result.push_str(&format!("{}.0", matched));
        }

        last_end = mat.end();
    }

    result.push_str(&expr[last_end..]);
    result
}

pub fn getStrFromVec(stringVec: Vec<String>, stringToSeek: String) -> (i64, bool) {
    let indexOpt = stringVec.iter().position(|x| *x == stringToSeek);
	let mut trueOrFalse: bool = false;
	let mut index: i64 = -1;
    match indexOpt {
        Some(i) => {
        	trueOrFalse = true;
        	index = i as i64;
        },
        None => {
            trueOrFalse = false;
        },
    }
    return (index, trueOrFalse);
}
