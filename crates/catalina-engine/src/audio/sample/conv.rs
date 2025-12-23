//! Pure functions and traits for converting between i8, i16, I24, i32, I48, i64, u8, u16, U24,
//! u32, U48, u64, f32 and f64.
//!
//! Each conversion function is performance focused, memory-sensitive and expects that the user has
//! validated their input prior to the function call.
//!
//! No conversion function will ever cast to a type with a size in bytes larger than the largest
//! between the source and target sample types.
//!
//! The conversion functions do *not* check the range of incoming values for floating point values
//! or any of the custom `I24`, `U24`, `I48` and `U48` types.
//!
//! Note that floating point conversions use the range -1.0 <= v < 1.0:
//! `(1.0 as f64).to_sample::<i16>()` will overflow!

use crate::audio::sample::types::{I24, I48, U24, U48};

macro_rules! conversion_fn {
    ($Rep:ty, $s:ident to_i8 { $body:expr }) => {
        #[inline]
        pub fn to_i8($s: $Rep) -> i8 {
            $body
        }
    };

    ($Rep:ty, $s:ident to_i16 { $body:expr }) => {
        #[inline]
        pub fn to_i16($s: $Rep) -> i16 {
            $body
        }
    };

    ($Rep:ty, $s:ident to_i24 { $body:expr }) => {
        #[inline]
        pub fn to_i24($s: $Rep) -> I24 {
            $body
        }
    };

    ($Rep:ty, $s:ident to_i32 { $body:expr }) => {
        #[inline]
        pub fn to_i32($s: $Rep) -> i32 {
            $body
        }
    };

    ($Rep:ty, $s:ident to_i48 { $body:expr }) => {
        #[inline]
        pub fn to_i48($s: $Rep) -> I48 {
            $body
        }
    };

    ($Rep:ty, $s:ident to_i64 { $body:expr }) => {
        #[inline]
        pub fn to_i64($s: $Rep) -> i64 {
            $body
        }
    };

    ($Rep:ty, $s:ident to_u8 { $body:expr }) => {
        #[inline]
        pub fn to_u8($s: $Rep) -> u8 {
            $body
        }
    };

    ($Rep:ty, $s:ident to_u16 { $body:expr }) => {
        #[inline]
        pub fn to_u16($s: $Rep) -> u16 {
            $body
        }
    };

    ($Rep:ty, $s:ident to_u24 { $body:expr }) => {
        #[inline]
        pub fn to_u24($s: $Rep) -> U24 {
            $body
        }
    };

    ($Rep:ty, $s:ident to_u32 { $body:expr }) => {
        #[inline]
        pub fn to_u32($s: $Rep) -> u32 {
            $body
        }
    };

    ($Rep:ty, $s:ident to_u48 { $body:expr }) => {
        #[inline]
        pub fn to_u48($s: $Rep) -> U48 {
            $body
        }
    };

    ($Rep:ty, $s:ident to_u64 { $body:expr }) => {
        #[inline]
        pub fn to_u64($s: $Rep) -> u64 {
            $body
        }
    };

    ($Rep:ty, $s:ident to_f32 { $body:expr }) => {
        #[inline]
        pub fn to_f32($s: $Rep) -> f32 {
            $body
        }
    };

    ($Rep:ty, $s:ident to_f64 { $body:expr }) => {
        #[inline]
        pub fn to_f64($s: $Rep) -> f64 {
            $body
        }
    };
}

macro_rules! conversion_fns {
    ($Rep:ty, $s:ident $fn_name:tt { $body:expr } $($rest:tt)*) => {
        conversion_fn!($Rep, $s $fn_name { $body });
        conversion_fns!($Rep, $($rest)*);
    };
    ($Rep:ty, ) => {};
}

macro_rules! conversions {
    ($T:ident, $mod_name:ident { $($rest:tt)* }) => {
        pub mod $mod_name {
            use crate::audio::sample::types::{I24, U24, I48, U48};
            conversion_fns!($T, $($rest)*);
        }
    };
}

conversions!(i8, i8 {
    s to_i16 { (s as i16) << 8 }
    s to_i24 { I24::new_unchecked((s as i32) << 16) }
    s to_i32 { (s as i32) << 24 }
    s to_i48 { I48::new_unchecked((s as i64) << 40) }
    s to_i64 { (s as i64) << 56 }
    s to_u8 {
        if s < 0 {
            // 128i8 overflows, so we must use 127 + 1 instead.
            (s + 127 + 1) as u8
        } else {
            (s as u8) + 128
        }
    }
    s to_u16 {
        if s < 0 {
            ((s + 127 + 1) as u16) << 8
        } else {
            (s as u16 + 128) << 8
        }
    }
    s to_u24 {
        U24::new_unchecked((s as i32 + 128) << 16)
    }
    s to_u32 {
        if s < 0 {
            ((s + 127 + 1) as u32) << 24
        } else {
            (s as u32 + 128) << 24
        }
    }
    s to_u48 {
        U48::new_unchecked((s as i64 + 128) << 40)
    }
    s to_u64 {
        if s < 0 {
            ((s + 127 + 1) as u64) << 56
        } else {
            (s as u64 + 128) << 56
        }
    }
    s to_f32 {
        s as f32 / 128.0
    }
    s to_f64 {
        s as f64 / 128.0
    }
});

conversions!(i16, i16 {
    s to_i8 { (s >> 8) as i8 }
    s to_i24 { I24::new_unchecked((s as i32) << 8) }
    s to_i32 { (s as i32) << 16 }
    s to_i48 { I48::new_unchecked((s as i64) << 32) }
    s to_i64 { (s as i64) << 48 }
    s to_u8 {
        super::i8::to_u8(to_i8(s))
    }
    s to_u16 {
        if s < 0 {
            // 32_768i16 overflows, so we must use + 1 instead.
            (s + 32_767 + 1) as u16
        } else {
            s as u16 + 32_768
        }
    }
    s to_u24 {
        if s < 0 {
            U24::new_unchecked(((s + 32_767 + 1) as i32) << 8)
        } else {
            U24::new_unchecked((s as i32 + 32_768) << 8)
        }
    }
    s to_u32 {
        if s < 0 {
            ((s + 32_767 + 1) as u32) << 16
        } else {
            ((s as u32) + 32_768) << 16
        }
    }
    s to_u48 {
        if s < 0 {
            U48::new_unchecked(((s + 32_767 + 1) as i64) << 32)
        } else {
            U48::new_unchecked((s as i64 + 32_768) << 32)
        }
    }
    s to_u64 {
        if s < 0 {
            ((s + 32_767 + 1) as u64) << 48
        } else {
            ((s as u64) + 32_768) << 48
        }
    }
    s to_f32 {
        s as f32 / 32_768.0
    }
    s to_f64 {
        s as f64 / 32_768.0
    }
});

