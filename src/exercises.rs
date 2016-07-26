extern crate crypto;
use std::str;
use std::path;
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
pub fn ex1()
{
    println!("Exercise 1.1");
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let output = crypto::hex_to_base64(input);
    println!("{}", &output);
}

pub fn ex2()
{
    
    println!("\n Exercise 1.2");
    let text = "1c0111001f010100061a024b53535009181c";
    let key = "686974207468652062756c6c277320657965";
    let text_bytes = crypto::hex_to_bytes(text);
    let key_bytes = crypto::hex_to_bytes(key);
    let out = crypto::xor_hex(&text_bytes, &key_bytes);
    println!("{}", crypto::bytes_to_hex(&out));
}


pub fn ex3()
{
    println!("\n Exercise 1.3");
    let text = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    println!("The decoded string is: {:?}", crypto::single_char_xor(text));
}


pub fn ex4()
{
    let filename = path::Path::new("src/data/4.txt");
    let file = File::open(filename);
    let reader = BufReader::new(file.unwrap());
    let lines = reader.lines();
    let mut max_score = 0;
    let mut max_string:String = String::new();
    for l in lines {
        let (temp_score, temp_string) = crypto::single_char_xor(str::trim(&l.unwrap()));
        if max_score < temp_score
        {
            max_score = temp_score;
            max_string = temp_string;
        }
    }
    println!("The decrypted string is {}", &max_string);
    
}

pub fn ex5()
{
    let text = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let out = crypto::xor_hex(&text.to_string().into_bytes(), &"ICE".to_string().into_bytes());
    println!("{}", crypto::bytes_to_hex(&out));
}

pub fn ex6()
{
    let text_a = "this is a test";
    let text_b = "wokka wokka!!!";
    println!("{}", crypto::hamming_distance_str(text_a, text_b));
}

