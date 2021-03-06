//! Defines `BareYCbCr` for YCbCr colors that don't store their model.

use std::slice;
use std::mem;
use std::fmt;
use num;
use approx;
use channel::{NormalBoundedChannel, ColorChannel, NormalChannelScalar, ChannelFormatCast,
              ChannelCast, PosNormalChannelScalar, PosNormalBoundedChannel};
use color::{Color, FromTuple, Invert, Bounded, Lerp, Flatten};

use ycbcr::model::YCbCrModel;
use ycbcr::YCbCr;
use rgb::Rgb;

/// Methods for handling out of gamut colors when converting to Rgb.
///
/// These are used by the `to_rgb` method. Using `TryFromColor` will instead
/// return `None` any time an out of gamut value is produced.
pub enum OutOfGamutMode {
    /// Return the exact result of the transformation.
    ///
    /// This can result in channels outside the normal range
    /// (eg. less than 0 or greater than 1).
    Preserve,
    /// Clip any out-of-bounds channels to their minimum or maximum value (0.0 or 1.0).
    ///
    /// For example, -0.2 would go to 0.0 and 2.0 would go to 1.
    Clip,
}

/// A unit struct for identifying and constraining YCbCr colors in generic code.
pub struct YCbCrTag;

/// A YCbCr color that does not know its model.
///
/// `BareYCbCr` is used internally to implement `YCbCr` and is provided as
/// a separate type for performance reasons; generally, the use of `YCbCr` is preferred.
/// It is "bare" in the sense that it does not store the model information along with the
/// channel information. This makes it less smart, but can save memory when used with custom
/// models.
///
/// When using a custom model, `YCbCr` must store a reference to the model along with its
/// channel values. This can increase the memory footprint significantly, but gives greater
/// safety and convenience as well as forbidding illogical conversions and comparisons.
/// It is therefore only advised to use this when the extra memory consumption
/// has show to be an issue.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct BareYCbCr<T> {
    luma: PosNormalBoundedChannel<T>,
    cb: NormalBoundedChannel<T>,
    cr: NormalBoundedChannel<T>,
}

impl<T> BareYCbCr<T>
    where T: NormalChannelScalar + PosNormalChannelScalar
{
    /// Construct a `BareYCbCr` from channel values.
    pub fn from_channels(luma: T, cb: T, cr: T) -> Self {
        BareYCbCr {
            luma: PosNormalBoundedChannel::new(luma),
            cb: NormalBoundedChannel::new(cb),
            cr: NormalBoundedChannel::new(cr),
        }
    }

    impl_color_color_cast_square!(BareYCbCr {luma, cb, cr},
        chan_traits={PosNormalChannelScalar, NormalChannelScalar});

    /// Get the luma (Y') channel.
    pub fn luma(&self) -> T {
        self.luma.0.clone()
    }
    /// Get the Cb channel.
    pub fn cb(&self) -> T {
        self.cb.0.clone()
    }
    /// Get the Cr channel.
    pub fn cr(&self) -> T {
        self.cr.0.clone()
    }
    /// Get a mutable reference to the luma (Y') channel.
    pub fn luma_mut(&mut self) -> &mut T {
        &mut self.luma.0
    }
    /// Get a mutable reference to the Cb channel.
    pub fn cb_mut(&mut self) -> &mut T {
        &mut self.cb.0
    }
    /// Get a mutable reference to the Cr channel.
    pub fn cr_mut(&mut self) -> &mut T {
        &mut self.cr.0
    }
    /// Set the luma (Y') channel to a value.
    pub fn set_luma(&mut self, val: T) {
        self.luma.0 = val;
    }
    /// Set the Cb channel to a value.
    pub fn set_cb(&mut self, val: T) {
        self.cb.0 = val;
    }
    /// Set the Cr channel to a value.
    pub fn set_cr(&mut self, val: T) {
        self.cr.0 = val;
    }

    /// Construct a new `YCbCr` object from `self` and a model.
    ///
    /// Equivalent to constructing the `YCbCr` object directly:
    ///
    /// ```
    /// # use simple_color::ycbcr::*;
    /// let c = BareYCbCr::from_channels(0.5, 0.3, 0.2).with_model(JpegModel);
    /// assert_eq!(c, YCbCrJpeg::from_channels(0.5, 0.3, 0.2));
    /// ```
    pub fn with_model<M>(self, model: M) -> YCbCr<T, M>
        where M: YCbCrModel<T>
    {
        YCbCr::from_color_and_model(self, model)
    }
}

