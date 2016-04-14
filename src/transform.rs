extern crate vec3;

use num::Num;
use create::{clone, new_identity};


#[inline(always)]
pub fn look_at<T: Num>(out: &mut [T; 16], eye: [T; 3], target: [T; 3], up: [T; 3]) -> &mut [T; 16] {
    let mut x = vec3::create(T::zero(), T::zero(), T::zero());
    let mut y = vec3::create(T::zero(), T::zero(), T::zero());
    let mut z = vec3::create(T::zero(), T::zero(), T::zero());
    let mut tmp = vec3::create(T::zero(), T::zero(), T::zero());

    vec3::sub(&mut z, eye, target);
    vec3::copy(&mut tmp, z);
    vec3::normalize(&mut z, tmp);

    if vec3::length(z) == T::zero() {
        z[2] = T::one();
    }

    vec3::cross(&mut x, up, z);
    vec3::copy(&mut tmp, x);
    vec3::normalize(&mut x, tmp);

    if vec3::length(x) == T::zero() {
        z[0] = z[0] + T::from_f32(0.000001);
        vec3::cross(&mut x, up, z);
        vec3::copy(&mut tmp, x);
        vec3::normalize(&mut x, tmp);
    }

    vec3::cross(&mut y, z, x);

    out[0] = x[0];
    out[4] = y[0];
    out[8] = z[0];
    out[1] = x[1];
    out[5] = y[1];
    out[9] = z[1];
    out[2] = x[2];
    out[6] = y[2];
    out[10] = z[2];
    out
}

#[inline(always)]
pub fn compose<T: Num>(out: &mut [T; 16], position: [T; 3], scale: [T; 3], rotation: [T; 4]) -> &mut [T; 16] {
    let x = rotation[0];
    let y = rotation[1];
    let z = rotation[2];
    let w = rotation[3];
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x * x2;
    let xy = x * y2;
    let xz = x * z2;
    let yy = y * y2;
    let yz = y * z2;
    let zz = z * z2;
    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;

    let sx = scale[0];
    let sy = scale[1];
    let sz = scale[2];

    out[0] = (T::one() - (yy + zz)) * sx;
    out[4] = (xy - wz) * sy;
    out[8] = (xz + wy) * sz;

    out[1] = (xy + wz) * sx;
    out[5] = (T::one() - (xx + zz)) * sy;
    out[9] = (yz - wx) * sz;

    out[2] = (xz - wy) * sx;
    out[6] = (yz + wx) * sy;
    out[10] = (T::one() - (xx + yy)) * sz;

    out[3] = T::zero();
    out[7] = T::zero();
    out[11] = T::zero();

    out[12] = position[0];
    out[13] = position[1];
    out[14] = position[2];
    out[15] = T::one();
    out
}
#[test]
pub fn test_compose() {
    let mut m = [
        1f32, 0f32, 0f32, 0f32,
        0f32, 1f32, 0f32, 0f32,
        0f32, 0f32, 1f32, 0f32,
        0f32, 0f32, 0f32, 1f32
    ];
    let position = [0f32, 0f32, 0f32];
    let scale = [1f32, 1f32, 1f32];
    let rotation = [0f32, 0f32, 0f32, 1f32];
    compose(&mut m, position, scale, rotation);
    assert_eq!(m, [
        1f32, 0f32, 0f32, 0f32,
        0f32, 1f32, 0f32, 0f32,
        0f32, 0f32, 1f32, 0f32,
        0f32, 0f32, 0f32, 1f32
    ]);
}

