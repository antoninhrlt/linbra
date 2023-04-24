// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

#[test]
fn same_getters() {
    use linbra::{
        colours::RGB,
        points::{ Point2, Point3 },
        vector::Vector3,
    };

    let vec3 = Vector3::<u8>::new([10, 5, 2]);
    assert_eq!(10, vec3.x());
    assert_eq!(5, vec3.y());
    assert_eq!(2, vec3.z());
    
    assert_eq!(10, vec3.r());
}

#[test]
fn same_into() {
    use linbra::{
        colours::RGB,
        points::{ Point2, Point3 },
        vector::Vector3,
    };

    let vec3: Vector3<u8> = [10, 5, 2].into();
    assert_eq!(10, vec3.x());
    assert_eq!(5, vec3.y());
    assert_eq!(2, vec3.z());
    
    assert_eq!(10, vec3.r());
}
