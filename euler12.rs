//euler12
//https://projecteuler.net/problem=12

const MAX:u32 = 2147483647;

fn main() {
	let mut triangle = 0;
	let mut i = 1;
	//use a loop because there is no explicit limit
	//i is a counter starting with 1
	loop {
		if MAX - triangle < 1 {
			println!("Overflow");
		}
		triangle = triangle + i;
		if count_divisors(triangle) > 500 {
			println!("{triangle}");
			break;
		}
		i = i + 1;	
	}
}

fn count_divisors(n:u32) -> u32 {
	let mut count:u32 = 0;
	let end: u32 = (n as f64).sqrt() as u32;
	for i in 1 .. end {
		if n % i ==0 {
			count = count + 2;
		}
	}
	if end*end == n {
		//perfect square
		count = count + 1;
	}
	return count;
}