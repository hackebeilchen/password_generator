extern crate crypto;
extern crate rand;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use rand::{Rng, SeedableRng, XorShiftRng};
use std::iter::repeat;

pub fn generate_password(name: &str, masterpw: &str, symbols: Vec<Vec<char>>, length: usize)->Result<String,&'static str> {
    if length < symbols.len() {
        return Err("Password to short.")
    }
    let mut input = name.to_string();
    input.push_str(masterpw);
    //hash
    let mut sha = Sha256::new();
    sha.input_str(&input);
    let mut bytes: Vec<u8> = repeat(0u8).take(sha.output_bytes()).collect();
    sha.result(&mut bytes[..]);
    //build seed
    let mut seed:[u32;4] = [0;4];
    for (i,b) in bytes.iter().enumerate() {
        let pos = (i/4)%4;
        seed[pos] = seed[pos] ^ (*b as u32)<<(i*8)%32;
    }
    //select symbols
    let mut r: XorShiftRng = SeedableRng::from_seed(seed);
    let mut pw: Vec<char> = Vec::new();
    //one from each
    let first = symbols.len();
    for i in 0..first {
        pw.push(symbols[i][r.gen_range(0,symbols[i].len())]);
    }
    //fill up
    let mut all = Vec::new();
    for v in symbols {
        all.extend(v);
    }
    for _ in first..length {
        pw.push(all[r.gen_range(0,all.len())]);
    }
    //now to shuffeling:
    for i in 1..pw.len() {
        let pos = r.gen_range(0,i+1);
        let temp = pw[i];
        pw[i] = pw[pos];
        pw[pos] = temp;
    }
    Ok(pw.into_iter().collect())
}
