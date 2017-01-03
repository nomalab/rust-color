use num;
use angle;
use angle::Angle;

pub trait ChannelFormatCast<Out> {
    fn cast(self) -> Out;
}

impl ChannelFormatCast<u8> for u8 {
    fn cast(self) -> u8 {
        self
    }
}
impl ChannelFormatCast<u16> for u8 {
    fn cast(self) -> u16 {
        (self as u16) * 0x0101_u16
    }
}
impl ChannelFormatCast<u32> for u8 {
    fn cast(self) -> u32 {
        (self as u32) * 0x01010101_u32
    }
}
impl ChannelFormatCast<u64> for u8 {
    fn cast(self) -> u64 {
        (self as u64) * 0x0101010101010101_u64
    }
}
impl ChannelFormatCast<f32> for u8 {
    fn cast(self) -> f32 {
        (self as f32) / (0xFF as f32)
    }
}
impl ChannelFormatCast<f64> for u8 {
    fn cast(self) -> f64 {
        (self as f64) / (0xFF as f64)
    }
}


impl ChannelFormatCast<u8> for u16 {
    fn cast(self) -> u8 {
        (self >> 8) as u8
    }
}
impl ChannelFormatCast<u16> for u16 {
    fn cast(self) -> u16 {
        self
    }
}
impl ChannelFormatCast<u32> for u16 {
    fn cast(self) -> u32 {
        (self as u32) * 0x00010001_u32
    }
}
impl ChannelFormatCast<u64> for u16 {
    fn cast(self) -> u64 {
        (self as u64) * 0x0001000100010001_u64
    }
}
impl ChannelFormatCast<f32> for u16 {
    fn cast(self) -> f32 {
        (self as f32) / (0xFFFF as f32)
    }
}
impl ChannelFormatCast<f64> for u16 {
    fn cast(self) -> f64 {
        (self as f64) / (0xFFFF as f64)
    }
}


impl ChannelFormatCast<u8> for u32 {
    fn cast(self) -> u8 {
        (self >> 24) as u8
    }
}
impl ChannelFormatCast<u16> for u32 {
    fn cast(self) -> u16 {
        (self >> 16) as u16
    }
}
impl ChannelFormatCast<u32> for u32 {
    fn cast(self) -> u32 {
        self
    }
}
impl ChannelFormatCast<u64> for u32 {
    fn cast(self) -> u64 {
        (self as u64) * 0x0000000100000001_u64
    }
}
impl ChannelFormatCast<f32> for u32 {
    fn cast(self) -> f32 {
        (self as f32) / (0xFFFFFFFF_u32 as f32)
    }
}
impl ChannelFormatCast<f64> for u32 {
    fn cast(self) -> f64 {
        (self as f64) / (0xFFFFFFFF_u32 as f64)
    }
}


impl ChannelFormatCast<u8> for u64 {
    fn cast(self) -> u8 {
        (self >> 56) as u8
    }
}
impl ChannelFormatCast<u16> for u64 {
    fn cast(self) -> u16 {
        (self >> 48) as u16
    }
}
impl ChannelFormatCast<u32> for u64 {
    fn cast(self) -> u32 {
        (self >> 24) as u32
    }
}
impl ChannelFormatCast<u64> for u64 {
    fn cast(self) -> u64 {
        self
    }
}
impl ChannelFormatCast<f32> for u64 {
    fn cast(self) -> f32 {
        (self as f32) / (0xFFFFFFFFFFFFFFFF_u64 as f32)
    }
}
impl ChannelFormatCast<f64> for u64 {
    fn cast(self) -> f64 {
        (self as f64) / (0xFFFFFFFFFFFFFFFF_u64 as f64)
    }
}


impl ChannelFormatCast<u8> for f32 {
    fn cast(self) -> u8 {
        // For u8, we want some extra precision, so multiply by just under 256.
        // This will make more than just 1.0 map to 255.
        (self * 255.99_f32).floor() as u8
    }
}
impl ChannelFormatCast<u16> for f32 {
    fn cast(self) -> u16 {
        (self * (0xFFFF_u32 as f32)) as u16
    }
}
impl ChannelFormatCast<u32> for f32 {
    fn cast(self) -> u32 {
        (self * (0xFFFFFFFF_u32 as f32)) as u32
    }
}
impl ChannelFormatCast<u64> for f32 {
    fn cast(self) -> u64 {
        (self * (0xFFFFFFFFFFFFFFFF_u64 as f32)) as u64
    }
}
impl ChannelFormatCast<f32> for f32 {
    fn cast(self) -> f32 {
        self
    }
}
impl ChannelFormatCast<f64> for f32 {
    fn cast(self) -> f64 {
        self as f64
    }
}


impl ChannelFormatCast<u8> for f64 {
    fn cast(self) -> u8 {
        // For u8, we want some extra precision, so multiply by just under 256.
        // This will make more than just 1.0 map to 255.
        (self * 255.99_f64).floor() as u8
    }
}
impl ChannelFormatCast<u16> for f64 {
    fn cast(self) -> u16 {
        (self * (0xFFFF_u32 as f64)) as u16
    }
}
impl ChannelFormatCast<u32> for f64 {
    fn cast(self) -> u32 {
        (self * (0xFFFFFFFF_u32 as f64)) as u32
    }
}
impl ChannelFormatCast<u64> for f64 {
    fn cast(self) -> u64 {
        (self * (0xFFFFFFFFFFFFFFFF_u64 as f64)) as u64
    }
}
impl ChannelFormatCast<f32> for f64 {
    fn cast(self) -> f32 {
        self as f32
    }
}
impl ChannelFormatCast<f64> for f64 {
    fn cast(self) -> f64 {
        self
    }
}

macro_rules! impl_channel_format_cast_for_angle {
    ($angle: ident) => {
        impl<T, A, U> ChannelFormatCast<A> for angle::$angle<T> 
            where A: Angle<Scalar=U>,
                  T: num::Float + ChannelFormatCast<U>,
                  U: num::Float
        {
            fn cast(self) -> A {
                let scalar: U = self.0.cast() 
                    * (A::period() / num::cast(Self::period()).unwrap());
                A::new(scalar)
            }
        }
    }
}

impl_channel_format_cast_for_angle!(Deg);
impl_channel_format_cast_for_angle!(Rad);
impl_channel_format_cast_for_angle!(Turns);
impl_channel_format_cast_for_angle!(ArcMinutes);
impl_channel_format_cast_for_angle!(ArcSeconds);