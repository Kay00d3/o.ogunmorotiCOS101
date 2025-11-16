use std::io;

fn main() {
    loop {
        println!(" AREA & VOLUME CALCULATOR ");
        println!("Choose an option:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");
        println!("6. Exit");

        let choice = input("Enter your choice: ")
            .trim()
            .parse::<u32>()
            .unwrap_or(0);

        match choice {
            1 => calc_trapezium(),
            2 => calc_rhombus(),
            3 => calc_parallelogram(),
            4 => calc_cube(),
            5 => calc_cylinder(),
            6 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}



fn input(prompt: &str) -> String {
    let mut value = String::new();
    print!("{}", prompt);
    let _ = io::Write::flush(&mut io::stdout());
    io::stdin().read_line(&mut value).expect("Failed input");
    value
}

fn calc_trapezium() {
    let h = input("Enter height: ").trim().parse::<f32>().unwrap();
    let b1 = input("Enter base 1: ").trim().parse::<f32>().unwrap();
    let b2 = input("Enter base 2: ").trim().parse::<f32>().unwrap();

    let area = (h / 2.0) * (b1 + b2);
    println!("Area of trapezium = {}", area);
}

fn calc_rhombus() {
    let d1 = input("Enter diagonal 1: ").trim().parse::<f32>().unwrap();
    let d2 = input("Enter diagonal 2: ").trim().parse::<f32>().unwrap();

    let area = 0.5 * d1 * d2;
    println!("Area of rhombus = {}", area);
}

fn calc_parallelogram() {
    let base = input("Enter base: ").trim().parse::<f32>().unwrap();
    let height = input("Enter altitude: ").trim().parse::<f32>().unwrap();

    let area = base * height;
    println!("Area of parallelogram = {}", area);
}

fn calc_cube() {
    let side = input("Enter side length: ").trim().parse::<f32>().unwrap();

    let area = 6.0 * side * side;
    println!("Area of cube = {}", area);
}

fn calc_cylinder() {
    let radius = input("Enter radius: ").trim().parse::<f32>().unwrap();
    let height = input("Enter height: ").trim().parse::<f32>().unwrap();

    let volume = std::f32::consts::PI * radius * radius * height;
    println!("Volume of cylinder = {}", volume);
}
