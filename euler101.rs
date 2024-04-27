//euler101
//See https://projecteuler.net/problem=101
/**
 * Solution to Project Euler problem 101
 * Copyright (c) Project Nayuki. All rights reserved.
 *
 * https://www.nayuki.io/page/project-euler-solutions
 * https://github.com/nayuki/Project-Euler-solutions
 *
 * The generating function u(n) is a polynomial of degree 10.
 * OP(k, n) is a polynomial of degree at most k-1, which can be obtained
 * by the Lagrange interpolating polynomial (or other methods).
 * Any polynomial P(n) of degree k has at most k roots (i.e. points where P(n) = 0).
 * The zero polynomial Z(n) = 0 has negative infinite degree, and has roots everywhere.
 * Now, let E(n) = u(n) - OP(k, n), which is also a polynomial.
 *
 * If k <= 10, then OP(k, n) has degree less than k <= 10, so E(n) has degree 10. So E(n) has at most 10 roots.
 * By construction, OP(k, n) = u(n) for n = 1, 2, ..., k, thus E(n) already has k roots at {1, 2, ..., k}.
 * E(n) has at most 10 - k roots remaining, hence among the 11 - k values {k+1, k+2, ..., 11},
 * there must be an n where E(n) != 0 (i.e. an incorrect term where OP(k, n) != u(n)).
 *
 * If k > 10, E(n) has k roots at {1, 2, ..., k}, and possibly others.
 * So either E(n) has degree at least k, or it's the zero polynomial.
 * Now, u(n) has degree 10 and OP(k, n) has degree at most k-1,
 * so their difference E(n) has at most degree max(10, k-1) = k-1.
 * This means E(n) does not have degree k, so it is the zero polynomial.
 * Hence u(n) = OP(k, n), and there are no incorrect terms.
 *
 * In conclusion, BOPs exist for and only for 1 <= k <= 10. For each k in that range,
 * the first incorrect term (FIT) of OP(k, n) exists for some n in {k+1, k+2, ..., 11}.
*/

fn gcd(mut a:i64, mut b:i64) -> i64 {
	if a<0 {
		a = -a;
	}
	if b<0 {
		b = -b;
	}
	while b!=0 {
		let t = b;
	    b = a % b;
	    a = t;
	}
	return a;
}

struct Fraction {
	numer: i64,
	denom: i64,
}

impl Fraction {
	fn new(mut n:i64, mut d:i64) -> Self {
    	if (d < 0) {
        	n = -n;
        	d = -d;
    	}
    	//sanity check
		if d<1 {
			panic!("invalid value of d {}",d);
		}

		//reduce
		let g:i64 = gcd(n,d);
	    if g > 1 {
	    	n = n/g;
	    	d = d/g;
		}

		Self{
			numer: n,
			denom: d,
		}
	}
	
	fn to_string(&self) -> String {
		if self.denom==1 {
			return self.numer.to_string();
		} else {
			return format!("{}/{}",self.numer,self.denom);
		}
	}
	
	fn equals(&self,f:Fraction) -> bool {
		return self.numer == f.numer && self.denom == f.denom;
	}
	
	fn clone(&self) -> Fraction {
		return Fraction::new(self.numer,self.denom);
	}

	fn add(&self,f:Fraction) -> Fraction {
		return Fraction::new(self.numer * f.denom + f.numer * self.denom, self.denom * f.denom);
	}

	fn mul(&self,f:Fraction) -> Fraction {
    	return Fraction::new(self.numer * f.numer, self.denom * f.denom);
	}

	//I don't implement sub or div because I don't need them
}

//=============================
const DEGREE:i64 = 10;

fn run() -> String {
	let mut sum = Fraction::new(0,1);
	
	//1..10
	for k in 1 .. DEGREE+1 {
		let mut n = k+1;
		loop {
			let reference:Fraction = Fraction::new(generating_function(n as i64),1 as i64);
			let term = optimum_polynomial(k as i32,n as i64);
			if !term.equals(reference) {
				sum = sum.add(term);
				break;
			}
			n=n+1;
			if n==DEGREE+2 {
				panic!("Assertion Error: n = {}",n);
			}
		}
	}
	return sum.to_string();
}

fn optimum_polynomial(k:i32,n:i64) -> Fraction {
	// Lagrange interpolation
	let mut sum = Fraction::new(0,1);
	
	for i in 1 .. k+1 {
		let mut product:Fraction = Fraction::new(generating_function(i as i64),1);
		for j in 1 .. k+1 {
			if j!=i {
				let f2:Fraction = Fraction::new(n - j as i64,i as i64 - j as i64);
				product = product.mul(f2);
			}
		}
		sum = sum.add(product);
	}
	return sum;
}

fn generating_function(n:i64) -> i64 {
	let mut sum:i64 = 0;
	let bin:i64 = -n;
	for i in 0 .. DEGREE+1 {
		sum = sum + bin.pow(i as u32);
	}
	return sum;
}

fn main() {
	println!("{}",run());
}

