#[test]
fn test_reflect() {
    let normal = Vec3::Z;
    let input = Vec3::new(1.0, 1.0, 2.0);

    assert_eq!(input.reflect_from(normal), Vec3::new(-1.0, -1.0, 2.0));
}