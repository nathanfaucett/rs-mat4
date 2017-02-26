use num::Num;


#[inline]
pub fn sdiv<'a, 'b, T: Copy + Num>(out: &'a mut [T; 16], a: &'b [T; 16], s: T) ->  &'a mut [T; 16] {
    let not_zero = s != T::zero();
    out[0] = if not_zero {a[0] / s} else  {T::zero()};
    out[1] = if not_zero {a[1] / s} else  {T::zero()};
    out[2] = if not_zero {a[2] / s} else  {T::zero()};
    out[3] = if not_zero {a[3] / s} else  {T::zero()};
    out[4] = if not_zero {a[4] / s} else  {T::zero()};
    out[5] = if not_zero {a[5] / s} else  {T::zero()};
    out[6] = if not_zero {a[6] / s} else  {T::zero()};
    out[7] = if not_zero {a[7] / s} else  {T::zero()};
    out[8] = if not_zero {a[8] / s} else  {T::zero()};
    out[9] = if not_zero {a[9] / s} else  {T::zero()};
    out[10] = if not_zero {a[10] / s} else  {T::zero()};
    out[11] = if not_zero {a[11] / s} else  {T::zero()};
    out[12] = if not_zero {a[12] / s} else  {T::zero()};
    out[13] = if not_zero {a[13] / s} else  {T::zero()};
    out[14] = if not_zero {a[14] / s} else  {T::zero()};
    out[15] = if not_zero {a[15] / s} else  {T::zero()};
    out
}
#[test]
fn test_sdiv() {
    let mut v = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];
    sdiv(&mut v, &[1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1], 1);
    assert!(v == [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);
}