#[inline(always)]
pub fn decompose<T: Num>(out: [T; 16], position: &mut [T; 3], scale: &mut [T; 3], rotation: &mut [T; 4]) {
    let mut m11 = out[0];
    let mut m12 = out[4];
    let mut m13 = out[8];
    let mut m21 = out[1];
    let mut m22 = out[5];
    let mut m23 = out[9];
    let mut m31 = out[2];
    let mut m32 = out[6];
    let mut m33 = out[10];

    let sx = vec3::length_values(m11, m21, m31);
    let sy = vec3::length_values(m12, m22, m32);
    let sz = vec3::length_values(m13, m23, m33);

    let inv_sx = T::one() / sx;
    let inv_sy = T::one() / sy;
    let inv_sz = T::one() / sz;

    scale[0] = sx;
    scale[1] = sy;
    scale[2] = sz;

    position[0] = out[12];
    position[1] = out[13];
    position[2] = out[14];

    m11 = m11 * inv_sx;
    m12 = m12 * inv_sy;
    m13 = m13 * inv_sz;
    m21 = m21 * inv_sx;
    m22 = m22 * inv_sy;
    m23 = m23 * inv_sz;
    m31 = m31 * inv_sx;
    m32 = m32 * inv_sy;
    m33 = m33 * inv_sz;

    let trace = m11 + m22 + m33;

    if trace > T::zero() {
        let s = T::from_f32(0.5f32) / (trace + T::one()).sqrt();
        let inv_s = if s != T::zero() {T::one() / s} else {s};

        rotation[0] = (m32 - m23) * s;
        rotation[1] = (m13 - m31) * s;
        rotation[2] = (m21 - m12) * s;
        rotation[3] = T::from_f32(0.25f32) * inv_s;
    } else if m11 > m22 && m11 > m33 {
        let s = T::from_isize(2isize) * (T::one() + m11 - m22 - m33).sqrt();
        let inv_s = if s != T::zero() {T::one() / s} else {s};

        rotation[0] = T::from_f32(0.25f32) * s;
        rotation[1] = (m12 + m21) * inv_s;
        rotation[2] = (m13 + m31) * inv_s;
        rotation[3] = (m32 - m23) * inv_s;
    } else if m22 > m33 {
        let s = T::from_isize(2isize) * (T::one() + m22 - m11 - m33).sqrt();
        let inv_s = if s != T::zero() {T::one() / s} else {s};

        rotation[0] = (m12 + m21) * inv_s;
        rotation[1] = T::from_f32(0.25f32) * s;
        rotation[2] = (m23 + m32) * inv_s;
        rotation[3] = (m13 - m31) * inv_s;
    } else {
        let s = T::from_isize(2isize) * (T::one() + m33 - m11 - m22).sqrt();
        let inv_s = if s != T::zero() {T::one() / s} else {s};

        rotation[0] = (m13 + m31) * inv_s;
        rotation[1] = (m23 + m32) * inv_s;
        rotation[2] = T::from_f32(0.25f32) * s;
        rotation[3] = (m21 - m12) * inv_s;
    }
}
#[test]
pub fn test_decompose() {
    let mut position = [0f32, 0f32, 0f32];
    let mut scale = [1f32, 1f32, 1f32];
    let mut rotation = [0f32, 0f32, 0f32, 1f32];
    decompose([
        1f32, 0f32, 0f32, 0f32,
        0f32, 1f32, 0f32, 0f32,
        0f32, 0f32, 1f32, 0f32,
        0f32, 0f32, 0f32, 1f32
    ], &mut position, &mut scale, &mut rotation);
    assert_eq!(position, [0f32, 0f32, 0f32]);
    assert_eq!(scale, [1f32, 1f32, 1f32]);
    assert_eq!(rotation, [0f32, 0f32, 0f32, 1f32]);
}

#[inline(always)]
pub fn set_position<T: Num>(out: &mut [T; 16], v: [T; 3]) -> &mut [T; 16] {
    out[12] = v[0];
    out[13] = v[1];
    out[14] = v[2];
    out
}

#[inline(always)]
pub fn extract_position<T: Num>(out: &mut [T; 16], a: [T; 16]) -> &mut [T; 16] {
    out[12] = a[12];
    out[13] = a[13];
    out[14] = a[14];
    out
}

#[inline(always)]
pub fn extract_rotation<T: Num>(out: &mut [T; 16], a: [T; 16]) -> &mut [T; 16] {
    let sx = vec3::length_values(a[0], a[1], a[2]);
    let sy = vec3::length_values(a[4], a[5], a[6]);
    let sz = vec3::length_values(a[8], a[9], a[10]);

    out[0] = a[0] * sx;
    out[1] = a[1] * sx;
    out[2] = a[2] * sx;
    out[4] = a[4] * sy;
    out[5] = a[5] * sy;
    out[6] = a[6] * sy;
    out[8] = a[8] * sz;
    out[9] = a[9] * sz;
    out[10] = a[10] * sz;
    out
}

