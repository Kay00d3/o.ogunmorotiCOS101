use std::io;

fn main() {
    // Step 1: Get values of a, b, and c from user
    let mut input = String::new();

    println!("Enter value of a: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a: f64 = input.trim().parse().expect("Please enter a valid number");


    println!("Enter value of b: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let b: f64 = input.trim().parse().expect("Please enter a valid number");

    println!("Enter value of c: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let c: f64 = input.trim().parse().expect("Please enter a valid number");

    // Step 2: Find discriminant
    let discriminant = b * b - 4.0 * a * c;

    // Step 3: Check the nature of roots
    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: {}", root);
    } else {
        println!("No real roots (the roots are imaginary).");
    }
}
