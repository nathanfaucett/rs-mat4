use num::Num;


#[inline]
pub fn new<T: Copy + Num>(
    m11: T, m12: T, m13: T, m14: T,
    m21: T, m22: T, m23: T, m24: T,
    m31: T, m32: T, m33: T, m34: T,
    m41: T, m42: T, m43: T, m44: T
) -> [T; 16] {[
    m11, m12, m13, m14,
    m21, m22, m23, m24,
    m31, m32, m33, m34,
    m41, m42, m43, m44
]}
#[inline]
pub fn create<T: Copy + Num>(
    m11: T, m12: T, m13: T, m14: T,
    m21: T, m22: T, m23: T, m24: T,
    m31: T, m32: T, m33: T, m34: T,
    m41: T, m42: T, m43: T, m44: T
) -> [T; 16] {new(
    m11, m12, m13, m14,
    m21, m22, m23, m24,
    m31, m32, m33, m34,
    m41, m42, m43, m44
)}
#[test]
fn test_new() {
    let m = new(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1);
    assert_eq!(m, [
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1
    ]);
}

#[inline]
pub fn new_identity<T: Copy + Num>() -> [T; 16] {
    new(
        T::one(), T::zero(), T::zero(), T::zero(),
        T::zero(), T::one(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::one(), T::zero(),
        T::zero(), T::zero(), T::zero(), T::one()
    )
}
#[inline]
pub fn new_zero<T: Copy + Num>() -> [T; 16] {
    new(
        T::zero(), T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero(), T::zero(),
        T::zero(), T::zero(), T::zero(), T::zero()
    )
}

#[inline]
pub fn clone<'b, T: Copy + Num>(m: &'b [T; 16]) -> [T; 16] {
    new(
        m[0], m[4], m[8], m[12],
        m[1], m[5], m[9], m[13],
        m[2], m[6], m[10], m[14],
        m[3], m[7], m[11], m[15]
    )
}

#[inline]
pub fn copy<'a, 'b, T: Copy + Num>(out: &'a mut [T; 16], a: &'b [T; 16]) -> &'a mut [T; 16] {
    out[0] = a[0];
    out[1] = a[1];
    out[2] = a[2];
    out[3] = a[3];
    out[4] = a[4];
    out[5] = a[5];
    out[6] = a[6];
    out[7] = a[7];
    out[8] = a[8];
    out[9] = a[9];
    out[10] = a[10];
    out[11] = a[11];
    out[12] = a[12];
    out[13] = a[13];
    out[14] = a[14];
    out[15] = a[15];
    out
}