conversions!(I24, i24 {
    s to_i8 { (s.inner() >> 16) as i8 }
    s to_i16 { (s.inner() >> 8) as i16 }
    s to_i32 { s.inner() << 8 }
    s to_i48 { I48::new_unchecked((s.inner() as i64) << 24) }
    s to_i64 { (s.inner() as i64) << 40 }
    s to_u8 {
        super::i8::to_u8(to_i8(s))
    }
    s to_u16 {
        super::i16::to_u16(to_i16(s))
    }
    s to_u24 {
        U24::new_unchecked(s.inner() + 8_388_608)
    }
    s to_u32 {
        ((s.inner() + 8_388_608) as u32) << 8
    }
    s to_u48 {
        U48::new_unchecked((s.inner() as i64 + 8_388_608) << 24)
    }
    s to_u64 {
        ((s.inner() + 8_388_608) as u64) << 40
    }
    s to_f32 {
        s.inner() as f32 / 8_388_608.0
    }
    s to_f64 {
        s.inner() as f64 / 8_388_608.0
    }
});

conversions!(i32, i32 {
    s to_i8 { (s >> 24) as i8 }
    s to_i16 { (s >> 16) as i16 }
    s to_i24 { I24::new_unchecked(s >> 8) }
    s to_i48 { I48::new_unchecked((s as i64) << 16) }
    s to_i64 { (s as i64) << 32 }
    s to_u8 {
        super::i8::to_u8(to_i8(s))
    }
    s to_u16 {
        super::i16::to_u16(to_i16(s))
    }
    s to_u24 {
        super::i24::to_u24(to_i24(s))
    }
    s to_u32 {
        if s < 0 {
            (s + 2_147_483_647 + 1) as u32
        } else {
            s as u32 + 2_147_483_648
        }
    }
    s to_u48 {
        U48::new_unchecked((s as i64 + 2_147_483_648) << 16)
    }
    s to_u64 {
        if s < 0 {
            ((s + 2_147_483_647 + 1) as u64) << 32
        } else {
            (s as u64) + 2_147_483_648 << 32
        }
    }
    s to_f32 {
        s as f32 / 2_147_483_648.0
    }
    s to_f64 {
        s as f64 / 2_147_483_648.0
    }
});

conversions!(I48, i48 {
    s to_i8 { (s.inner() >> 40) as i8 }
    s to_i16 { (s.inner() >> 32) as i16 }
    s to_i24 { I24::new_unchecked((s.inner() >> 24) as i32) }
    s to_i32 { (s.inner() >> 16) as i32 }
    s to_i64 { s.inner() << 16 }
    s to_u8 {
        super::i8::to_u8(to_i8(s))
    }
    s to_u16 {
        super::i16::to_u16(to_i16(s))
    }
    s to_u24 {
        super::i24::to_u24(to_i24(s))
    }
    s to_u32 {
        super::i32::to_u32(to_i32(s))
    }
    s to_u48 {
        U48::new_unchecked(s.inner() + 140_737_488_355_328)
    }
    s to_u64 {
        ((s.inner() + 140_737_488_355_328) as u64) << 16
    }
    s to_f32 {
        s.inner() as f32 / 140_737_488_355_328.0
    }
    s to_f64 {
        s.inner() as f64 / 140_737_488_355_328.0
    }
});

conversions!(i64, i64 {
    s to_i8 { (s >> 56) as i8 }
    s to_i16 { (s >> 48) as i16 }
    s to_i24 { I24::new_unchecked((s >> 40) as i32) }
    s to_i32 { (s >> 32) as i32 }
    s to_i48 { I48::new_unchecked(s >> 16) }
    s to_u8 {
        super::i8::to_u8(to_i8(s))
    }
    s to_u16 {
        super::i16::to_u16(to_i16(s))
    }
    s to_u24 {
        super::i24::to_u24(to_i24(s))
    }
    s to_u32 {
        super::i32::to_u32(to_i32(s))
    }
    s to_u48 {
        super::i48::to_u48(to_i48(s))
    }
    s to_u64 {
        if s < 0 {
            (s + 9_223_372_036_854_775_807 + 1) as u64
        } else {
            s as u64 + 9_223_372_036_854_775_808
        }
    }
    s to_f32 {
        s as f32 / 9_223_372_036_854_775_808.0
    }
    s to_f64 {
        s as f64 / 9_223_372_036_854_775_808.0
    }
});

conversions!(u8, u8 {
    s to_i8 {
        if s < 128 {
            s as i8 - 127 - 1
        } else {
            (s - 128) as i8
        }
    }
    s to_i16 {
        (s as i16 - 128) << 8
    }
    s to_i24 {
        I24::new_unchecked((s as i32 - 128) << 16)
    }
    s to_i32 {
        (s as i32 - 128) << 24
    }
    s to_i48 {
        I48::new_unchecked((s as i64 - 128) << 40)
    }
    s to_i64 {
        (s as i64 - 128) << 56
    }
    s to_u16 { (s as u16) << 8 }
    s to_u24 { U24::new_unchecked((s as i32) << 16) }
    s to_u32 { (s as u32) << 24 }
    s to_u48 { U48::new_unchecked((s as i64) << 40) }
    s to_u64 { (s as u64) << 56 }
    s to_f32 { super::i8::to_f32(to_i8(s)) }
    s to_f64 { super::i8::to_f64(to_i8(s)) }
});

conversions!(u16, u16 {
    s to_i8 { super::u8::to_i8(to_u8(s)) }
    s to_i16 {
        if s < 32_768 {
            s as i16 - 32_767 - 1
        } else {
            (s - 32_768) as i16
        }
    }
    s to_i24 {
        I24::new_unchecked((s as i32 - 32_768) << 8)
    }
    s to_i32 {
        (s as i32 - 32_768) << 16
    }
    s to_i48 {
        I48::new_unchecked((s as i64 - 32_768) << 32)
    }
    s to_i64 {
        (s as i64 - 32_768) << 48
    }
    s to_u8 { (s >> 8) as u8 }
    s to_u24 { U24::new_unchecked((s as i32) << 8) }
    s to_u32 { (s as u32) << 16 }
    s to_u48 { U48::new_unchecked((s as i64) << 32) }
    s to_u64 { (s as u64) << 48 }
    s to_f32 { super::i16::to_f32(to_i16(s)) }
    s to_f64 { super::i16::to_f64(to_i16(s)) }
});

