fn main() {
    let p:f64 = 510_000.0;  // principal
    let r:f64 = 5.0;     // interest rate
    let y = 3;     // time in years

    //Amount
    let a = p * (1.0 - r / 100.0).powf(y as f64);

    println!("The value of the TV after {} years is {:.2}",y,a);
}
