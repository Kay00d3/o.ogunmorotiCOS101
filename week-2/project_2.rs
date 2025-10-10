fn main() {
	let q = 2 + 1 + 3 + 3 + 1;
	let a = (2 * 450_000) + 1_500_000 + (3 * 750_000) + (3 * 2_850_000) + 250_000;
	let s = q * a;
	let avg = s / q;

	println!("The sum is N{}.",s);
	println!("The average is N{}",avg);
}