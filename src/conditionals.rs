// Conditionals - Used to check the condition of something and act on the results

pub fn run() {
    let age: u8 = 16;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    // If/Else
    if age >= 18 && check_id || knows_person_of_age {
        println!("Bartender: What do you like to drink?");
    } else if age <= 16 && check_id {
        println!("Bartender: Sorry, you have to be at least 18");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthand if
    let is_of_age = if age >= 18 { true } else { false };
    println!("Is Of Age: {}", is_of_age);
}