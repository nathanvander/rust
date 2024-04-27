/*
 * Solution to Project Euler problem 100
 * See https://projecteuler.net/problem=100
 * Copyright (c) Project Nayuki. All rights reserved.
 *
 * https://www.nayuki.io/page/project-euler-solutions
 * https://github.com/nayuki/Project-Euler-solutions
 */

//import java.math.BigInteger;
//public final class p100 implements EulerSolution {
//	public static void main(String[] args) {
//		System.out.println(new p100().run());
//	}

	/*
	 * Suppose the box has b blue discs and r red discs.
	 * The probability of taking 2 blue discs is [b / (b + r)] * [(b - 1) / (b + r - 1)],
	 * which we want to be equal to 1/2. Rearrange the equation:
	 *   [b(b - 1)] / [(b + r)(b + r - 1)] = 1 / 2.
	 *   2b(b - 1) = (b + r)(b + r - 1).
	 *   2b^2 - 2b = b^2 + br - b + br + r^2 - r.
	 *   b^2 - b = r^2 + 2br - r.
	 *   b^2 - (2r + 1)b + (r - r^2) = 0.
	 * Apply the quadratic equation to solve for b:
	 *   b = [(2r + 1) +/- sqrt((2r + 1)^2 - 4(r - r^2))] / 2
	 *     = r + [1 +/- sqrt(8r^2 + 1)]/2
	 *     = r + [sqrt(8r^2 + 1) + 1]/2.  (Discard the minus solution because it would make b < r)
	 *
	 * For b to be an integer, we need sqrt(8r^2 + 1) to be odd, and also 8r^2 + 1 be a perfect square.
	 * Assume 8y^2 + 1 = x^2 for some integer x > 0.
	 * We can see this is in fact a Pell's equation: x^2 - 8y^2 = 1.
	 *
	 * Suppose we have the solution (x0, y0) such that x0 > 0 and x0 is as small as possible.
	 * This is called the fundamental solution, and all other solutions be derived from it (proven elsewhere).
	 * Suppose (x0, y0) and (x1, y1) are solutions. Then we have:
	 *   x0^2 - 8*y0^2 = 1.
	 *   (x0 + y0*sqrt(8))(x0 - y0*sqrt(8)) = 1.
	 *   (x1 + y1*sqrt(8))(x1 - y1*sqrt(8)) = 1.  (Similarly)
	 * Multiply them together:
	 *   [(x0 + y0*sqrt(8))(x0 - y0*sqrt(8))][(x1 + y1*sqrt(8))(x1 - y1*sqrt(8))] = 1 * 1.
	 *   [(x0 + y0*sqrt(8))(x1 + y1*sqrt(8))][(x0 - y0*sqrt(8))(x1 - y1*sqrt(8))] = 1.
	 *   [x0*x1 + x0*y1*sqrt(8) + x1*y0*sqrt(8) + 8y0*y1][x0*x1 - x0*y1*sqrt(8) - x1*y0*sqrt(8) + 8y0*y1] = 1.
	 *   [(x0*x1 + 8y0*y1) + (x0*y1 + x1*y0)*sqrt(8)][(x0*x1 + 8y0*y1) - (x0*y1 + x1*y0)*sqrt(8)] = 1.
	 *   (x0*x1 + 8y0*y1)^2 - 8*(x0*y1 + x1*y0)^2 = 1.
	 * Therefore (x0*x1 + 8y0*y1, x0*y1 + x1*y0) is also a solution.
	 * By inspection, the fundamental solution is (3, 1).
	 */
//	public String run() {

//add this to the Cargo.toml
//----------------------
//[dependencies]
//num = "0.4.2"
//
//[[bin]]
//name = "euler100_2"
//path = "src/euler100_2.rs"
//----------------------

use num::integer::Roots; // 0.4.0

fn run() -> u128 {
	const B1:u128 = 1;
	const B2:u128 = 2;
	const B3:u128 = 3;
	const B8:u128 = 8;
	const B10:u128 = 10;

	const LIMIT:u32 = 12;
	
	let x0:u128 = B3.clone();
	let y0:u128 = B1.clone();

	let mut x:u128 = x0.clone();
	let mut y:u128 = y0.clone();
	println!("x = {}",x);
	println!("y = {}",y);
	
		//while (true) {
	loop {
		let a:u128 = y * y * B8 + B1;
		//this is truncated
		let q:u128 = a.sqrt();
		println!("sqrt = {}",q);
		
		if q % 2 == 1 { //is odd
			let blue:u128 = (q + B1) / B2 + y;
			if blue + y > B10.pow(LIMIT) {
				return blue;
			}
		}

		let nextx:u128 = x * x0 + y * y0 * B8;
		let nexty:u128 = x * y0 + y * x0;
		x = nextx;
		y = nexty;
	println!("x = {}",x);
	println!("y = {}",y);		
	}
}

fn main() {
	let b = run();
	println!("{}",b);
}