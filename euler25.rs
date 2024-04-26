//see https://projecteuler.net/problem=25

const ZERO: u8 = 48;

// argument validation
fn digitize(b: u8) -> u8 {
    if b < 48 || b > 57 {
    	println!("{}", b);
        panic!("digit 0-9 expected");
    }
    return b - ZERO;
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

//return the fibonacci term that contains the number of digits
fn fib_digits(n:u32) -> u32 {
	//vf0 is current t
	let mut vf0 = vectorize_slice("0");
	let mut vf1 = vectorize_slice("1");
	let mut vf2 = vectorize_slice("1");
	
	let mut i:u32 = 2;
	
	loop {
		vf2 = add(vf0.clone(), vf1.clone());
		let svf2 = stringify(vf2.clone());
		println!("term {} has fibonnaci value {}",i,svf2);
		if vf2.len() as u32 == n {
			return i;
		} else {
			vf0 = vf1;
			vf1 = vf2;
		}
		i = i + 1;
	}
}

fn main() {
	let d:u32 = 1000;
	let t = fib_digits(d);
	println!("The fibonacci sequence with {} digits has {} terms",d,t);
}
