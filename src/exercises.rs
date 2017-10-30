extern crate crypto;
use std::str;
use std::path;
use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::io::Read;
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
    let out = crypto::xor_bytes(&text_bytes, &key_bytes);
    println!("{}", crypto::bytes_to_hex(&out));
}


pub fn ex3()
{
    println!("\n Exercise 1.3");
    let text = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    let text_bytes = crypto::hex_to_bytes(text);
    let guesses = crypto::single_char_xor_score(&text_bytes, 6);
    for (_, guess_char) in guesses {
        let decoded = str::from_utf8(&crypto::xor_bytes(&text_bytes, &vec![guess_char as u8;1])).unwrap_or("").to_string();
    println!("The decoded string is: {:?}", decoded);
    }
}


pub fn ex4()
{
    let filename = path::Path::new("src/data/4.txt");
    let file = File::open(filename);
    let reader = BufReader::new(file.unwrap());
    let lines = reader.lines();
    let mut max_score = 0;
    let mut max_string = String::new();
    for l in lines {
        let mut temp_line = l.unwrap();
        let line = str::trim(&temp_line);
        let line_bytes = crypto::hex_to_bytes(line);
        let (temp_score, temp_char) = crypto::single_char_xor_score(&line_bytes, 1)[0].clone();
        if max_score < temp_score
        {
            max_score = temp_score;
            max_string = str::from_utf8(&crypto::xor_bytes(&line_bytes, &vec![temp_char as u8;1])).unwrap_or("").to_string();
        }
    }
    println!("The decrypted string is {}", max_string);
    
}

pub fn ex5()
{
    let text = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
    let out = crypto::xor_bytes(&text.to_string().into_bytes(), &"ICE".to_string().into_bytes());
    println!("{}", crypto::bytes_to_hex(&out));
}

pub fn ex6()
{
    let filename = path::Path::new("src/data/6.txt");
    let file = File::open(filename);
    let mut base64 = String::new();
    file.unwrap().read_to_string(& mut base64).expect("file to be opened");
    let hex = crypto::base64_to_hex(&base64);
    let encrypted = crypto::hex_to_bytes(&hex);
    let guess_keysize = crypto::guess_keysize(&encrypted, 1)[0];
    println!("guess key_size: {:?}", crypto::guess_keysize(&encrypted, 2));
    let key = crypto::key_vignere(&encrypted, guess_keysize);
    println!("guessed key: {}", key);
    let decrypted = str::from_utf8(&crypto::xor_bytes(&encrypted, &key.into_bytes())).unwrap_or("").to_string();
    println!("decrypted text: {}", decrypted);

}

