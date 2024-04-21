// Project Euler Problem 10
// from https://projecteuler.net/problem=10
//
// Problem statement:
// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17
// Find the sum of all the primes below two million.
//
//I use the Sieve of Atkin algorithm to solve this:
//
// Sieve of Atkin
// see https://en.wikipedia.org/wiki/Sieve_of_Atkin
// This calculates all the prime numbers below the given limit
//
// Cargo.toml
//[[bin]]
//name = "euler10"
//path = "src/euler10.rs"
//

const QUERY:u32 = 2_000_000;

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
	//this is just a wild guess as to the limit
	let limit:u32 = QUERY;
	let ba:Vec<bool> =  sieve_of_atkin(limit);
	let mut adder:u64 = 0;
	
	for i in 2 .. limit {
		if ba[i as usize] {
			adder = adder + i as u64;
		}
	}
	println!("The sum of all primes below {} is: {}",QUERY,adder);

}