conversions!(U24, u24 {
    s to_i8 { super::u8::to_i8(to_u8(s)) }
    s to_i16 { super::u16::to_i16(to_u16(s)) }
    s to_i24 {
        I24::new_unchecked(s.inner() - 8_388_608)
    }
    s to_i32 {
        (s.inner() - 8_388_608) << 8
    }
    s to_i48 {
        I48::new_unchecked(((s.inner() as i64) - 8_388_608) << 24)
    }
    s to_i64 {
        (s.inner() as i64 - 8_388_608) << 40
    }
    s to_u8 { (s.inner() >> 16) as u8 }
    s to_u16 { (s.inner() >> 8) as u16 }
    s to_u32 { (s.inner() as u32) << 8 }
    s to_u48 { U48::new_unchecked((s.inner() as i64) << 24) }
    s to_u64 { (s.inner() as u64) << 40 }
    s to_f32 { super::i24::to_f32(to_i24(s)) }
    s to_f64 { super::i24::to_f64(to_i24(s)) }
});

conversions!(u32, u32 {
    s to_i8 { super::u8::to_i8(to_u8(s)) }
    s to_i16 { super::u16::to_i16(to_u16(s)) }
    s to_i24 { super::u24::to_i24(to_u24(s)) }
    s to_i32 {
        if s < 2_147_483_648 {
            s as i32 - 2_147_483_647 - 1
        } else {
            (s - 2_147_483_648) as i32
        }
    }
    s to_i48 {
        I48::new_unchecked((s as i64 - 2_147_483_648) << 16)
    }
    s to_i64 {
        (s as i64 - 2_147_483_648) << 32
    }
    s to_u8 { (s >> 24) as u8 }
    s to_u16 { (s >> 16) as u16 }
    s to_u24 { U24::new_unchecked((s >> 8) as i32) }
    s to_u48 { U48::new_unchecked((s as i64) << 16) }
    s to_u64 { (s as u64) << 32 }
    s to_f32 { super::i32::to_f32(to_i32(s)) }
    s to_f64 { super::i32::to_f64(to_i32(s)) }
});

conversions!(U48, u48 {
    s to_i8 { super::u8::to_i8(to_u8(s)) }
    s to_i16 { super::u16::to_i16(to_u16(s)) }
    s to_i24 { super::u24::to_i24(to_u24(s)) }
    s to_i32 { super::u32::to_i32(to_u32(s)) }
    s to_i48 {
        I48::new_unchecked(s.inner() - 140_737_488_355_328)
    }
    s to_i64 {
        (s.inner() - 140_737_488_355_328) << 16
    }
    s to_u8 { (s.inner() >> 40) as u8 }
    s to_u16 { (s.inner() >> 32) as u16 }
    s to_u24 { U24::new_unchecked((s.inner() >> 24) as i32) }
    s to_u32 { (s.inner() >> 16) as u32 }
    s to_u64 { (s.inner() as u64) << 16 }
    s to_f32 { super::i48::to_f32(to_i48(s)) }
    s to_f64 { super::i48::to_f64(to_i48(s)) }
});

conversions!(u64, u64 {
    s to_i8 { super::u8::to_i8(to_u8(s)) }
    s to_i16 { super::u16::to_i16(to_u16(s)) }
    s to_i24 { super::u24::to_i24(to_u24(s)) }
    s to_i32 { super::u32::to_i32(to_u32(s)) }
    s to_i48 { super::u48::to_i48(to_u48(s)) }
    s to_i64 {
        if s < 9_223_372_036_854_775_808 {
            s as i64 - 9_223_372_036_854_775_807 - 1
        } else {
            (s - 9_223_372_036_854_775_808) as i64
        }
    }
    s to_u8 { (s >> 56) as u8 }
    s to_u16 { (s >> 48) as u16 }
    s to_u24 { U24::new_unchecked((s >> 40) as i32) }
    s to_u32 { (s >> 32) as u32 }
    s to_u48 { U48::new_unchecked((s >> 16) as i64) }
    s to_f32 { super::i64::to_f32(to_i64(s)) }
    s to_f64 { super::i64::to_f64(to_i64(s)) }
});

// The following conversions assume `-1.0 <= s < 1.0` (note that +1.0 is excluded) and will
// overflow otherwise.
conversions!(f32, f32 {
    s to_i8 { (s * 128.0) as i8 }
    s to_i16 { (s * 32_768.0) as i16 }
    s to_i24 { I24::new_unchecked((s * 8_388_608.0) as i32) }
    s to_i32 { (s * 2_147_483_648.0) as i32 }
    s to_i48 { I48::new_unchecked((s * 140_737_488_355_328.0) as i64) }
    s to_i64 { (s * 9_223_372_036_854_775_808.0) as i64 }
    s to_u8 { super::i8::to_u8(to_i8(s)) }
    s to_u16 { super::i16::to_u16(to_i16(s)) }
    s to_u24 { super::i24::to_u24(to_i24(s)) }
    s to_u32 { super::i32::to_u32(to_i32(s)) }
    s to_u48 { super::i48::to_u48(to_i48(s)) }
    s to_u64 { super::i64::to_u64(to_i64(s)) }
    s to_f64 { s as f64 }
});

// The following conversions assume `-1.0 <= s < 1.0` (note that +1.0 is excluded) and will
// overflow otherwise.
conversions!(f64, f64 {
    s to_i8 { (s * 128.0) as i8 }
    s to_i16 { (s * 32_768.0) as i16 }
    s to_i24 { I24::new_unchecked((s * 8_388_608.0) as i32) }
    s to_i32 { (s * 2_147_483_648.0) as i32 }
    s to_i48 { I48::new_unchecked((s * 140_737_488_355_328.0) as i64) }
    s to_i64 { (s * 9_223_372_036_854_775_808.0) as i64 }
    s to_u8 { super::i8::to_u8(to_i8(s)) }
    s to_u16 { super::i16::to_u16(to_i16(s)) }
    s to_u24 { super::i24::to_u24(to_i24(s)) }
    s to_u32 { super::i32::to_u32(to_i32(s)) }
    s to_u48 { super::i48::to_u48(to_i48(s)) }
    s to_u64 { super::i64::to_u64(to_i64(s)) }
    s to_f32 { s as f32 }
});

/// Similar to the std `From` trait, but specifically for converting between sample types.
///
/// We use this trait to be generic over the `Sample::to_sample` and `Sample::from_sample` methods.
pub trait FromSample<S> {
    fn from_sample_(s: S) -> Self;
}

impl<S> FromSample<S> for S {
    #[inline]
    fn from_sample_(s: S) -> Self {
        s
    }
}

