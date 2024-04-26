/* 
 * Solution to Project Euler problem 197
 * Copyright (c) Project Nayuki. All rights reserved.
 * 
 * https://www.nayuki.io/page/project-euler-solutions
 * https://github.com/nayuki/Project-Euler-solutions
 */

fn main() {
	println!("{}",run());
}

fn run() -> f64 {
	const ITERATIONS:u64 = 1000000000000; //12 zeros
	const F9:f64 = 1_000_000_000.0;
	let mut x:f64 = -1.0;
	let mut y:f64 = -1.0;
	
	let mut i:u64 = 0;
	while i < ITERATIONS {
		if i>0 && x == y {
			break;
		}
		x = f197(x);
		y = f197( f197(y));		
		i = i + 1;
	}
	
	let mut remain:u64 = (ITERATIONS - i) % i;
	while remain > 0 {
		x = f197(x);
		remain = remain - 1;
	}
	let mut answer:f64 = x + f197(x);
	answer = (answer * F9).floor() / F9;
	return answer;
}

//I call this f197 because it is described in problem 197
fn f197(x:f64) -> f64 {
	const F9:f64 = 1_000_000_000.0;
	let base:f64 = 2.0;
	let a:f64 = base.powf(30.403243784 - x * x);
	return a.floor() / F9;
}