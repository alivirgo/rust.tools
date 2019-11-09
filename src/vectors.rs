// Arrays - Fixed list where elements are same data type
// Data Type and length have to be exact

use std::mem;

pub fn run () {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5,6];


    // Reassign Values
    numbers[2] = 30;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last valuee
    numbers.pop();


    println!("{:?}", numbers);

    // Get Single Val
    println!("Single Value: {}", numbers[0] );

    // Get Vectors Length
    println!("Vector Length: {}", numbers.len() );

    // Vectors are stack allocated
    println!("This vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..4];
    println!("Slice: {:?}", slice);

    // Arrays are stack allocated
    println!("This slice occupies {} bytes", mem::size_of_val(&slice));

    //Loop thorugh Vector values
    for x in numbers.iter() {
        println!("Number: {}", x);

    }


    // Loop and mutate values Rust
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers );

}