/// Implement the `FromSample` trait for the given types.
macro_rules! impl_from_sample {
    ($T:ty, $fn_name:ident from $({$U:ident: $Umod:ident})*) => {
        $(
            impl FromSample<$U> for $T {
                #[inline]
                fn from_sample_(s: $U) -> Self {
                    self::$Umod::$fn_name(s)
                }
            }
        )*
    };
}

impl_from_sample! {i8, to_i8 from
    {i16:i16} {I24:i24} {i32:i32} {I48:i48} {i64:i64}
    {u8:u8} {u16:u16} {U24:u24} {u32:u32} {U48:u48} {u64:u64}
    {f32:f32} {f64:f64}
}

impl_from_sample! {i16, to_i16 from
    {i8:i8} {I24:i24} {i32:i32} {I48:i48} {i64:i64}
    {u8:u8} {u16:u16} {U24:u24} {u32:u32} {U48:u48} {u64:u64}
    {f32:f32} {f64:f64}
}

impl_from_sample! {I24, to_i24 from
    {i8:i8} {i16:i16} {i32:i32} {I48:i48} {i64:i64}
    {u8:u8} {u16:u16} {U24:u24} {u32:u32} {U48:u48} {u64:u64}
    {f32:f32} {f64:f64}
}

impl_from_sample! {i32, to_i32 from
    {i8:i8} {i16:i16} {I24:i24} {I48:i48} {i64:i64}
    {u8:u8} {u16:u16} {U24:u24} {u32:u32} {U48:u48} {u64:u64}
    {f32:f32} {f64:f64}
}

impl_from_sample! {I48, to_i48 from
    {i8:i8} {i16:i16} {I24:i24} {i32:i32} {i64:i64}
    {u8:u8} {u16:u16} {U24:u24} {u32:u32} {U48:u48} {u64:u64}
    {f32:f32} {f64:f64}
}

impl_from_sample! {i64, to_i64 from
    {i8:i8} {i16:i16} {I24:i24} {i32:i32} {I48:i48}
    {u8:u8} {u16:u16} {U24:u24} {u32:u32} {U48:u48} {u64:u64}
    {f32:f32} {f64:f64}
}

impl_from_sample! {u8, to_u8 from
    {i8:i8} {i16:i16} {I24:i24} {i32:i32} {I48:i48} {i64:i64}
    {u16:u16} {U24:u24} {u32:u32} {U48:u48} {u64:u64}
    {f32:f32} {f64:f64}
}

impl_from_sample! {u16, to_u16 from
    {i8:i8} {i16:i16} {I24:i24} {i32:i32} {I48:i48} {i64:i64}
    {u8:u8} {U24:u24} {u32:u32} {U48:u48} {u64:u64}
    {f32:f32} {f64:f64}
}

impl_from_sample! {U24, to_u24 from
    {i8:i8} {i16:i16} {I24:i24} {i32:i32} {I48:i48} {i64:i64}
    {u8:u8} {u16:u16} {u32:u32} {U48:u48} {u64:u64}
    {f32:f32} {f64:f64}
}

impl_from_sample! {u32, to_u32 from
    {i8:i8} {i16:i16} {I24:i24} {i32:i32} {I48:i48} {i64:i64}
    {u8:u8} {u16:u16} {U24:u24} {U48:u48} {u64:u64}
    {f32:f32} {f64:f64}
}

impl_from_sample! {U48, to_u48 from
    {i8:i8} {i16:i16} {I24:i24} {i32:i32} {I48:i48} {i64:i64}
    {u8:u8} {u16:u16} {U24:u24} {u32:u32} {u64:u64}
    {f32:f32} {f64:f64}
}

impl_from_sample! {u64, to_u64 from
    {i8:i8} {i16:i16} {I24:i24} {i32:i32} {I48:i48} {i64:i64}
    {u8:u8} {u16:u16} {U24:u24} {u32:u32} {U48:u48}
    {f32:f32} {f64:f64}
}

impl_from_sample! {f32, to_f32 from
    {i8:i8} {i16:i16} {I24:i24} {i32:i32} {I48:i48} {i64:i64}
    {u8:u8} {u16:u16} {U24:u24} {u32:u32} {U48:u48} {u64:u64}
    {f64:f64}
}

impl_from_sample! {f64, to_f64 from
    {i8:i8} {i16:i16} {I24:i24} {i32:i32} {I48:i48} {i64:i64}
    {u8:u8} {u16:u16} {U24:u24} {u32:u32} {U48:u48} {u64:u64}
    {f32:f32}
}

/// Similar to the std `Into` trait, but specifically for converting between sample types.
///
/// This trait has a blanket implementation for all types that implement
/// [`FromSample`](./trait.FromSample.html).
pub trait ToSample<S> {
    fn to_sample_(self) -> S;
}

impl<T, U> ToSample<U> for T
where
    U: FromSample<T>,
{
    #[inline]
    fn to_sample_(self) -> U {
        U::from_sample_(self)
    }
}

/// Sample types which may be converted to and from some type `S`.
pub trait Duplex<S>: FromSample<S> + ToSample<S> {}
impl<S, T> Duplex<S> for T where T: FromSample<S> + ToSample<S> {}

#[cfg(test)]
mod tests {
    //! The following is a series of tests that check conversions between every combination of sample
    //! types available within this crate.
    //!
    //! We assert that each sample type's minimum, maximum and centre are correctly converted to the
    //! min, max and centre of every other available sample type.

    /// Expands to an `assert_eq` for each pre-conversion and post-conversion pair.
    ///
    /// Literals that must be wrapped by a custom sample type are wrapped using $T/$U::new_unchecked.
    macro_rules! conv_cmp {
        ($fn_name:ident, $pre_conv:expr, $post_conv:expr) => {
            assert_eq!($fn_name($pre_conv), $post_conv);
        };
        ($fn_name:ident: $U:ident, $pre_conv:expr, $post_conv:expr) => {
            assert_eq!($fn_name($pre_conv), $U::new_unchecked($post_conv));
        };
        ($T:ident; $fn_name:ident, $pre_conv:expr, $post_conv:expr) => {
            assert_eq!($fn_name($T::new_unchecked($pre_conv)), $post_conv);
        };
        ($T:ident; $fn_name:ident: $U:ident, $pre_conv:expr, $post_conv:expr) => {
            assert_eq!(
                $fn_name($T::new_unchecked($pre_conv)),
                $U::new_unchecked($post_conv)
            );
        };
    }