#[inline(always)]
pub fn translate<T: Num>(out: &mut [T; 16], a: [T; 16], v: [T; 3]) -> &mut [T; 16] {
    let x = v[0];
    let y = v[1];
    let z = v[2];
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

    out[0] = a00;
    out[1] = a01;
    out[2] = a02;
    out[3] = a03;
    out[4] = a10;
    out[5] = a11;
    out[6] = a12;
    out[7] = a13;
    out[8] = a20;
    out[9] = a21;
    out[10] = a22;
    out[11] = a23;

    out[12] = a00 * x + a10 * y + a20 * z + a[12];
    out[13] = a01 * x + a11 * y + a21 * z + a[13];
    out[14] = a02 * x + a12 * y + a22 * z + a[14];
    out[15] = a03 * x + a13 * y + a23 * z + a[15];
    out
}

#[inline(always)]
pub fn scale<T: Num>(out: &mut [T; 16], a: [T; 16], v: [T; 3]) -> &mut [T; 16] {
    let x = v[0];
    let y = v[1];
    let z = v[2];

    out[0] = a[0] * x;
    out[1] = a[1] * x;
    out[2] = a[2] * x;
    out[3] = a[3] * x;
    out[4] = a[4] * y;
    out[5] = a[5] * y;
    out[6] = a[6] * y;
    out[7] = a[7] * y;
    out[8] = a[8] * z;
    out[9] = a[9] * z;
    out[10] = a[10] * z;
    out[11] = a[11] * z;
    out[12] = a[12];
    out[13] = a[13];
    out[14] = a[14];
    out[15] = a[15];
    out
}

#[inline(always)]
pub fn rotate_x<T: Num>(out: &mut [T; 16], a: [T; 16], angle: T) -> &mut [T; 16] {
    let s = angle.sin();
    let c = angle.cos();
    let a10 = a[4];
    let a11 = a[5];
    let a12 = a[6];
    let a13 = a[7];
    let a20 = a[8];
    let a21 = a[9];
    let a22 = a[10];
    let a23 = a[11];

    out[0] = a[0];
    out[1] = a[1];
    out[2] = a[2];
    out[3] = a[3];
    out[12] = a[12];
    out[13] = a[13];
    out[14] = a[14];
    out[15] = a[15];
    out[4] = a10 * c + a20 * s;
    out[5] = a11 * c + a21 * s;
    out[6] = a12 * c + a22 * s;
    out[7] = a13 * c + a23 * s;
    out[8] = a20 * c - a10 * s;
    out[9] = a21 * c - a11 * s;
    out[10] = a22 * c - a12 * s;
    out[11] = a23 * c - a13 * s;
    out
}

#[inline(always)]
pub fn rotate_y<T: Num>(out: &mut [T; 16], a: [T; 16], angle: T) -> &mut [T; 16] {
    let s = angle.sin();
    let c = angle.cos();
    let a00 = a[0];
    let a01 = a[1];
    let a02 = a[2];
    let a03 = a[3];
    let a20 = a[8];
    let a21 = a[9];
    let a22 = a[10];
    let a23 = a[11];

    out[4] = a[4];
    out[5] = a[5];
    out[6] = a[6];
    out[7] = a[7];
    out[12] = a[12];
    out[13] = a[13];
    out[14] = a[14];
    out[15] = a[15];
    out[0] = a00 * c - a20 * s;
    out[1] = a01 * c - a21 * s;
    out[2] = a02 * c - a22 * s;
    out[3] = a03 * c - a23 * s;
    out[8] = a00 * s + a20 * c;
    out[9] = a01 * s + a21 * c;
    out[10] = a02 * s + a22 * c;
    out[11] = a03 * s + a23 * c;
    out
}

