//Modified from https://rosettacode.org/wiki/Long_multiplication#Go

// Long multiplication per WP article referenced by task description.
// That is, multiplicand is multiplied by single digits of multiplier
// to form intermediate results.  Intermediate results are accumulated
// for the product.  Used here is the abacus method mentioned by the
// article, of summing intermediate results as they are produced,
// rather than all at once at the end.
//
// Limitations:  Negative numbers not supported, superfluous leading zeros
// not generally removed.

//add this to Cargo.toml
// [[bin]]
// name = "long_multiplication"
// path = "src/long_multiplication.rs"

//Ascii 48 is 0
const ZERO: u8 = 48;
const N:&str = "18446744073709551616";

// argument validation
fn digitize(b: u8) -> u8 {
    if b < 48 || b > 57 {
    	println!("{}", b);
        panic!("digit 0-9 expected");
    }
    return b - ZERO;
}

//convert String to vector of bytes
//remove the trailing space
fn vectorize(s: String) -> Vec<u8> {
	return s.trim().as_bytes().to_vec();
}

fn vectorize_slice(s: &str) -> Vec<u8> {
	return s.trim().as_bytes().to_vec();	
}

fn stringify(v: Vec<u8>) -> String {
	return String::from_utf8(v).unwrap();
}

//we use Vec of u8 instead of strings
fn add(x: Vec<u8>, y: Vec<u8>) -> Vec<u8> {
	let mut x1 = x;
	let mut y1 = y;

	//if y is longer than x then swap them
	//so x is always longer
	if y1.len() > x1.len() {
		let tmp = y1;
		y1 = x1;
		x1 = tmp;
	}

	//create a new Vec to hold the result
	let vb_len = x1.len() + 1;
	let mut vb:Vec<u8> = vec![0 as u8;vb_len];
	let mut c:u8 = 0;
	//note that the range is exclusive
  	for i in 1 .. vb_len {
    	if i <= y1.len() {
        	c += digitize(y1[y1.len()-i]);
       	}  	
    	let s:u8 = digitize(x1[x1.len()-i]) + c;
        c = s / 10;
        let c2:u8 = (s % 10) + ZERO;
        //println!("{} = {}", vb_len-1, c2);
        vb[vb_len-i] = c2;
  	}
  	
  	if c == 0 {
  		vb.remove(0);
  		return vb;
  	}  //else
    vb[0] = c + ZERO;	
	return vb;
}

fn mul_digit(x: Vec<u8>, cy: u8) -> Vec<u8> {
	if cy == ZERO {
		return vec![ZERO];
	}
	let y:u8 = digitize(cy);
	let vb_len = x.len() + 1;
	let mut vb:Vec<u8> = vec![0 as u8;vb_len];	
	let mut c:u8 = 0;
  	for i in 1 .. vb_len {
  		let s:u8 = digitize(x[x.len()-i])*y + c;
  		c = s / 10;
  		vb[vb_len-i] = (s % 10) + ZERO;
  	}
  	if c == 0 {
  		vb.remove(0);
  		return vb;
  	}  //else
    vb[0] = c + ZERO;	
	return vb;  	
}

fn mul(x: Vec<u8>, y: Vec<u8>) -> Vec<u8> {
	//we have to clone the vec because it is consumed by mul_digit
	let mut x1:Vec<u8> = x.clone();

	//multiple the x by the right-most digit of y
	let mut result:Vec<u8> = mul_digit(x1, y[y.len()-1]);

	let mut factor:usize = 0;
	for i in 2.. y.len()+1 {
		//increase the factor
		factor = factor+1;
		x1 = x.clone();
		let mut intermediate:Vec<u8> = mul_digit(x1, y[y.len()-i]);

		for _j in 0..factor {
			//by adding zeros to the end
			//we are in effect multiplying it by 10
			intermediate.push(ZERO);

		}
		result = add(result, intermediate);
	}
	return result;
}

fn main() {
	let m = vectorize_slice(N);
	let n = vectorize_slice(N);
	//I do this twice because mul absorbs the inputs
	let p = mul(m, n);
    println!("{}",stringify(p));
}
