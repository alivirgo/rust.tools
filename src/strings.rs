//Primitive str = immutable fixed length string somewhere in memory
// String = Growable, heap allocated data structure - Used when data is modified



pub fn run() {
    let hello = "Hello";
    let mut hello1 = String::from("Hello ");
    println!("{}", hello );

    // Get Length
    println!("Length: {}", hello.len());

    // Push one character to the end
    hello1.push('W');

    // Push string to the end
    hello1.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {} and {}", hello1.capacity(), hello1.capacity());
    
    // Check if String is Empty
    println!("Is Empty: {}", hello1.is_empty());
    

    //Contains
    println!("Contains 'World: {}", hello1.contains("World"));

    // Replace
    println!("Replace {}", hello1.replace("World", "There") );
    
    // Loop through String by whitespace
    for word in hello1.split_whitespace() {
        println!("{}", word);
    }

    // Create String with Capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("New: {}", s);

    //Assertion Testing
    assert_eq!(2,  s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello1 );
}