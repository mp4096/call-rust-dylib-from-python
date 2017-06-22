mod my_math;

use std::slice;
use std::mem;

#[no_mangle]
pub extern "C" fn scalar_sum(a: f64, b: f64) -> f64 {
    my_math::scalar_sum(a, b)
}

#[no_mangle]
pub unsafe extern "C" fn vec_sum(arr: *const f64, len: usize) -> f64 {
    assert!(!arr.is_null());
    my_math::vec_sum(slice::from_raw_parts(arr, len))
}

#[no_mangle]
pub unsafe extern "C" fn vec_mean(arr: *const f64, len: usize) -> f64 {
    assert!(!arr.is_null());
    my_math::vec_mean(slice::from_raw_parts(arr, len))
}

#[no_mangle]
pub unsafe extern "C" fn vec_cumsum(arr: *const f64, len: usize) -> *const f64 {
    assert!(!arr.is_null());
    let cumsum = my_math::vec_cumsum(slice::from_raw_parts(arr, len));
    let ptr_cumsum = cumsum.as_ptr();
    mem::forget(cumsum);
    ptr_cumsum
}

#[no_mangle]
pub unsafe extern "C" fn vec_cumsum_mut(arr: *const f64, out: *mut f64, len: usize) {
    assert!(!arr.is_null());
    assert!(!out.is_null());
    let slice_arr = slice::from_raw_parts(arr, len);
    let mut slice_out = slice::from_raw_parts_mut(out, len);
    my_math::vec_cumsum_mut(slice_arr, slice_out);
}
