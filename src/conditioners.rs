// Conditioners check the condition of something and act on the result


pub fn run() {
let age = 22;
let check_id: bool = false;
// if else
if age >= 21 && check_id {
    println!("Bartender say: What would like to drink?");
}
else if age < 21 && check_id {
    println!("Sorry you have to leave!");
}

// Shorthand if
let is_of_age = if age >= 21 {true} else {false};
println!("Is of age? {}", is_of_age)
}