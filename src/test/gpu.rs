#![allow(unused)]
use rand::Rng;

use crate::gpu::add_array_i32;

#[test]
fn add_array() {
    let mut rng = rand::thread_rng();
    let length = 1usize<<20;

    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();

    for i in 0..length {
        arr1.push(rng.gen::<i32>() / 100);
        arr2.push(rng.gen::<i32>() / 100);
    }

    let sum = add_array_i32(&arr1, &arr2);

    for i in 0..length {
        assert_eq!(sum[i], arr1[i] + arr2[i]);
    }
}