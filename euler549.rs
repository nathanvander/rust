//https://github.com/HaochenLiu/My-Project-Euler/blob/master/549.py

use std::cmp;

fn get(n:u64) -> u64 {
 	//println!("get: {} ",n); 
 	//the default value is 1
	let mut m_values = vec![1; (n+1) as usize];
    m_values[0] = 0;
    m_values[1] = 0;
        for i in 2 ..  n + 1 {
        	//print!("{} ",i);
            if m_values[i as usize] == 1 {
                let mut d:u64 = i;
                let mut e:u64 = 1;
                while d <= n {
                    let m:u64  = get_smallest_number(i, e);
                    for j in (d .. n+1).step_by(d as usize) {
                        m_values[j as usize] = cmp::max(m_values[j as usize], m)
                    }
                    d = d * i;
                    e += 1;
                 }
             }
        }
    return sum(m_values);
}

fn get_smallest_number(prime:u64, exponent:u64) -> u64 {
        let mut lower_bound:u64 = 1;
        let mut upper_bound:u64 = exponent; 
        while lower_bound <= upper_bound {
            let middle:u64 = (lower_bound + upper_bound) / 2;
            let middle_value:u64 = adic_valuation(middle * prime, prime);
            if middle_value < exponent {
                lower_bound = middle + 1;
            } else if middle_value == exponent {
                return middle * prime;
            } else {
                upper_bound = middle - 1;
            }
        }
        
        let out = lower_bound * prime;
        println!("small: {}, {} -> {} ", prime, exponent, out);
        return out;
}

fn sum(v:Vec<u64>) -> u64 {
	let mut s:u64 = 0;
	for i in 0 .. v.len() {
		//print!("{} ",v[i as usize]);
		s = s + v[i as usize]; 
	}
	return s;
}


fn adic_valuation(fact_n:u64, prime:u64) -> u64 {
        let mut output:u64 = 0;
        let mut d:u64 = prime;

        while d <= fact_n { 
            output = output + fact_n / d;
            d = d * prime;
        }
        println!("adic: {}, {} -> {} ", fact_n, prime, output);
        return output;
}

    
fn main() {
	//println!("{}",get(100));
	println!("{}",get(100_000_000));
}

