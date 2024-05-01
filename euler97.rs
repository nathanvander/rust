//problem from https://projecteuler.net/problem=97
const MOD:u128 = 10_000_000_000;	//10 billion

fn main() {
	println!("What are the last 10 digits of 28433 x 2^7830457 + 1?");
	let x:u128 = mod_pow(2, 7830457, MOD);
	let y:u128 = (28433 * x + 1) % MOD;
	println!("{y}");
}

//from https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 { return 0; }
    let mut result:u128 = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus;
    }
    result
}
