extern crate num;
#[macro_use]
extern crate approx;
extern crate angular_units as angle;

#[macro_use]
mod impl_macros;

pub mod channel;
pub mod convert;
pub mod color;
pub mod chromaticity;
pub mod linalg;

pub mod white_point;

pub mod encoding;
pub mod color_space;

pub mod alpha;

pub mod rgb;
pub mod rgi;
pub mod hsv;
pub mod hsl;
pub mod hwb;
pub mod hsi;
pub mod ehsi;
pub mod ycbcr;
pub mod xyz;
pub mod xyy;
pub mod lab;
pub mod lchab;
pub mod luv;
pub mod lchuv;
pub mod lms;

#[cfg(test)]
pub mod test;
