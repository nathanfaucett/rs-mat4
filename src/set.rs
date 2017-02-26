use num::Num;


#[inline]
pub fn set<'a, 'b, T: Copy + Num>(
    out: &'a mut [T; 16],
    m11: T, m12: T, m13: T, m14: T,
    m21: T, m22: T, m23: T, m24: T,
    m31: T, m32: T, m33: T, m34: T,
    m41: T, m42: T, m43: T, m44: T
) -> &'a mut [T; 16] {
    out[0] = m11;
    out[4] = m12;
    out[8] = m13;
    out[12] = m14;
    out[1] = m21;
    out[5] = m22;
    out[9] = m23;
    out[13] = m24;
    out[2] = m31;
    out[6] = m32;
    out[10] = m33;
    out[14] = m34;
    out[3] = m41;
    out[7] = m42;
    out[11] = m43;
    out[15] = m44;
    out
}

#[inline]
pub fn zero<'a, 'b, T: Copy + Num>(out: &'a mut [T; 16]) -> &'a mut [T; 16] {
    set(out,
        T::zero(), T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero(), T::zero()
    )
}
#[inline]
pub fn identity<'a, 'b, T: Copy + Num>(out: &'a mut [T; 16]) -> &'a mut [T; 16] {
    set(out,
        T::one(), T::zero(), T::zero(), T::zero(),
        T::zero(), T::one(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::one(), T::zero(),
        T::zero(), T::zero(), T::zero(), T::one()
    )
}

#[inline]
pub fn from_mat2<'a, 'b, T: Copy + Num>(out: &'a mut [T; 16], m: &'b [T; 4]) -> &'a mut [T; 16] {
    set(
        out,
        m[0], m[2], T::zero(), T::zero(),
        m[1], m[3], T::zero(), T::zero(),
        T::zero(), T::zero(), T::one(), T::zero(),
        T::zero(), T::zero(), T::zero(), T::one()
    )
}
#[inline]
pub fn from_mat32<'a, 'b, T: Copy + Num>(out: &'a mut [T; 16], m: &'b [T; 6]) -> &'a mut [T; 16] {
    set(
        out,
        m[0], m[2], T::zero(), m[4],
        m[1], m[3], T::zero(), m[5],
        T::zero(), T::zero(), T::one(), T::zero(),
        T::zero(), T::zero(), T::zero(), T::one()
    )
}
#[inline]
pub fn from_mat3<'a, 'b, T: Copy + Num>(out: &'a mut [T; 16], m: &'b [T; 9]) -> &'a mut [T; 16] {
    set(
        out,
        m[0], m[3], m[6], T::zero(),
        m[1], m[4], m[7], T::zero(),
        m[2], m[5], m[8], T::zero(),
        T::zero(), T::zero(), T::zero(), T::one()
    )
}