#[inline(always)]
pub fn rotate_z<T: Num>(out: &mut [T; 16], a: [T; 16], angle: T) -> &mut [T; 16] {
    let s = angle.sin();
    let c = angle.cos();
    let a00 = a[0];
    let a01 = a[1];
    let a02 = a[2];
    let a03 = a[3];
    let a10 = a[4];
    let a11 = a[5];
    let a12 = a[6];
    let a13 = a[7];

    out[8] = a[8];
    out[9] = a[9];
    out[10] = a[10];
    out[11] = a[11];
    out[12] = a[12];
    out[13] = a[13];
    out[14] = a[14];
    out[15] = a[15];
    out[0] = a00 * c + a10 * s;
    out[1] = a01 * c + a11 * s;
    out[2] = a02 * c + a12 * s;
    out[3] = a03 * c + a13 * s;
    out[4] = a10 * c - a00 * s;
    out[5] = a11 * c - a01 * s;
    out[6] = a12 * c - a02 * s;
    out[7] = a13 * c - a03 * s;
    out
}

#[inline(always)]
pub fn rotate<T: Num>(out: &mut [T; 16], a: [T; 16], x: T, y: T, z: T) -> &mut [T; 16] {
    let mut tmp_a = clone(a);
    let mut tmp_b = new_identity();
    rotate_z(&mut tmp_a, a, z);
    rotate_x(&mut tmp_b, tmp_a, x);
    rotate_y(out, tmp_b, y);
    out
}

#[inline(always)]
pub fn from_quat<T: Num>(out: &mut [T; 16], q: [T; 4]) -> &mut [T; 16] {
    let x = q[0];
    let y = q[1];
    let z = q[2];
    let w = q[3];
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x * x2;
    let xy = x * y2;
    let xz = x * z2;
    let yy = y * y2;
    let yz = y * z2;
    let zz = z * z2;
    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;

    out[0] = T::one() - (yy + zz);
    out[4] = xy - wz;
    out[8] = xz + wy;

    out[1] = xy + wz;
    out[5] = T::one() - (xx + zz);
    out[9] = yz - wx;

    out[2] = xz - wy;
    out[6] = yz + wx;
    out[10] = T::one() - (xx + yy);

    out[3] = T::zero();
    out[7] = T::zero();
    out[11] = T::zero();

    out[12] = T::zero();
    out[13] = T::zero();
    out[14] = T::zero();
    out[15] = T::one();
    out
}

#[inline(always)]
pub fn frustum<T: Num>(out: &mut [T; 16], left: T, right: T, top: T, bottom: T, near: T, far: T) -> &mut [T; 16] {
    let x = T::from_isize(2isize) * near / (right - left);
    let y = T::from_isize(2isize) * near / (top - bottom);
    let a = (right + left) / (right - left);
    let b = (top + bottom) / (top - bottom);
    let c = -(far + near) / (far - near);
    let d = -T::from_isize(2isize) * far * near / (far - near);

    out[0] = x;
    out[4] = T::zero();
    out[8] = a;
    out[12] = T::zero();
    out[1] = T::zero();
    out[5] = y;
    out[9] = b;
    out[13] = T::zero();
    out[2] = T::zero();
    out[6] = T::zero();
    out[10] = c;
    out[14] = d;
    out[3] = T::zero();
    out[7] = T::zero();
    out[11] = -T::one();
    out[15] = T::zero();
    out
}

#[inline(always)]
pub fn perspective<T: Num>(out: &mut [T; 16], fov: T, aspect: T, near: T, far: T) -> &mut [T; 16] {
    let ymax = near * (fov / T::from_isize(2isize)).tan();
    let ymin = -ymax;
    let xmin = ymin * aspect;
    let xmax = ymax * aspect;
    frustum(out, xmin, xmax, ymax, ymin, near, far)
}

#[inline(always)]
pub fn orthographic<T: Num>(out: &mut [T; 16], left: T, right: T, top: T, bottom: T, near: T, far: T) -> &mut [T; 16] {
    let w = right - left;
    let h = top - bottom;
    let p = far - near;
    let x = (right + left) / w;
    let y = (top + bottom) / h;
    let z = (far + near) / p;

    out[0] = T::from_isize(2isize) / w;
    out[1] = T::zero();
    out[2] = T::zero();
    out[3] = T::zero();
    out[4] = T::zero();
    out[5] = T::from_isize(2isize) / h;
    out[6] = T::zero();
    out[7] = T::zero();
    out[8] = T::zero();
    out[9] = T::zero();
    out[10] = -T::from_isize(2isize) / p;
    out[11] = T::zero();
    out[12] = -x;
    out[13] = -y;
    out[14] = -z;
    out[15] = T::one();
    out
}
