// Sieve of Atkin
// see https://en.wikipedia.org/wiki/Sieve_of_Atkin
// This calculates all the prime numbers below the given limit
//
// Cargo.toml
//[[bin]]
//name = "atkin"
//path = "src/atkin.rs"
//
use std::env;

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

//returns 0 if invalid
fn command_line_arg() -> u32 {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
    	println!("Syntax: atkin <num>");
    	return 0;
    }
    
    let arg1:&str = &args[1];	
    //convert it to a number
	match arg1.trim().parse::<u32>() {
        Ok(n) => return n,
        Err(_e) => return 0,
    }	    
}

fn main() {
	let limit:u32 = command_line_arg();
	if limit < 2 { return;}
	let ba:Vec<bool> =  sieve_of_atkin(limit);    
	for i in 2 .. limit {
		if ba[i as usize] {
			print!("{} ", i);
		}
	}
	println!(" ");
}