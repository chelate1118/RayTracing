#[test]
fn test_load_camera() {
    use::glam::Vec3;
    use crate::camera::Camera;

    const APPROXIMATE_VALUE: f32 = 1e-6;

    let camera = Camera::from_str(r#"{
        "x": 10.0,
        "y": 5.0,
        "z": 3.0,
        "pi": 45.0,
        "theta": 45.0,
        "width": 1200,
        "height": 800
    }"#);

    assert!(
        Vec3::distance(
            camera.position, Vec3::new(10.0, 5.0, 3.0)
        ) < APPROXIMATE_VALUE
    );

    assert!(
        Vec3::distance(
            camera.direction, Vec3::new(0.5f32, 0.5f32, 0.5f32.sqrt())
        ) < APPROXIMATE_VALUE
    );

    assert_eq!((camera.width, camera.height), (1200usize, 800usize));
}
