// Project Euler Problem 50
// from https://projecteuler.net/problem=50
//
// Problem statement:
// The prime 41, can be written as the sum of six consecutive primes:
//	41 = 2 + 3 + 5 + 7 + 11 + 13
// This is the longest sum of consecutive primes that adds to a prime below one-hundred.
// The longest sum of consecutive primes below one-thousand that adds to a prime, contains 
// 21 terms, and is equal to 953.
// Which prime, below one-million, can be written as the sum of the most consecutive primes?
//
// Note that this is counting the number of consecutive primes, which don't have to begin with 2
//
//I use the Sieve of Atkin algorithm to solve this:
//
// Sieve of Atkin
// see https://en.wikipedia.org/wiki/Sieve_of_Atkin
// This calculates all the prime numbers below the given limit
//
// Cargo.toml
//[[bin]]
//name = "euler50"
//path = "src/euler50.rs"
//

fn sieve_of_atkin(limit: u32 ) -> Vec<bool> {
	let rt: f64 = (limit as f64).sqrt();
	let sqrt_max: u32 = (rt as u32) + 1; 
	let mut barray:Vec<bool> = vec![false;limit as usize]; 
		
    for x in 1 .. sqrt_max
    {
        for y in 1 .. sqrt_max
        {
 			//group 1
 			let mut k:usize = (4 * x * x + (y * y)) as usize;
			let mut m:usize = k % 12;

    		if k < (limit as usize) && (m == 1 || m == 5) {
    			barray[k]=!barray[k];
    		}

    		//group 2
      		k = (3 * x * x + (y * y)) as usize;
      		m = k % 12;
    		if k < (limit as usize) && m == 7 {
    			barray[k]=!barray[k];
    		}

    		//group 3
			if x > y {
				k = (3 * x * x - (y * y)) as usize;
				m = k % 12;
			    if k < (limit as usize) && m == 11 {
			    	barray[k]=!barray[k];
				}
			}
		}
	}

	//final clean up
	barray[2]=true;
	barray[3]=true;
	for n in 5 .. (sqrt_max+1) {
		let b :bool= barray[n as usize];
		if b {
			let n2 :u32= n * n;
			let mut z:u32 = n2;
			while z < limit {
				barray[z as usize]=false;
				z = z + n2;
			}
		}
	}
	return barray;
}

fn main() {
	//let limit:u32 = 100;
	//let limit:u32 = 1000;
	let limit:u32 = 1_000_000;
	let ba:Vec<bool> =  sieve_of_atkin(limit);
	let mut best:u64 = 0;	
	let mut tentative:u64 = 0;
	let mut tent_long:u32 = 0;
	let mut best_long:u32 = 0;
	let mut starting_with:u32 = 0;
	
	//x is the starting point
	for x in 2 .. limit {
		//bail if not prime
		if !ba[x as usize] {
			continue;
		}
		tentative = 0;
		tent_long = 0;		
		//println!("starting at {}",x);
		for i in x .. limit {	
			if ba[i as usize] {
				tentative = tentative + i as u64;
				tent_long = tent_long + 1;
			}
			if tentative >= limit as u64 {
				break;
			} else {
				//check to see if it is prime
				if ba[tentative as usize] {
					if tent_long > best_long {
						best_long = tent_long;
						best = tentative;
						starting_with = x;
						println!("longest is now {}",best_long);
					}
				} 
			}
		}
	}
	println!("The longest sum of consecutive primes below {} contains {} terms (starting with {}) and totals {}",limit,best_long,starting_with,best);
}