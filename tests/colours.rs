// This file is part of "linbra"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

#[test]
fn no_implementation() {
    use linbra::vector::Vector3;
 
    let colour: Vector3<u8> = [255, 100, 100].into();
 
    let red = colour[0];
    assert_eq!(255, red);
}

#[test]
fn implementation() {
    use linbra::{
        vector::Vector3,
        colours::RGB,
    };
 
    let colour: Vector3<u8> = [255, 100, 100].into();
 
    let red = colour.r();
    assert_eq!(255, red);
}
