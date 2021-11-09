extern crate bitcoin;

use bitcoin::hashes::Hash;
use bitcoin::hashes::HashEngine;
use bitcoin::network::constants::Network;
use bitcoin::schnorr::PublicKey;
use bitcoin::secp256k1::rand::rngs::OsRng;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::address::Address;
use bitcoin::util::taproot::TapTweakHash;

fn main() {

    const PREFIX: &str = "bc1ponet";

    let secp = Secp256k1::new();

    let mut rng = OsRng::new().unwrap();

    loop {
        let (internal_seckey, internal_pubkey) = secp.generate_schnorrsig_keypair(&mut rng);

        let mut tweak: Vec<u8> = Vec::new();
        tweak.extend_from_slice(&internal_pubkey.serialize());
        let mut engine = TapTweakHash::engine();
        engine.input(&tweak);
        let tweak_value: [u8; 32] = TapTweakHash::from_engine(engine).into_inner();

        let mut output_seckey = internal_seckey.clone();
        output_seckey.tweak_add_assign(&secp, &tweak_value).unwrap();

        let output_pubkey = PublicKey::from_keypair(&secp, &output_seckey);

        let addr = Address::p2tr(output_pubkey, Network::Bitcoin);

        if addr.to_string().get(0..PREFIX.len()) == Some(PREFIX) {
            println!("internal_pubkey: {}", internal_pubkey);
            println!("output_pubkey: {}", output_pubkey);
            println!("Address: {}", addr);
            break;
        }
    }
}
