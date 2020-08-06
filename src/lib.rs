use itertools::Itertools;
use primal;
// use serde_json;
use std::convert::TryInto;
use std::time::Instant;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
    println!("The Rust function say() received {}", s);
    let r = String::from("hello ");
    return r + s;
}

#[wasm_bindgen]
pub fn factorize(n: i32) -> String {
    let now = Instant::now();
    if primal::is_prime(n.try_into().unwrap()) {
        let elapse = now.elapsed().as_micros();
        return format!("{}, elapsed {} µs in WASM.", n, elapse);
    }

    let upper: usize = f64::from(n).sqrt().floor() as usize;
    let sieve = primal::Sieve::new(upper);
    let factors = match sieve.factor(n.try_into().unwrap()) {
        // Ok(factors) => serde_json::to_string(&factors).unwrap(),
        // Err(_) => String::from("[]")
        Ok(factors) => factors
            .iter()
            .map(|(prime, power)| match power {
                1 => prime.to_string(),
                _ => format!("{}^{}", prime, power),
            })
            .join(" * "),
        Err(_) => String::from("Error occurs"),
    };
    let elapse = now.elapsed().as_micros();
    format!("{}, elapsed {} µs in WASM", factors, elapse)
}