impl<T> Color for BareYCbCr<T>
    where T: PosNormalChannelScalar + NormalChannelScalar
{
    type Tag = YCbCrTag;
    type ChannelsTuple = (T, T, T);

    #[inline]
    fn num_channels() -> u32 {
        3
    }

    fn to_tuple(self) -> Self::ChannelsTuple {
        (self.luma.0, self.cb.0, self.cr.0)
    }
}

impl<T> FromTuple for BareYCbCr<T>
    where T: PosNormalChannelScalar + NormalChannelScalar
{
    fn from_tuple(values: Self::ChannelsTuple) -> Self {
        BareYCbCr::from_channels(values.0, values.1, values.2)
    }
}

impl<T> Invert for BareYCbCr<T>
    where T: PosNormalChannelScalar + NormalChannelScalar
{
    impl_color_invert!(BareYCbCr {luma, cb, cr});
}

impl<T> Bounded for BareYCbCr<T>
    where T: PosNormalChannelScalar + NormalChannelScalar
{
    impl_color_bounded!(BareYCbCr {luma, cb, cr});
}

impl<T> Lerp for BareYCbCr<T>
    where T: PosNormalChannelScalar + NormalChannelScalar + Lerp
{
    type Position = <T as Lerp>::Position;
    impl_color_lerp_square!(BareYCbCr {luma, cb, cr});
}

impl<T> Flatten for BareYCbCr<T>
    where T: PosNormalChannelScalar + NormalChannelScalar
{
    type ScalarFormat = T;

    impl_color_as_slice!(T);
    impl_color_from_slice_square!(BareYCbCr<T> {luma:PosNormalBoundedChannel - 0,
        cb:NormalBoundedChannel - 1, cr:NormalBoundedChannel - 2});
}

impl<T> approx::ApproxEq for BareYCbCr<T>
    where T: PosNormalChannelScalar + NormalChannelScalar + approx::ApproxEq,
          T::Epsilon: Clone
{
    impl_approx_eq!({luma, cb, cr});
}

impl<T> Default for BareYCbCr<T>
    where T: PosNormalChannelScalar + NormalChannelScalar + num::Zero
{
    impl_color_default!(BareYCbCr {luma:PosNormalBoundedChannel,
        cb:NormalBoundedChannel,
        cr:NormalBoundedChannel});
}

impl<T> fmt::Display for BareYCbCr<T>
    where T: PosNormalChannelScalar + NormalChannelScalar + fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "YCbCr({}, {}, {})", self.luma, self.cb, self.cr)
    }
}

impl<T> BareYCbCr<T>
    where T: NormalChannelScalar + PosNormalChannelScalar + num::NumCast
{
    /// Construct a `BareYCbCr` by converting from an Rgb `value`.
    ///
    /// `model` is only used within the conversion, it is up to the user
    /// to remember which model any `BareYCbCr` is using.
    pub fn from_rgb_and_model<M: YCbCrModel<T>>(from: &Rgb<T>, model: &M) -> Self {
        let transform = model.forward_transform();
        let shift = model.shift();

        let (y, cb, cr) = transform.transform_vector(from.clone().to_tuple());

        BareYCbCr::from_channels(y + shift.0, cb + shift.1, cr + shift.2)
    }

    /// Convert from YCbCr to Rgb.
    ///
    /// # Params
    ///
    /// * model - The model to use for the conversion. Note that this does not change the model
    ///   of the color being converted. If you convert to YCbCr from Rgb and convert back under a
    ///   different model, the resulting colors will be different.
    /// * out_of_gamut_mode - How to handle colors that are out of gamut in `Rgb`. See
    ///   [OutOfGamutMode](enum.OutOfGamutMode.html) for a description the options.
    pub fn to_rgb<M: YCbCrModel<T>>(&self, model: &M, out_of_gamut_mode: OutOfGamutMode) -> Rgb<T> {
        let transform = model.inverse_transform();
        let shift = model.shift();

        let (i1, i2, i3) = self.clone().to_tuple();
        let shifted_color =
            (num::cast::<_, f64>(i1).unwrap() - num::cast::<_, f64>(shift.0).unwrap(),
             num::cast::<_, f64>(i2).unwrap() - num::cast::<_, f64>(shift.1).unwrap(),
             num::cast::<_, f64>(i3).unwrap() - num::cast::<_, f64>(shift.2).unwrap());

        let (r, g, b) = transform.transform_vector(shifted_color);

        let out = Rgb::from_channels(num::cast(r).unwrap(),
                                     num::cast(g).unwrap(),
                                     num::cast(b).unwrap());

        match out_of_gamut_mode {
            OutOfGamutMode::Preserve => out,
            OutOfGamutMode::Clip => out.normalize(),
        }
    }
}
