use bs58;
// use std::io::{self, BufRead};

fn main() {
    println!("Enter a base58 string:");
    // let stdin = io::stdin();
    let base58 = "YOUR_PK_HERE"; //stdin.lock().lines().next().unwrap().unwrap();

    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("Converted to Vec<u8>: {:?}", wallet);

    let base58 = bs58::encode(wallet).into_string();
    println!("Converted back to base58: {:?}", base58);
}