    /// Expands to a list of `assert_eq` statements.
    macro_rules! conv_cmps {
    ($fn_name:ident, $pre_conv:expr, $post_conv:expr; $($rest:tt)*) => {
        conv_cmp!($fn_name, $pre_conv, $post_conv);
        conv_cmps!($fn_name, $($rest)*);
    };
    ($fn_name:ident: $U:ident, $pre_conv:expr, $post_conv:expr; $($rest:tt)*) => {
        conv_cmp!($fn_name:$U, $pre_conv, $post_conv);
        conv_cmps!($fn_name:$U, $($rest)*);
    };
    ($T:ident; $fn_name:ident, $pre_conv:expr, $post_conv:expr; $($rest:tt)*) => {
        conv_cmp!($T; $fn_name, $pre_conv, $post_conv);
        conv_cmps!($T; $fn_name, $($rest)*);
    };
    ($T:ident; $fn_name:ident: $U:ident, $pre_conv:expr, $post_conv:expr; $($rest:tt)*) => {
        conv_cmp!($T; $fn_name:$U, $pre_conv, $post_conv);
        conv_cmps!($T; $fn_name:$U, $($rest)*);
    };
    ($fn_name:ident,) => {};
    ($fn_name:ident: $U:ident,) => {};
    ($T:ident; $fn_name:ident,) => {};
    ($T:ident; $fn_name:ident: $U:ident,) => {};
}

    /// Expands to a test function for the given test function name.
    ///
    /// We must use one for each as:
    /// 1. There is no concat-idents macro for constructing unique names from other identifiers and
    /// 2. We need to check for functions that convert to custom sample types (i.e. to_i24 converts to
    ///    `I24`).
    macro_rules! test_fn {

    (to_i8 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_i8() {
            conv_cmps!(to_i8, $($conv_cmps)*);
        }
    };

    (to_i16 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_i16() {
            conv_cmps!(to_i16, $($conv_cmps)*);
        }
    };

    (to_i24 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_i24() {
            conv_cmps!(to_i24: I24, $($conv_cmps)*);
        }
    };

    (to_i32 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_i32() {
            conv_cmps!(to_i32, $($conv_cmps)*);
        }
    };

    (to_i48 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_i48() {
            conv_cmps!(to_i48: I48, $($conv_cmps)*);
        }
    };

    (to_i64 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_i64() {
            conv_cmps!(to_i64, $($conv_cmps)*);
        }
    };

    (to_u8 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_u8() {
            conv_cmps!(to_u8, $($conv_cmps)*);
        }
    };

    (to_u16 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_u16() {
            conv_cmps!(to_u16, $($conv_cmps)*);
        }
    };

    (to_u24 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_u24() {
            conv_cmps!(to_u24: U24, $($conv_cmps)*);
        }
    };

    (to_u32 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_u32() {
            conv_cmps!(to_u32, $($conv_cmps)*);
        }
    };

    (to_u48 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_u48() {
            conv_cmps!(to_u48: U48, $($conv_cmps)*);
        }
    };

    (to_u64 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_u64() {
            conv_cmps!(to_u64, $($conv_cmps)*);
        }
    };

    (to_f32 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_f32() {
            conv_cmps!(to_f32, $($conv_cmps)*);
        }
    };

    (to_f64 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_f64() {
            conv_cmps!(to_f64, $($conv_cmps)*);
        }
    };

    // Test functions for wrapper sample types.

    ($T:ident: to_i8 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_i8() {
            conv_cmps!($T; to_i8, $($conv_cmps)*);
        }
    };

    ($T:ident: to_i16 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_i16() {
            conv_cmps!($T; to_i16, $($conv_cmps)*);
        }
    };

    ($T:ident: to_i24 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_i24() {
            conv_cmps!($T; to_i24: I24, $($conv_cmps)*);
        }
    };

    ($T:ident: to_i32 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_i32() {
            conv_cmps!($T; to_i32, $($conv_cmps)*);
        }
    };

    ($T:ident: to_i48 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_i48() {
            conv_cmps!($T; to_i48: I48, $($conv_cmps)*);
        }
    };

    ($T:ident: to_i64 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_i64() {
            conv_cmps!($T; to_i64, $($conv_cmps)*);
        }
    };

    ($T:ident: to_u8 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_u8() {
            conv_cmps!($T; to_u8, $($conv_cmps)*);
        }
    };

    ($T:ident: to_u16 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_u16() {
            conv_cmps!($T; to_u16, $($conv_cmps)*);
        }
    };

    ($T:ident: to_u24 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_u24() {
            conv_cmps!($T; to_u24: U24, $($conv_cmps)*);
        }
    };

    ($T:ident: to_u32 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_u32() {
            conv_cmps!($T; to_u32, $($conv_cmps)*);
        }
    };

    ($T:ident: to_u48 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_u48() {
            conv_cmps!($T; to_u48: U48, $($conv_cmps)*);
        }
    };

    ($T:ident: to_u64 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_u64() {
            conv_cmps!($T; to_u64, $($conv_cmps)*);
        }
    };

    ($T:ident: to_f32 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_f32() {
            conv_cmps!($T; to_f32, $($conv_cmps)*);
        }
    };

    ($T:ident: to_f64 { $($conv_cmps:tt)* }) => {
        #[test]
        fn test_to_f64() {
            conv_cmps!($T; to_f64, $($conv_cmps)*);
        }
    };
}

    /// Expands to a list of test functions.
    macro_rules! test_fns {
    ($fn_name:tt { $($conv_cmps:tt)* } $($rest:tt)*) => {
        test_fn!($fn_name { $($conv_cmps)* });
        test_fns!($($rest)*);
    };
    ($T:ident: $fn_name:tt { $($conv_cmps:tt)* } $($rest:tt)*) => {
        test_fn!($T: $fn_name { $($conv_cmps)* });
        test_fns!($T: $($rest)*);
    };
    () => {};
    ($T:ident:) => {};
}

    /// Expands to a unique test module containing a list of test functions.
    macro_rules! tests {
    ($T:ident { $($rest:tt)* }) => {
        pub mod $T {
            use crate::audio::sample::conv::$T::*;
            use crate::audio::sample::types::{I24, U24, I48, U48};
            test_fns!($($rest)*);
        }
    };
    ($T:ident: $mod_name:ident { $($rest:tt)* }) => {
        pub mod $mod_name {
            use crate::audio::sample::conv::$mod_name::*;
            use crate::audio::sample::types::{I24, U24, I48, U48};
            test_fns!($T: $($rest)*);
        }
    };
}

