use std::io;

fn main() {
    let mut total = 0;

    loop {
        println!("        MENU (Rust Cafe)");
        println!("P = Poundo Yam/Edinkaiko Soup  - ₦3,200");
        println!("F = Fried Rice & Chicken        - ₦3,000");
        println!("A = Amala & Ewedu Soup          - ₦2,500");
        println!("E = Eba & Egusi Soup            - ₦2,000");
        println!("W = White Rice & Stew           - ₦2,500");
        println!("Q = Quit");

        println!("Enter your choice:");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        let choice = choice.trim().to_uppercase();

        if choice == "Q" {
            println!("Your total bill is ₦{}", total);
            println!("Thank you for your order! Goodbye!");
            break;
        } else if choice == "P" {
            println!("You ordered Poundo Yam/Edinkaiko Soup - ₦3,200");
            total += 3200;
        } else if choice == "F" {
            println!("You ordered Fried Rice & Chicken - ₦3,000");
            total += 3000;
        } else if choice == "A" {
            println!("You ordered Amala & Ewedu Soup - ₦2,500");
            total += 2500;
        } else if choice == "E" {
            println!("You ordered Eba & Egusi Soup - ₦2,000");
            total += 2000;
        } else if choice == "W" {
            println!("You ordered White Rice & Stew - ₦2,500");
            total += 2500;
        } else {
            println!("Invalid choice! Please try again.");
        }
    }
}
