use std::io;

fn main() {
    println!("\n Student Information Manangement System!");

    println!("\n Please Enter your name.");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("Faile to read input");
    println!("Your name is: {}",name );

    println!("Enter your age.");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Input not an integer");
    println!("Your age is: {}",age );
    
}
