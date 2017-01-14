#![allow(non_snake_case)]
use std::slice;
use std::mem;
use std::fmt;
use num;
use approx;
use channel::{FreeChannel, FreeChannelScalar, ChannelFormatCast, ChannelCast, ColorChannel};
use color::{Color, Bounded, Lerp, Flatten, FromTuple};
use xyz::Xyz;

pub struct LabTag;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Lab<T> {
    pub L: FreeChannel<T>,
    pub a: FreeChannel<T>,
    pub b: FreeChannel<T>,
}

impl<T> Lab<T>
    where T: FreeChannelScalar
{
    pub fn from_channels(L: T, a: T, b: T) -> Self {
        Lab {
            L: FreeChannel::new(L),
            a: FreeChannel::new(a),
            b: FreeChannel::new(b),
        }
    }

    impl_color_color_cast_square!(Lab {L, a, b}, chan_traits={FreeChannelScalar});

    pub fn L(&self) -> T {
        self.L.0.clone()
    }
    pub fn a(&self) -> T {
        self.a.0.clone()
    }
    pub fn b(&self) -> T {
        self.b.0.clone()
    }
    pub fn L_mut(&mut self) -> &mut T {
        &mut self.L.0
    }
    pub fn a_mut(&mut self) -> &mut T {
        &mut self.a.0
    }
    pub fn b_mut(&mut self) -> &mut T {
        &mut self.b.0
    }
    pub fn set_L(&mut self, val: T) {
        self.L.0 = val;
    }
    pub fn set_a(&mut self, val: T) {
        self.a.0 = val;
    }
    pub fn set_b(&mut self, val: T) {
        self.b.0 = val;
    }
}

impl<T> Color for Lab<T>
    where T: FreeChannelScalar
{
    type Tag = LabTag;
    type ChannelsTuple = (T, T, T);

    #[inline]
    fn num_channels() -> u32 {
        3
    }
    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.L.0, self.a.0, self.b.0)
    }
}

impl<T> FromTuple for Lab<T>
    where T: FreeChannelScalar
{
    fn from_tuple(values: (T, T, T)) -> Self {
        Lab::from_channels(values.0, values.1, values.2)
    }
}

impl<T> Bounded for Lab<T>
    where T: FreeChannelScalar
{
    fn normalize(self) -> Self {
        self
    }
    fn is_normalized(&self) -> bool {
        true
    }
}

impl<T> Lerp for Lab<T>
    where T: FreeChannelScalar,
          FreeChannel<T>: Lerp
{
    type Position = <FreeChannel<T> as Lerp>::Position;
    impl_color_lerp_square!(Lab {L, a, b});
}

impl<T> Flatten for Lab<T>
    where T: FreeChannelScalar
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_square!(Lab<T> {L:FreeChannel - 0, a:FreeChannel - 1,
        b:FreeChannel - 2});
}

impl<T> approx::ApproxEq for Lab<T>
    where T: FreeChannelScalar + approx::ApproxEq,
          T::Epsilon: Clone
{
    impl_approx_eq!({L, a, b});
}

impl<T> Default for Lab<T>
    where T: FreeChannelScalar
{
    impl_color_default!(Lab {L:FreeChannel, a:FreeChannel, b:FreeChannel});
}

impl<T> fmt::Display for Lab<T>
    where T: FreeChannelScalar + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L*a*b*({}, {}, {})", self.L, self.a, self.b)
    }
}

