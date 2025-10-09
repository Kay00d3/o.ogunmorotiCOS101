fn main() {
    let p:f64 = 520_000_000.0;  // principal
    let r:f64 = 10.0;     // interest rate
    let n:f64 = 5.0;     // time in years

    //Amount
    let a = p * ((1.0 + (r / 100.0)).powf(n));

    //Compound Interest
    let cl = a - p;

    println!("Compound Interest is N{}", cl);
}
