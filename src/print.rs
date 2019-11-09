pub fn run() {
    //Print to console
    println!("Hello from the print.rs file");

    //Placeholder
    println!("Number: {}", 1);

    //Basic Formating
    println!("{} is from {}", "Brad", "Mass");

    //Positional Arguments
    println!("{0} is from {1} and {0} loves to {2}", "Brad", "Mass", "code");

    //Named Arguments
    println!("{name} likes to play {activity}", name="Brad", activity="Baseball");

    //Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octo: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    //Basic Math
    println!("10+10={}", 10 + 10);

}