    tests!(i8 {
        to_i16 { -128, -32_768; 0, 0; 127, 32_512; }
        to_i24 { -128, -8_388_608; 0, 0; 127, 8_323_072; }
        to_i32 { -128, -2_147_483_648; 0, 0; 127, 2_130_706_432; }
        to_i48 { -128, -140_737_488_355_328; 0, 0; 127, 139_637_976_727_552; }
        to_i64 { -128, -9_223_372_036_854_775_808; 0, 0; 127, 9_151_314_442_816_847_872; }
        to_u8  { -128, 0; 0, 128; 127, 255; }
        to_u16 { -128, 0; 0, 32_768; 127, 65_280; }
        to_u24 { -128, 0; 0, 8_388_608; 127, 16_711_680; }
        to_u32 { -128, 0; 0, 2_147_483_648; 127, 4_278_190_080; }
        to_u48 { -128, 0; 0, 140_737_488_355_328; 127, 280_375_465_082_880; }
        to_u64 { -128, 0; 0, 9_223_372_036_854_775_808; 127, 18_374_686_479_671_623_680; }
        to_f32 { -128, -1.0; 0, 0.0; }
        to_f64 { -128, -1.0; 0, 0.0; }
    });

    tests!(i16 {
        to_i8  { -32_768, -128; 0, 0; 32_767, 127; }
        to_i24 { -32_768, -8_388_608; 0, 0; 32_767, 8_388_352; }
        to_i32 { -32_768, -2_147_483_648; 0, 0; 32_767, 2_147_418_112; }
        to_i48 { -32_768, -140_737_488_355_328; 0, 0; 32_767, 140_733_193_388_032; }
        to_i64 { -32_768, -9_223_372_036_854_775_808; 0, 0; 32_767, 9_223_090_561_878_065_152; }
        to_u8  { -32_768, 0; 0, 128; 32_767, 255; }
        to_u16 { -32_768, 0; 0, 32_768; 32_767, 65_535; }
        to_u24 { -32_768, 0; 0, 8_388_608; 32_767, 16_776_960; }
        to_u32 { -32_768, 0; 0, 2_147_483_648; 32_767, 4_294_901_760; }
        to_u48 { -32_768, 0; 0, 140_737_488_355_328; 32_767, 281_470_681_743_360; }
        to_u64 { -32_768, 0; 0, 9_223_372_036_854_775_808; 32_767, 18_446_462_598_732_840_960; }
        to_f32 { -32_768, -1.0; 0, 0.0; }
        to_f64 { -32_768, -1.0; 0, 0.0; }
    });

    tests!(I24: i24 {
        to_i8  { -8_388_608, -128; 0, 0; 8_388_607, 127; }
        to_i16 { -8_388_608, -32_768; 0, 0; 8_388_607, 32_767; }
        to_i32 { -8_388_608, -2_147_483_648; 0, 0; 8_388_607, 2_147_483_392; }
        to_i48 { -8_388_608, -140_737_488_355_328; 0, 0; 8_388_607, 140_737_471_578_112; }
        to_i64 { -8_388_608, -9_223_372_036_854_775_808; 0, 0; 8_388_607, 9_223_370_937_343_148_032; }
        to_u8  { -8_388_608, 0; 0, 128; 8_388_607, 255; }
        to_u16 { -8_388_608, 0; 0, 32_768; 8_388_607, 65_535; }
        to_u24 { -8_388_608, 0; 0, 8_388_608; 8_388_607, 16_777_215; }
        to_u32 { -8_388_608, 0; 0, 2_147_483_648; 8_388_607, 4_294_967_040; }
        to_u48 { -8_388_608, 0; 0, 140_737_488_355_328; 8_388_607, 281_474_959_933_440; }
        to_u64 { -8_388_608, 0; 0, 9_223_372_036_854_775_808; 8_388_607, 18_446_742_974_197_923_840; }
        to_f32 { -8_388_608, -1.0; 0, 0.0; }
        to_f64 { -8_388_608, -1.0; 0, 0.0; }
    });

    tests!(i32 {
        to_i8  { -2_147_483_648, -128; 0, 0; 2_147_483_647, 127; }
        to_i16 { -2_147_483_648, -32_768; 0, 0; 2_147_483_647, 32_767; }
        to_i24 { -2_147_483_648, -8_388_608; 0, 0; 2_147_483_647, 8_388_607; }
        to_i48 { -2_147_483_648, -140_737_488_355_328; 0, 0; 2_147_483_647, 140_737_488_289_792; }
        to_i64 { -2_147_483_648, -9_223_372_036_854_775_808; 0, 0; 2_147_483_647, 9_223_372_032_559_808_512; }
        to_u8  { -2_147_483_648, 0; 0, 128; 2_147_483_647, 255; }
        to_u16 { -2_147_483_648, 0; 0, 32_768; 2_147_483_647, 65_535; }
        to_u24 { -2_147_483_648, 0; 0, 8_388_608; 2_147_483_647, 16_777_215; }
        to_u32 { -2_147_483_648, 0; 0, 2_147_483_648; 2_147_483_647, 4_294_967_295; }
        to_u48 { -2_147_483_648, 0; 0, 140_737_488_355_328; 2_147_483_647, 281_474_976_645_120; }
        to_u64 { -2_147_483_648, 0; 0, 9_223_372_036_854_775_808; 2_147_483_647, 18_446_744_069_414_584_320; }
        to_f32 { -2_147_483_648, -1.0; 0, 0.0; }
        to_f64 { -2_147_483_648, -1.0; 0, 0.0; }
    });

    tests!(I48: i48 {
        to_i8  { -140_737_488_355_328, -128; 0, 0; 140_737_488_355_327, 127; }
        to_i16 { -140_737_488_355_328, -32_768; 0, 0; 140_737_488_355_327, 32_767; }
        to_i24 { -140_737_488_355_328, -8_388_608; 0, 0; 140_737_488_355_327, 8_388_607; }
        to_i32 { -140_737_488_355_328, -2_147_483_648; 0, 0; 140_737_488_355_327, 2_147_483_647; }
        to_i64 { -140_737_488_355_328, -9_223_372_036_854_775_808; 0, 0; 140_737_488_355_327, 9_223_372_036_854_710_272; }
        to_u8  { -140_737_488_355_328, 0; 0, 128; 140_737_488_355_327, 255; }
        to_u16 { -140_737_488_355_328, 0; 0, 32_768; 140_737_488_355_327, 65_535; }
        to_u24 { -140_737_488_355_328, 0; 0, 8_388_608; 140_737_488_355_327, 16_777_215; }
        to_u32 { -140_737_488_355_328, 0; 0, 2_147_483_648; 140_737_488_355_327, 4_294_967_295; }
        to_u48 { -140_737_488_355_328, 0; 0, 140_737_488_355_328; 140_737_488_355_327, 281_474_976_710_655; }
        to_u64 { -140_737_488_355_328, 0; 0, 9_223_372_036_854_775_808; 140_737_488_355_327, 18_446_744_073_709_486_080; }
    });

