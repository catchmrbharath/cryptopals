extern crate rustc_serialize as serialize;


use serialize::hex::{FromHex, ToHex};
use serialize::base64::{ToBase64, FromBase64, STANDARD};
use std::str;
pub fn hex_to_base64(hex_string: &str) -> String
{
    let hex = hex_string.to_string().from_hex().ok().expect("Error in hex string");
    hex.as_slice().to_base64(STANDARD)

}

pub fn base64_to_hex(base64_string: &str) -> String
{
    let base64 = base64_string.to_string().from_base64().unwrap();
    base64.as_slice().to_hex()
}

pub fn xor_hex(text: &[u8], key: &[u8]) -> Vec<u8>
{
    let key_iter = key.iter().cycle();
    text.iter().zip(key_iter).map(|(&a, b)| a ^ b).collect::<Vec<u8>>()
}

pub fn hex_to_bytes(text: &str) -> Vec<u8>
{
    text.from_hex().unwrap()
}


pub fn bytes_to_hex(text: &[u8]) -> String
{
    text.to_hex()
}


pub fn single_char_xor(text: &str) -> (u64, String)
{
    let mut max_score = 0;
    let mut max_char = 0u8;
    let text_bytes = hex_to_bytes(text);
    for v in 0u8..255
    {
        let out = xor_hex(&text_bytes, &vec![v; 1]);
        let score = score_text(&str::from_utf8(&out).unwrap_or(""));
        if score > max_score
        {
            max_score = score;
            max_char = v;
        }
    }
    let out = xor_hex(&text_bytes, &vec![max_char; 1]);
    (max_score, str::from_utf8(&out).unwrap_or("").to_string())
    
}

fn score_text(text: &str) -> u64
{
    text.chars().map(|a|
                     match a.to_uppercase().next().unwrap() {
                         'E' => 2,
                         'T' => 2,
                         'A' => 2,
                         'I' => 2,
                         'O' => 2,
                         'N' => 2,
                         'S' => 1,
                         'H' => 1,
                         'R' => 1,
                         'D' => 1,
                         'L' => 1,
                         'U' => 1,
                         _ => 0
                     }).fold(0, |sum, elem| sum + elem)
}

fn hamming_dist_u8(a: u8, b: u8) -> u8
{
    let mut c = a ^ b;
    let mut count = 0u8;
    while c != 0
    {
        count = count + 1;
        c = c & c - 1;

    }
    count
}

fn combinations<'a>(arr: &[&'a[u8]]) -> Vec<(&'a[u8], &'a[u8])>
{
    let len = arr.len();
    let comb_len: usize = len * (len - 1) / 2;
    let mut comb = Vec::< (&[u8], &[u8]) >::with_capacity(comb_len);
    for (i, item_a) in arr.iter().enumerate()
    {
        for item_b in arr[i + 1..].iter()
        {
            comb.push((*item_a, *item_b));
        }
    }
    comb
}


fn hamming_distance(bytes_a: &[u8], bytes_b: &[u8]) -> u64
{
    bytes_a.iter().zip(bytes_b.iter()).map(|(&a, &b)| hamming_dist_u8(a, b)).fold(0u64, |sum, t| sum + t as u64)
}

pub fn hamming_distance_str(text_a: &str, text_b : &str ) -> u64
{
    let bytes_a = text_a.to_string().into_bytes();
    let bytes_b = text_b.to_string().into_bytes();
    hamming_distance(&bytes_a, &bytes_b)
}

pub fn guess_keysize(bytes: &[u8]) -> u32
{
    let mut min_score = 10000f64;
    let mut min_key_size = 0u32;
    for key_size in 2..41
    {
        let chunks = create_chunks(bytes, key_size);
        let mut count = 0;
        let mut total_score = 0f64;
        while count + 4 < chunks.len()
        {
            let combs = combinations(&chunks[count..count + 4]);
            let score = score_hamming(&combs);
            total_score  += score;
            count += 4;
        }
        total_score = total_score / (key_size * chunks.len()) as f64;
        if total_score < min_score
        {
            min_score = total_score;
            min_key_size = key_size as u32;
        }
    }
    min_key_size
}

pub fn score_hamming<'a>(chunks: &[(&'a[u8], &'a[u8])]) -> f64
{
    let len = chunks.len();
    let total = chunks.iter().fold(0, |sum, &(a, b)| sum + hamming_distance(a, b));
    let ans = total as f64/ len as f64;
    ans
}

fn create_chunks(bytes: &[u8], key_size: usize) -> Vec<&[u8]>
{
    let mut count: usize = 0;
    let mut chunks = Vec::<&[u8]>::new();
    while count + key_size < bytes.len()
    {
        chunks.push(&bytes[count..count + key_size]);
        count += key_size;
            
    }
    chunks
}
