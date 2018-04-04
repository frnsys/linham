extern crate hex;
extern crate rayon;
extern crate hamming;

use rayon::prelude::*;
use std::io::{self, BufRead};

fn to_bytearray(hex: String) ->  [u8; 32] {
    let hash = hex::decode(hex).unwrap();
    let mut arr = [0u8; 32];
    arr.clone_from_slice(&hash);
    return arr;
}

fn linsearch(hex: String, threshold: u64, db: Vec<[u8; 32]>) -> Vec<(usize, [u8; 32])> {
    let arr = to_bytearray(hex);
    return db.into_par_iter().enumerate().filter(|&(_, imh)| {
        let dist = hamming::distance_fast(&imh, &arr).unwrap();
        return dist < threshold;
    }).collect();
}

fn main() {
    let size = 100000000;
    let mut db = Vec::with_capacity(size);;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let hex = line.unwrap();
        let arr = to_bytearray(hex);
        db.push(arr);
    }
    println!("loaded {:?} hashes", db.len());

    let query = "de62deb4cf24eb05fc863cc66e866c166954e959c448c606db87cc008e57ba97".to_string();
    let results = linsearch(query, 80, db);
    println!("{:?} results", results.len());
}

