mod vec3;
use vec3::Vec3;

#[test]
fn addition_of_vectors() {
    let vec1 = Vec3::new(1.0, 2.0, 3.0);
    let vec2 = Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(vec1+vec2, Vec3::new(3.0, 5.0, 7.0));
}

#[test]
fn subtraction_of_vectors() {
    let vec1 = Vec3::new(1.0, 2.0, 3.0);
    let vec2 = Vec3::new(2.0, 3.0, 4.0);
    assert_eq!(vec1-vec2, Vec3::new(-1.0, -1.0, -1.0));
}

#[test]
fn negative_of_vector() {
    let vec1 = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(-vec1, Vec3::new(-1.0, -2.0, -3.0));
}