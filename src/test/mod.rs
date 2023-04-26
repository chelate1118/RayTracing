mod ray;
mod sphere;
mod camera;

#[allow(unused_macros)]
macro_rules! assert_eq_vec3 {
    ($v1: expr, $v2: expr) => {
        {
            use glam::Vec3;

            const APPROXIMATE_VALUE: f32 = 1e-6;

            assert!(Vec3::distance($v1, $v2) < APPROXIMATE_VALUE);
        }
    };
}

#[cfg(test)]
pub(crate) use assert_eq_vec3;