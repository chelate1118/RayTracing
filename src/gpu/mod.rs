use std::ffi::c_int;

#[link(name = "add_array", kind = "static")]
extern "C" {
    fn add_array_int(arr1: *const c_int, arr2: *const c_int, length: usize) -> *mut c_int;
}

pub(crate) fn add_array_i32(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let length = arr1.len();

    if length != arr2.len() {
        panic!("Can't add two arrays of differenct sizes.");
    }

    let sum;

    unsafe {
        let sum_ptr = add_array_int(arr1.as_ptr(), arr2.as_ptr(), length);
        sum = std::slice::from_raw_parts(sum_ptr, length);
    }

    sum.to_vec()
}