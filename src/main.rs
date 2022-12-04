extern crate bitcoin;

use bitcoin::hashes::hex::FromHex;
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::rand;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::SecretKey;
use bitcoin::util::address::Address;
use bitcoin::util::key::PrivateKey;
use bitcoin::util::taproot::TapBranchHash;

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        println!("Invalid number of args");
        return;
    }

    let prefix = args[1].to_lowercase();
    if prefix.len() <= 4 {
        println!("Prefix is too short");
        return;
    }
    if prefix.get(0..4) != Some("bc1p") {
        println!("Invalid prefix, must begin with bc1p");
        return;
    }

    const CHARSET: &str = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";
    let prefix_split: Vec<&str> = prefix.split("1").collect();
    for pc in prefix_split[1].chars() {
        if !CHARSET.contains(pc) {
            println!("Invalid character in prefix");
            return;
        }
    }

    let mut merkle_root: Option<TapBranchHash> = None;
    if args.len() == 3 {
        merkle_root = Some(TapBranchHash::from_hex(&args[2]).unwrap());
    }

    let secp = Secp256k1::new();

    loop {
        let internal_seckey = SecretKey::new(&mut rand::thread_rng());
        let internal_privkey = PrivateKey::new(internal_seckey, Network::Bitcoin);
        let (internal_pubkey, _) = internal_seckey.x_only_public_key(&secp);

        let addr = Address::p2tr(&secp, internal_pubkey, merkle_root, Network::Bitcoin);

        if addr.to_string().get(0..prefix.len()) == Some(&prefix) {
            println!("internal_privkey: {}", internal_privkey.to_wif());
            println!("internal_pubkey: {}", internal_pubkey);
            println!("Address: {}", addr);
            break;
        }
    }
}
