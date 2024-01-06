use core::{
    fmt::Binary,
    ops::{Add, AddAssign, BitAnd, BitOr, Div, Rem, Shl, Shr, Sub},
};

pub trait SupportedInteger<
    Signed: Div<Output = Signed>
        + PartialOrd
        + Rem<Output = Signed>
        + Shl<Self, Output = Signed>
        + Shr<Self, Output = Signed>,
>:
    Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + Binary
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + Copy
    + PartialEq
    + PartialOrd
    + Rem<Output = Self>
    + Sized
    + Shl<Output = Self>
    + Shr<Output = Self>
{
    fn zero() -> Self;
    fn one() -> Self;
    fn len() -> Self;
    fn max() -> Self;
    fn to_signed(self) -> Signed;
    fn from_signed(val: Signed) -> Self;
    fn clz(self) -> Self {
        if self == Self::zero() {
            return Self::len();
        } else {
            let mask = Self::one() << (Self::len() - Self::one());
            let mut x = self;
            let mut count = Self::zero();
            while (x & mask) == Self::zero() {
                count += Self::one();
                x = x << Self::one();
            }
            return count;
        }
    }
    fn ctz(self) -> Self {
        if self == Self::zero() {
            return Self::len();
        } else {
            let mask = Self::one();
            let mut x = self;
            let mut count = Self::zero();
            while (x & mask) == Self::zero() {
                count += Self::one();
                x = x >> Self::one();
            }
            return count;
        }
    }
    fn popcnt(self) -> Self {
        if self == Self::zero() {
            return Self::len();
        }
        let mut count = Self::zero();
        for chr in format!("{:b}", self).chars().rev() {
            if chr != '0' {
                break;
            }
            count += Self::one();
        }
        return count;
    }
    fn eqz(self) -> bool {
        Self::eq(&self, &Self::zero())
    }
    fn eq_(val1: Self, val2: Self) -> bool {
        Self::eq(&val1, &val2)
    }
    fn ne_(v1: Self, v2: Self) -> bool {
        v1 != v2
    }
    fn ltu(v1: Self, v2: Self) -> bool {
        v1 < v2
    }
    fn lts(v1: Self, v2: Self) -> bool {
        v1.to_signed() < v2.to_signed()
    }
    fn gtu(v1: Self, v2: Self) -> bool {
        v1 > v2
    }
    fn gts(v1: Self, v2: Self) -> bool {
        v1.to_signed() > v2.to_signed()
    }
    fn leu(v1: Self, v2: Self) -> bool {
        v1 <= v2
    }
    fn les(v1: Self, v2: Self) -> bool {
        v1.to_signed() <= v2.to_signed()
    }
    fn geu(v1: Self, v2: Self) -> bool {
        v1 >= v2
    }
    fn ges(v1: Self, v2: Self) -> bool {
        v1.to_signed() >= v2.to_signed()
    }
    fn div_s(v1: Self, v2: Self) -> Self {
        Self::from_signed(v1.to_signed() / v2.to_signed())
    }
    fn rem_s(v1: Self, v2: Self) -> Self {
        Self::from_signed(v1.to_signed() % v2.to_signed())
    }
    fn shl(v1: Self, v2: Self) -> Self {
        let n = v2 % Self::len();
        v1 << n
    }
    fn shr_u(v1: Self, v2: Self) -> Self {
        let n = v2 % Self::len();
        v1 >> n
    }
    fn shr_s(v1: Self, v2: Self) -> Self {
        let n = v2 % Self::len();
        Self::from_signed(v1.to_signed() >> n)
    }
    fn rotl(v1: Self, v2: Self) -> Self {
        let n = v2 % Self::len();
        (v1 << n) | (v1 >> (Self::len() - n) % Self::len())
    }
    fn rotr(v1: Self, v2: Self) -> Self {
        let n = v2 % Self::len();
        (v1 >> n) | (v1 << (Self::len() - n) % Self::len())
    }
}

impl SupportedInteger<i32> for u32 {
    fn zero() -> u32 {
        0
    }

    fn one() -> Self {
        1
    }

    fn len() -> Self {
        32
    }

    fn max() -> Self {
        u32::MAX
    }

    fn to_signed(self) -> i32 {
        self as i32
    }

    fn from_signed(val: i32) -> Self {
        val as u32
    }
}

impl SupportedInteger<i64> for u64 {
    fn zero() -> u64 {
        0
    }

    fn one() -> Self {
        1
    }

    fn len() -> Self {
        64
    }

    fn max() -> Self {
        u64::MAX
    }

    fn to_signed(self) -> i64 {
        self as i64
    }

    fn from_signed(val: i64) -> Self {
        val as u64
    }
}
