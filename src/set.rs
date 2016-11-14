use num::Num;


#[inline(always)]
pub fn set<'a, 'b, T: Num>(
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

#[inline(always)]
pub fn zero<'a, 'b, T: Num>(out: &'a mut [T; 16]) -> &'a mut [T; 16] {
    set(out,
        T::zero(), T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero(), T::zero()
    )
}
#[inline(always)]
pub fn identity<'a, 'b, T: Num>(out: &'a mut [T; 16]) -> &'a mut [T; 16] {
    set(out,
        T::one(), T::zero(), T::zero(), T::zero(),
        T::zero(), T::one(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::one(), T::zero(),
        T::zero(), T::zero(), T::zero(), T::one()
    )
}
