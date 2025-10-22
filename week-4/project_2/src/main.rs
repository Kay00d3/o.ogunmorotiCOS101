use std::io;

fn main() {
    // Get experience status
    let mut input = String::new();
    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let experience = input.trim().to_lowercase();

    // Get age
    println!("Enter the age of the employee: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let age: u32 = input.trim().parse().expect("Please enter a valid number");

    // Determine incentive
    if experience == "yes" {
        if age >= 40 {
            println!("Incentive: ₦1,560,000 per year");
        } else if age >= 30 && age < 40 {
            println!("Incentive: ₦1,480,000 per year");
        } else if age < 28 {
            println!("Incentive: ₦1,300,000 per month");
        } else {
            println!("No specific incentive for this age range.");
        }
    } else {
        println!("Incentive: ₦100,000 per year");
    }
}
