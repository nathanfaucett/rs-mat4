use num::Unsigned;
use approx::Approx;
use set::identity;


#[inline(always)]
pub fn inverse<'a, 'b, T: Unsigned>(out: &'a mut [T; 16], a: &'b [T; 16]) -> &'a mut [T; 16] {
    let a00 = a[0];
    let a01 = a[1];
    let a02 = a[2];
    let a03 = a[3];
    let a10 = a[4];
    let a11 = a[5];
    let a12 = a[6];
    let a13 = a[7];
    let a20 = a[8];
    let a21 = a[9];
    let a22 = a[10];
    let a23 = a[11];
    let a30 = a[12];
    let a31 = a[13];
    let a32 = a[14];
    let a33 = a[15];

    let b00 = a00 * a11 - a01 * a10;
    let b01 = a00 * a12 - a02 * a10;
    let b02 = a00 * a13 - a03 * a10;
    let b03 = a01 * a12 - a02 * a11;
    let b04 = a01 * a13 - a03 * a11;
    let b05 = a02 * a13 - a03 * a12;
    let b06 = a20 * a31 - a21 * a30;
    let b07 = a20 * a32 - a22 * a30;
    let b08 = a20 * a33 - a23 * a30;
    let b09 = a21 * a32 - a22 * a31;
    let b10 = a21 * a33 - a23 * a31;
    let b11 = a22 * a33 - a23 * a32;

    let d = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

    if d != T::zero() {
        let inv_d = T::one() / d;

        out[0] = (a11 * b11 - a12 * b10 + a13 * b09) * inv_d;
        out[1] = (a02 * b10 - a01 * b11 - a03 * b09) * inv_d;
        out[2] = (a31 * b05 - a32 * b04 + a33 * b03) * inv_d;
        out[3] = (a22 * b04 - a21 * b05 - a23 * b03) * inv_d;
        out[4] = (a12 * b08 - a10 * b11 - a13 * b07) * inv_d;
        out[5] = (a00 * b11 - a02 * b08 + a03 * b07) * inv_d;
        out[6] = (a32 * b02 - a30 * b05 - a33 * b01) * inv_d;
        out[7] = (a20 * b05 - a22 * b02 + a23 * b01) * inv_d;
        out[8] = (a10 * b10 - a11 * b08 + a13 * b06) * inv_d;
        out[9] = (a01 * b08 - a00 * b10 - a03 * b06) * inv_d;
        out[10] = (a30 * b04 - a31 * b02 + a33 * b00) * inv_d;
        out[11] = (a21 * b02 - a20 * b04 - a23 * b00) * inv_d;
        out[12] = (a11 * b07 - a10 * b09 - a12 * b06) * inv_d;
        out[13] = (a00 * b09 - a01 * b07 + a02 * b06) * inv_d;
        out[14] = (a31 * b01 - a30 * b03 - a32 * b00) * inv_d;
        out[15] = (a20 * b03 - a21 * b01 + a22 * b00) * inv_d;
        out
    } else {
        identity(out)
    }
}
#[test]
fn test_inverse() {
    let mut v = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];
    inverse(&mut v, &[1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
    assert!(v == [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
}

#[inline(always)]
pub fn determinant<'a, 'b, T: Unsigned>(out: &'b [T; 16]) -> T {
    let a00 = out[0];
    let a01 = out[1];
    let a02 = out[2];
    let a03 = out[3];
    let a10 = out[4];
    let a11 = out[5];
    let a12 = out[6];
    let a13 = out[7];
    let a20 = out[8];
    let a21 = out[9];
    let a22 = out[10];
    let a23 = out[11];
    let a30 = out[12];
    let a31 = out[13];
    let a32 = out[14];
    let a33 = out[15];

    let b00 = a00 * a11 - a01 * a10;
    let b01 = a00 * a12 - a02 * a10;
    let b02 = a00 * a13 - a03 * a10;
    let b03 = a01 * a12 - a02 * a11;
    let b04 = a01 * a13 - a03 * a11;
    let b05 = a02 * a13 - a03 * a12;
    let b06 = a20 * a31 - a21 * a30;
    let b07 = a20 * a32 - a22 * a30;
    let b08 = a20 * a33 - a23 * a30;
    let b09 = a21 * a32 - a22 * a31;
    let b10 = a21 * a33 - a23 * a31;
    let b11 = a22 * a33 - a23 * a32;

    b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06
}
#[test]
fn test_determinant() {
    assert_eq!(determinant(&[1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]), 1);
}

#[inline(always)]
pub fn transpose<'a, 'b, T: Unsigned>(out: &'a mut [T; 16], a: &'b [T; 16]) -> &'a mut [T; 16] {
    out[0] = a[0];
    out[1] = a[4];
    out[2] = a[8];
    out[3] = a[12];
    out[4] = a[1];
    out[5] = a[5];
    out[6] = a[9];
    out[7] = a[13];
    out[8] = a[2];
    out[9] = a[6];
    out[10] = a[10];
    out[11] = a[14];
    out[12] = a[3];
    out[13] = a[7];
    out[14] = a[11];
    out[15] = a[15];
    out
}
#[test]
fn test_transpose() {
    let mut v = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];
    transpose(&mut v, &[1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
    assert_eq!(v, [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
}

#[inline(always)]
pub fn eq<'a, T: Unsigned>(a: &'a [T; 16], b: &'a [T; 16]) -> bool {
    !ne(a, b)
}

#[inline(always)]
pub fn ne<'a, T: Unsigned>(a: &'a [T; 16], b: &'a [T; 16]) -> bool {
    !a[0].approx_eq(b[0]) ||
    !a[1].approx_eq(b[1]) ||
    !a[2].approx_eq(b[2]) ||
    !a[3].approx_eq(b[3]) ||
    !a[4].approx_eq(b[4]) ||
    !a[5].approx_eq(b[5]) ||
    !a[6].approx_eq(b[6]) ||
    !a[7].approx_eq(b[7]) ||
    !a[8].approx_eq(b[8]) ||
    !a[9].approx_eq(b[9]) ||
    !a[10].approx_eq(b[10]) ||
    !a[11].approx_eq(b[11]) ||
    !a[12].approx_eq(b[12]) ||
    !a[13].approx_eq(b[13]) ||
    !a[14].approx_eq(b[14]) ||
    !a[15].approx_eq(b[15])
}
#[test]
fn test_ne() {
    assert_eq!(ne(
        &[1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32],
        &[1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32]),
        false
    );
    assert_eq!(ne(
        &[0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32],
        &[1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32]),
        true
    );
}
