// Functions are used to store blocks of code for reuse in Rust Language

pub fn run() {
    greeting("Hello", "Jane");

    // Bind functions values to variables in Rust Language
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);
}
    // Closure Function in Rust Language
    let n3: i32 = 10;
    let add_nums = |nq:i32, n2:i32| n2 + n3 + n3;
    println!("Closure sum: {}", add(3, 3));
fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1:i32, n2:i32) -> i32 {
    n1 + n2
}