extern crate rustc_serialize as serialize;


use serialize::hex::{FromHex, ToHex};
use serialize::base64::{ToBase64, FromBase64, STANDARD};
use std::str;
use std::cmp::Ordering::Less;
pub fn hex_to_base64(hex_string: &str) -> String {
    let hex = hex_string.to_string().from_hex().ok().expect(
        "Error in hex string",
    );
    hex.as_slice().to_base64(STANDARD)

}

pub fn base64_to_hex(base64_string: &str) -> String {
    let base64 = base64_string.to_string().from_base64().unwrap();
    base64.as_slice().to_hex()
}

pub fn xor_bytes(text: &[u8], key: &[u8]) -> Vec<u8> {
    let key_iter = key.iter().cycle();
    text.iter()
        .zip(key_iter)
        .map(|(&a, b)| a ^ b)
        .collect::<Vec<u8>>()
}

pub fn hex_to_bytes(text: &str) -> Vec<u8> {
    text.from_hex().unwrap()
}


pub fn bytes_to_hex(text: &[u8]) -> String {
    text.to_hex()
}


pub fn single_char_xor_score(text_bytes: &[u8], topn: usize) -> Vec<(u64, char)> {
    let mut result: Vec<(u64, char)> = Vec::new();
    for v in 0u8..255 {
        let out = xor_bytes(&text_bytes, &vec![v; 1]);
        let score = score_text(&str::from_utf8(&out).unwrap_or(""));
        result.push((score, v as char));
    }
    result.sort_by_key(|&(a, _)| a);
    result.reverse();
    result.truncate(topn);
    result
}


fn score_text(text: &str) -> u64 {
    text.chars()
        .map(|a| match a.to_uppercase().next().unwrap() {
            'E' => 21912,
            'T' => 16587,
            'A' => 14810,
            'O' => 14003,
            'I' => 13318,
            'N' => 12666,
            'S' => 11450,
            'R' => 10977,
            'H' => 10795,
            'D' => 7874,
            'L' => 7253,
            'U' => 5246,
            'C' => 4943,
            'M' => 4761,
            'F' => 4200,
            'Y' => 3853,
            'W' => 3819,
            'G' => 3693,
            'P' => 3316,
            'B' => 2715,
            'V' => 2019,
            'K' => 1257,
            'X' => 315,
            'Q' => 205,
            'J' => 188,
            'Z' => 128,
            _ => 0,
        })
        .fold(0, |sum, elem| sum + elem)
}

fn hamming_dist_u8(a: u8, b: u8) -> u8 {
    let mut c = a ^ b;
    let mut count = 0u8;
    while c != 0 {
        count = count + 1;
        c = c & c - 1;

    }
    count
}

pub fn hamming_distance(bytes_a: &[u8], bytes_b: &[u8]) -> u64 {
    bytes_a
        .iter()
        .zip(bytes_b.iter())
        .map(|(&a, &b)| hamming_dist_u8(a, b))
        .fold(0u64, |sum, t| sum + t as u64)
}

fn score_keysize(encrypted_text : &[u8], key_size: usize) -> f64 {
    let chunks: Vec<&[u8]> = encrypted_text.chunks(key_size).collect();
    let mut score = 0.0;
    for i in 0..4 {
        for j in 1..4 {
            let v1 = chunks[i];
            let v2 = chunks[j];
            score += hamming_distance(v1, v2) as f64;
        }
    }
    score / key_size as f64
}

pub fn guess_keysize(encrypted_text : &[u8], topn: usize) -> Vec<usize>{
    let mut key_sizes: Vec<(f64, usize)> = (2usize..40)
        .map( |x| (score_keysize(encrypted_text, x), x))
        .collect();
    key_sizes.sort_by(|&a, &b| a.0.partial_cmp(&b.0).unwrap_or(Less));

    key_sizes.truncate(topn);
    key_sizes.iter().map(|&a| a.1).collect()

}


pub fn key_vignere(encrypted_text: &[u8], key_size: usize) ->String {
    let mut key = String::new();
    for i in 0..key_size {
        let mut vec : Vec<u8> = Vec::new();
        let mut index = i;
        while index < encrypted_text.len() {
            vec.push(encrypted_text[index]);
            index += key_size;
        }
        let (_score, key_char) = single_char_xor_score(&vec, 1)[0];
        key.push(key_char);
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hamming_distance() {
        let text_a = "this is a test";
        let text_b = "wokka wokka!!!";

        let temp_a = text_a.to_string().into_bytes();
        let temp_b = text_b.to_string().into_bytes();
        assert_eq!(37, hamming_distance(&temp_a, &temp_b));
    }
}

