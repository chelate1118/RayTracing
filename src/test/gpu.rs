#![allow(unused)]
use rand::Rng;

use crate::gpu::add_array_i32;

#[test]
fn add_array() {
    let mut rng = rand::thread_rng();
    let length = 1048576usize;

    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();

    for i in 0..length {
        arr1.push(i as i32);
        arr2.push(i as i32 + 1);
    }

    let s = std::time::Instant::now();
    let mut sum = Vec::<i32>::new();

    for i in 0..length {
        sum.push(arr1[i] + arr2[i]);
    }

    println!("{}", s.elapsed().as_millis());

    let sum = add_array_i32(&arr1, &arr2);

    for i in 0..length {
        assert_eq!(sum[i], arr1[i] + arr2[i]);
    }
}