impl<T> Lab<T>
    where T: FreeChannelScalar
{
    pub fn from_xyz(from: &Xyz<T>, wp: &Xyz<T>) -> Lab<T> {
        let x = from.x() / wp.x();
        let y = from.y() / wp.y();
        let z = from.z() / wp.z();
        let L = num::cast::<_, T>(116.0).unwrap() * Lab::lab_f(y) - num::cast(16.0).unwrap();
        let a = num::cast::<_, T>(500.0).unwrap() * (Lab::lab_f(x) - Lab::lab_f(y));
        let b = num::cast::<_, T>(200.0).unwrap() * (Lab::lab_f(y) - Lab::lab_f(z));

        return Lab::from_channels(L, a, b);

    }

    pub fn to_xyz(&self, wp: &Xyz<T>) -> Xyz<T> {
        let fy = Self::inv_f_y(self.L());
        let fx = Self::inv_f_x(self.a(), fy);
        let fz = Self::inv_f_z(self.b(), fy);

        let x = Self::calc_xz(fx) * wp.x();
        let y = Self::calc_y(self.L()) * wp.y();
        let z = Self::calc_xz(fz) * wp.z();
        Xyz::from_channels(x, y, z)
    }

    fn lab_f(channel: T) -> T {
        if channel > Self::epsilon() {
            channel.cbrt()
        } else {
            (Self::kappa() * channel + num::cast(16.0).unwrap()) / num::cast(116.0).unwrap()
        }
    }

    fn calc_xz(f: T) -> T {
        let f3 = f * f * f;
        if f3 > Self::epsilon() {
            f3
        } else {
            (num::cast::<_, T>(116.0).unwrap() * f - num::cast::<_, T>(16.00).unwrap()) /
            Self::kappa()
        }
    }
    fn calc_y(L: T) -> T {
        if L > Self::kappa() * Self::epsilon() {
            let num = (L + num::cast::<_, T>(16.0).unwrap()) / num::cast::<_, T>(116.0).unwrap();
            num * num * num
        } else {
            L / Self::kappa()
        }

    }

    fn inv_f_x(a: T, fy: T) -> T {
        a / num::cast::<_, T>(500.0).unwrap() + fy
    }
    fn inv_f_y(L: T) -> T {
        (L + num::cast::<_, T>(16.0).unwrap()) / num::cast::<_, T>(116.0).unwrap()
    }
    fn inv_f_z(b: T, fy: T) -> T {
        fy - b / num::cast::<_, T>(200.0).unwrap()
    }

    #[inline]
    pub fn epsilon() -> T {
        num::cast(0.008856451679035631).unwrap()
    }
    #[inline]
    pub fn kappa() -> T {
        num::cast(903.2962962963).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use white_point::*;
    use xyz::Xyz;

    #[test]
    fn test_from_xyz() {
        let c1 = Xyz::from_channels(0.3, 0.22, 0.5);
        let t1 = Lab::from_xyz(&c1, &D65::get_xyz());
        assert_relative_eq!(t1, Lab::from_channels(54.0270, 38.5919, -33.5640), epsilon=1e-4);
        assert_relative_eq!(t1.to_xyz(&D65::get_xyz()), c1, epsilon=1e-4);

        let c2 = Xyz::from_channels(0.0, 0.0, 0.0);
        let t2 = Lab::from_xyz(&c2, &D65::get_xyz());
        assert_relative_eq!(t2, Lab::from_channels(0.0, 0.0, 0.0), epsilon=1e-4);
        assert_relative_eq!(t2.to_xyz(&D65::get_xyz()), c2, epsilon=1e-4);

        let c3 = Xyz::from_channels(1.0, 1.0, 1.0);
        let t3 = Lab::from_xyz(&c3, &D65::get_xyz());
        assert_relative_eq!(t3, Lab::from_channels(100.0, 8.5385, 5.5939), epsilon=1e-4);
        assert_relative_eq!(t3.to_xyz(&D65::get_xyz()), c3, epsilon=1e-4);

        let c4 = Xyz::from_channels(0.6, 0.8, 0.1);
        let t4 = Lab::from_xyz(&c4, &D50::get_xyz());
        let t4_2 = Lab::from_xyz(&c4, &E::get_xyz());
        assert_relative_eq!(t4, Lab::from_channels(91.6849, -37.2895, 86.6924), epsilon=1e-4);
        assert_relative_eq!(t4.to_xyz(&D50::get_xyz()), c4, epsilon=1e-4);
        assert!(t4.to_xyz(&D65::get_xyz()) != c4);
        assert_relative_eq!(t4_2, Lab::from_channels(91.6849, -42.4425, 92.8319), epsilon=1e-3);
        assert_relative_eq!(t4_2.to_xyz(&E::get_xyz()), c4, epsilon=1e-4);

        let c5 = D65::get_xyz();
        let t5 = Lab::from_xyz(&c5, &D65::get_xyz());
        assert_relative_eq!(t5, Lab::from_channels(100.0, 0.0, 0.0), epsilon=1e-4);
        assert_relative_eq!(t5.to_xyz(&D65::get_xyz()), c5);
    }

    #[test]
    fn test_to_xyz() {
        let c1 = Lab::from_channels(50.0, 33.0, -66.0);
        let t1 = c1.to_xyz(&D65::get_xyz());
        assert_relative_eq!(t1, Xyz::from_channels(0.243326, 0.184187, 0.791023), epsilon=1e-4);
        assert_relative_eq!(Lab::from_xyz(&t1, &D65::get_xyz()), c1, epsilon=1e-4);

        let c2 = Lab::from_channels(65.0, 47.5, 11.1);
        let t2 = c2.to_xyz(&D50::get_xyz());
        assert_relative_eq!(t2, Xyz::from_channels(0.4811337, 0.340472, 0.219151), epsilon=1e-3);
        assert_relative_eq!(Lab::from_xyz(&t2, &D50::get_xyz()), c2, epsilon=1e-3);

        let c3 = Lab::from_channels(100.0, -100.0, -100.0);
        let t3 = c3.to_xyz(&D75::get_xyz());
        assert_relative_eq!(t3, Xyz::from_channels(0.486257, 1.00, 4.139032), epsilon=1e-4);
        assert_relative_eq!(Lab::from_xyz(&t3, &D75::get_xyz()), c3, epsilon=1e-4);
    }
}