    tests!(i64 {
        to_i8  { -9_223_372_036_854_775_808, -128; 0, 0; 9_223_372_036_854_775_807, 127; }
        to_i16 { -9_223_372_036_854_775_808, -32_768; 0, 0; 9_223_372_036_854_775_807, 32_767; }
        to_i24 { -9_223_372_036_854_775_808, -8_388_608; 0, 0; 9_223_372_036_854_775_807, 8_388_607; }
        to_i32 { -9_223_372_036_854_775_808, -2_147_483_648; 0, 0; 9_223_372_036_854_775_807, 2_147_483_647; }
        to_i48 { -9_223_372_036_854_775_808, -140_737_488_355_328; 0, 0; 9_223_372_036_854_775_807, 140_737_488_355_327; }
        to_u8  { -9_223_372_036_854_775_808, 0; 0, 128; 9_223_372_036_854_775_807, 255; }
        to_u16 { -9_223_372_036_854_775_808, 0; 0, 32_768; 9_223_372_036_854_775_807, 65_535; }
        to_u24 { -9_223_372_036_854_775_808, 0; 0, 8_388_608; 9_223_372_036_854_775_807, 16_777_215; }
        to_u32 { -9_223_372_036_854_775_808, 0; 0, 2_147_483_648; 9_223_372_036_854_775_807, 4_294_967_295; }
        to_u48 { -9_223_372_036_854_775_808, 0; 0, 140_737_488_355_328; 9_223_372_036_854_775_807, 281_474_976_710_655; }
        to_u64 { -9_223_372_036_854_775_808, 0; 0, 9_223_372_036_854_775_808; 9_223_372_036_854_775_807, 18_446_744_073_709_551_615; }
        to_f32 { -9_223_372_036_854_775_808, -1.0; 0, 0.0; }
        to_f64 { -9_223_372_036_854_775_808, -1.0; 0, 0.0; }
    });

    tests!(u8 {
        to_i8  { 0, -128; 128, 0; 255, 127; }
        to_i16 { 0, -32_768; 128, 0; 255, 32_512; }
        to_i24 { 0, -8_388_608; 128, 0; 255, 8_323_072; }
        to_i32 { 0, -2_147_483_648; 128, 0; 255, 2_130_706_432; }
        to_i48 { 0, -140_737_488_355_328; 128, 0; 255, 139_637_976_727_552; }
        to_i64 { 0, -9_223_372_036_854_775_808; 128, 0; 255, 9_151_314_442_816_847_872; }
        to_u16 { 0, 0; 128, 32_768; 255, 65_280; }
        to_u24 { 0, 0; 128, 8_388_608; 255, 16_711_680; }
        to_u32 { 0, 0; 128, 2_147_483_648; 255, 4_278_190_080; }
        to_u48 { 0, 0; 128, 140_737_488_355_328; 255, 280_375_465_082_880; }
        to_u64 { 0, 0; 128, 9_223_372_036_854_775_808; 255, 18_374_686_479_671_623_680; }
        to_f32 { 0, -1.0; 128, 0.0; }
        to_f64 { 0, -1.0; 128, 0.0; }
    });

    tests!(u16 {
        to_i8  { 0, -128; 32_768, 0; 65_535, 127; }
        to_i16 { 0, -32_768; 32_768, 0; 65_535, 32_767; }
        to_i24 { 0, -8_388_608; 32_768, 0; 65_535, 8_388_352; }
        to_i32 { 0, -2_147_483_648; 32_768, 0; 65_535, 2_147_418_112; }
        to_i48 { 0, -140_737_488_355_328; 32_768, 0; 65_535, 140_733_193_388_032; }
        to_i64 { 0, -9_223_372_036_854_775_808; 32_768, 0; 65_535, 9_223_090_561_878_065_152; }
        to_u8  { 0, 0; 32_768, 128; 65_535, 255; }
        to_u24 { 0, 0; 32_768, 8_388_608; 65_535, 16_776_960; }
        to_u32 { 0, 0; 32_768, 2_147_483_648; 65_535, 4_294_901_760; }
        to_u48 { 0, 0; 32_768, 140_737_488_355_328; 65_535, 281_470_681_743_360; }
        to_u64 { 0, 0; 32_768, 9_223_372_036_854_775_808; 65_535, 18_446_462_598_732_840_960; }
        to_f32 { 0, -1.0; 32_768, 0.0; }
        to_f64 { 0, -1.0; 32_768, 0.0; }
    });

    tests!(U24: u24 {
        to_i8  { 0, -128; 8_388_608, 0; 16_777_215, 127; }
        to_i16 { 0, -32_768; 8_388_608, 0; 16_777_215, 32_767; }
        to_i24 { 0, -8_388_608; 8_388_608, 0; 16_777_215, 8_388_607; }
        to_i32 { 0, -2_147_483_648; 8_388_608, 0; 16_777_215, 2_147_483_392; }
        to_i48 { 0, -140_737_488_355_328; 8_388_608, 0; 16_777_215, 140_737_471_578_112; }
        to_i64 { 0, -9_223_372_036_854_775_808; 8_388_608, 0; 16_777_215, 9_223_370_937_343_148_032; }
        to_u8  { 0, 0; 8_388_608, 128; 16_777_215, 255; }
        to_u16 { 0, 0; 8_388_608, 32_768; 16_777_215, 65_535; }
        to_u32 { 0, 0; 8_388_608, 2_147_483_648; 16_777_215, 4_294_967_040; }
        to_u48 { 0, 0; 8_388_608, 140_737_488_355_328; 16_777_215, 281_474_959_933_440; }
        to_u64 { 0, 0; 8_388_608, 9_223_372_036_854_775_808; 16_777_215, 18_446_742_974_197_923_840; }
        to_f32 { 0, -1.0; 8_388_608, 0.0; }
        to_f64 { 0, -1.0; 8_388_608, 0.0; }
    });

