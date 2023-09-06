use std::env;
use std::process::exit;
use std::fs::File;
use std::io::{Read, Write};
use flate2::write::DeflateDecoder;
​
fn decrypt_file(file_path: &String)
{
    println!("decrypting {}", file_path);
​
    // Read the encrypted file
    let mut file = File::open(file_path).expect("Error opening input file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Error reading input file");
​
    // The last byte of the buffer is the xor key
    let last_index = data.len() - 1;
    let xor_key = data[last_index];
​
    // Xor the buffer until the last byte
    let mut xored_data = Vec::new();
    for index in 0..last_index
    {
        xored_data.push(data[index] ^ xor_key);
    }
​
    // Decoding the XORed data
    let mut decoder = DeflateDecoder::new(Vec::new());
    decoder.write_all(xored_data.as_slice()).expect("Error writing data to decoder");
    let decompressed_data=decoder.finish().expect("Error decoding data");
​
    // Write the decrypted data
    let mut decrypted_file_name = file_path.to_owned();
    decrypted_file_name.push_str(".decrypted");
    let mut decrypted_file = File::create(decrypted_file_name).expect("Error opening output file");
    decrypted_file.write_all(decompressed_data.as_slice()).expect("Error writing to output file");
}
​
fn main()
{
    let args: Vec<_> = env::args().collect();
    if args.len() != 2
    {
        println!("usage: {} FILE", args[0]);
        exit(1);
    }
    let file_path = &args[1];
    decrypt_file(file_path);
}
