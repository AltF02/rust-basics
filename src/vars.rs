pub fn run() {
    let name = "Matthew";
    let mut age = 16;

    println!("My name is {} and I am {}", name, age);
    age = 17;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);


    // Assign multiple vars
    let ( my_name, my_age ) = ("Matthew", 16);
    println!("{} is {}", my_name, my_age);
}