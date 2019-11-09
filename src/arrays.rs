// Arrays - Fixed list where elements are same data type
// Data Type and length have to be exact

use std::mem;

pub fn run () {
    let mut numbers: [i32; 6] = [1,2,3,4,5,6];


    // Reassign Values
    numbers[2] = 30;

    println!("{:?}", numbers);

    // Get Single Val
    println!("Single Value: {}", numbers[0] );

    // Get Array Length
    println!("Array Length: {}", numbers.len() );

    // Arrays are stack allocated
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..4];
    println!("Slice: {:?}", slice);

    // Arrays are stack allocated
    println!("This slice occupies {} bytes", mem::size_of_val(&slice));



}