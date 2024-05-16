use std::fs::{self, File};
use std::io::Write;
use std::str::from_utf8;

use rand::*;

use crate::encryption::{decrypt, encrypt};

mod encryption;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() > 2 {
        let method = &args[1];
        let file = &args[2];

        println!("method: {}\nfile: {}", method, file);
    }

    let secret = fs::read_to_string("secret.txt").expect("Failed to read file");

    let mut key: [u8; 32] = [0; 32];
    let mut iv: [u8; 16] = [0; 16];

    let mut rng = thread_rng();
    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);

    let encrypted_data = encrypt(secret.as_bytes(), &key, &iv).unwrap();
    let decrypted_data = decrypt(&encrypted_data, &key, &iv).unwrap();

    let mut file = File::create("secret").expect("Failed to create file");
    file.write_all(&encrypted_data)
        .expect("Failed to write to file");

    let mut file = File::create("readme.txt").expect("Failed to create file");
    file.write_all("Claque ton prime sinon plus de secret big noobs".as_bytes())
        .expect("Failed to write readme.txt");

    // let mut file = File::create("key.txt").expect("Failed to create file");
    // file.write_all(&key).expect("Failed to write key.txt");

    // let mut file = File::create("iv.txt").expect("Failed to create file");
    // file.write_all(&iv).expect("Failed to write iv.txt");

    println!("secret: {}\n", secret);
    println!("key: {:?}", key);
    println!("iv: {:?}", iv);
    println!("encrypted data: {:?}", encrypted_data);
    println!(
        "decrypted data: {:?}",
        from_utf8(&decrypted_data).expect("Failed to convert data to string")
    );
}
