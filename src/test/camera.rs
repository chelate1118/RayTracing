#[test]
fn load_camera() {
    use crate::loader;
    use crate::map::camera::Camera;
    use super::assert_eq_vec3;
    use crate::loader::FromValue;

    let camera = Camera::from_value(
        loader::str_to_value(r#"{
            "position": [10.0, 5.0, 3.0],
            "pi": 45.0,
            "theta": 45.0,
            "width": 1200,
            "height": 800,
            "distance": 400.0
        }"#).unwrap()
    ).unwrap();

    assert_eq_vec3!(
        camera.position,
        Vec3::new(10.0, 5.0, 3.0)
    );

    assert_eq_vec3!(
        camera.direction,
        Vec3::new(0.5f32, 0.5f32, 0.5f32.sqrt())
    );

    assert_eq!((camera.width, camera.height), (1200usize, 800usize));
    assert_eq!(camera.distance, 400f32);

    assert_eq_vec3!(
        camera.screen_unit_x,
        Vec3::new(0.5f32.sqrt(), -(0.5f32.sqrt()), 0.0)
    );

    assert_eq_vec3!(
        camera.screen_unit_x,
        Vec3::new(0.5f32.sqrt(), -(0.5f32.sqrt()), 0.0)
    );

    assert_eq_vec3!(
        camera.screen_unit_y,
        Vec3::new(0.5f32, 0.5f32, -(0.5f32.sqrt()))
    )
}

#[test]
fn gen_ray() {

}