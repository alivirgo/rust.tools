/*
Primitive Types--
Integers: u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 
(number of bits they take in memory)
Floats: f32 f64
Boolean (bool)
Character (char)
Tuples
Arrays
*/


pub fn run() {

    // Default Integer is "i32"
    let x = 1;

    //Default Float is "f64"
    let y = 2.51;

    //Add explicit type
    let z: i64 = 123432423;

    //Find Max Size
    println!("Max i64: {}", std::i64::MAX);
    println!("Max i32: {}", std::i32::MAX);

    //Boolean
    let is_active = true;

    //Get Boolean from Expression
    let is_greator: bool = 10 > 5;


    //Character
    let a1 = 'a';
    let face = '\u{1F600}'; //Emojis are unicode

    println!("{:?}", (x,y,z,is_active,is_greator,a1,face) );
}