    tests!(u32 {
        to_i8  { 0, -128; 2_147_483_648, 0; 4_294_967_295, 127; }
        to_i16 { 0, -32_768; 2_147_483_648, 0; 4_294_967_295, 32_767; }
        to_i24 { 0, -8_388_608; 2_147_483_648, 0; 4_294_967_295, 8_388_607; }
        to_i32 { 0, -2_147_483_648; 2_147_483_648, 0; 4_294_967_295, 2_147_483_647; }
        to_i48 { 0, -140_737_488_355_328; 2_147_483_648, 0; 4_294_967_295, 140_737_488_289_792; }
        to_i64 { 0, -9_223_372_036_854_775_808; 2_147_483_648, 0; 4_294_967_295, 9_223_372_032_559_808_512; }
        to_u8  { 0, 0; 2_147_483_648, 128; 4_294_967_295, 255; }
        to_u16 { 0, 0; 2_147_483_648, 32_768; 4_294_967_295, 65_535; }
        to_u24 { 0, 0; 2_147_483_648, 8_388_608; 4_294_967_295, 16_777_215; }
        to_u48 { 0, 0; 2_147_483_648, 140_737_488_355_328; 4_294_967_295, 281_474_976_645_120; }
        to_u64 { 0, 0; 2_147_483_648, 9_223_372_036_854_775_808; 4_294_967_295, 18_446_744_069_414_584_320; }
        to_f32 { 0, -1.0; 2_147_483_648, 0.0; }
        to_f64 { 0, -1.0; 2_147_483_648, 0.0; }
    });

    tests!(U48: u48 {
        to_i8  { 0, -128; 140_737_488_355_328, 0; 281_474_976_710_655, 127; }
        to_i16 { 0, -32_768; 140_737_488_355_328, 0; 281_474_976_710_655, 32_767; }
        to_i24 { 0, -8_388_608; 140_737_488_355_328, 0; 281_474_976_710_655, 8_388_607; }
        to_i32 { 0, -2_147_483_648; 140_737_488_355_328, 0; 281_474_976_710_655, 2_147_483_647; }
        to_i48 { 0, -140_737_488_355_328; 140_737_488_355_328, 0; 281_474_976_710_655, 140_737_488_355_327; }
        to_i64 { 0, -9_223_372_036_854_775_808; 140_737_488_355_328, 0; 281_474_976_710_655, 9_223_372_036_854_710_272; }
        to_u8  { 0, 0; 140_737_488_355_328, 128; 281_474_976_710_655, 255; }
        to_u16 { 0, 0; 140_737_488_355_328, 32_768; 281_474_976_710_655, 65_535; }
        to_u24 { 0, 0; 140_737_488_355_328, 8_388_608; 281_474_976_710_655, 16_777_215; }
        to_u32 { 0, 0; 140_737_488_355_328, 2_147_483_648; 281_474_976_710_655, 4_294_967_295; }
        to_u64 { 0, 0; 140_737_488_355_328, 9_223_372_036_854_775_808; 281_474_976_710_655, 18_446_744_073_709_486_080; }
        to_f32 { 0, -1.0; 140_737_488_355_328, 0.0; }
        to_f64 { 0, -1.0; 140_737_488_355_328, 0.0; }
    });

    tests!(u64 {
        to_i8  { 0, -128; 9_223_372_036_854_775_808, 0; 18_446_744_073_709_551_615, 127; }
        to_i16 { 0, -32_768; 9_223_372_036_854_775_808, 0; 18_446_744_073_709_551_615, 32_767; }
        to_i24 { 0, -8_388_608; 9_223_372_036_854_775_808, 0; 18_446_744_073_709_551_615, 8_388_607; }
        to_i32 { 0, -2_147_483_648; 9_223_372_036_854_775_808, 0; 18_446_744_073_709_551_615, 2_147_483_647; }
        to_i48 { 0, -140_737_488_355_328; 9_223_372_036_854_775_808, 0; 18_446_744_073_709_551_615, 140_737_488_355_327; }
        to_i64 { 0, -9_223_372_036_854_775_808; 9_223_372_036_854_775_808, 0; 18_446_744_073_709_551_615, 9_223_372_036_854_775_807; }
        to_u8  { 0, 0; 9_223_372_036_854_775_808, 128; 18_446_744_073_709_551_615, 255; }
        to_u16 { 0, 0; 9_223_372_036_854_775_808, 32_768; 18_446_744_073_709_551_615, 65_535; }
        to_u24 { 0, 0; 9_223_372_036_854_775_808, 8_388_608; 18_446_744_073_709_551_615, 16_777_215; }
        to_u32 { 0, 0; 9_223_372_036_854_775_808, 2_147_483_648; 18_446_744_073_709_551_615, 4_294_967_295; }
        to_u48 { 0, 0; 9_223_372_036_854_775_808, 140_737_488_355_328; 18_446_744_073_709_551_615, 281_474_976_710_655; }
        to_f32 { 0, -1.0; 9_223_372_036_854_775_808, 0.0; }
        to_f64 { 0, -1.0; 9_223_372_036_854_775_808, 0.0; }
    });

    tests!(f32 {
        to_i8  { -1.0, -128; 0.0, 0; }
        to_i16 { -1.0, -32_768; 0.0, 0; }
        to_i24 { -1.0, -8_388_608; 0.0, 0; }
        to_i32 { -1.0, -2_147_483_648; 0.0, 0; }
        to_i48 { -1.0, -140_737_488_355_328; 0.0, 0; }
        to_i64 { -1.0, -9_223_372_036_854_775_808; 0.0, 0; }
        to_u8  { -1.0, 0; 0.0, 128; }
        to_u16 { -1.0, 0; 0.0, 32_768; }
        to_u24 { -1.0, 0; 0.0, 8_388_608; }
        to_u32 { -1.0, 0; 0.0, 2_147_483_648; }
        to_u48 { -1.0, 0; 0.0, 140_737_488_355_328; }
        to_u64 { -1.0, 0; 0.0, 9_223_372_036_854_775_808; }
        to_f64 { -1.0, -1.0; 0.0, 0.0; }
    });

    tests!(f64 {
        to_i8  { -1.0, -128; 0.0, 0; }
        to_i16 { -1.0, -32_768; 0.0, 0; }
        to_i24 { -1.0, -8_388_608; 0.0, 0; }
        to_i32 { -1.0, -2_147_483_648; 0.0, 0; }
        to_i48 { -1.0, -140_737_488_355_328; 0.0, 0; }
        to_i64 { -1.0, -9_223_372_036_854_775_808; 0.0, 0; }
        to_u8  { -1.0, 0; 0.0, 128; }
        to_u16 { -1.0, 0; 0.0, 32_768; }
        to_u24 { -1.0, 0; 0.0, 8_388_608; }
        to_u32 { -1.0, 0; 0.0, 2_147_483_648; }
        to_u48 { -1.0, 0; 0.0, 140_737_488_355_328; }
        to_u64 { -1.0, 0; 0.0, 9_223_372_036_854_775_808; }
        to_f32 { -1.0, -1.0; 0.0, 0.0; }
    });
}
