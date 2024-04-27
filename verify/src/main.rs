use openssl::{hash::MessageDigest, pkey::PKey, sign::Verifier};
use std::{
    fs::{self, File},
    io::Read,
};
fn main() -> anyhow::Result<()> {
    let mut file = File::open("./hello.txt")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    println!("{}", String::from_utf8_lossy(&data));

    let public_key_pem = fs::read("./rsa_public_key.pem")?;
    let public_key = PKey::public_key_from_pem(&public_key_pem)?;

    let mut file = File::open("./hello.sign")?;
    let mut signature = Vec::new();
    file.read_to_end(&mut signature)?;

    let hash = openssl::hash::hash(MessageDigest::md5(), &data)?;

    let mut verifier = Verifier::new(MessageDigest::md5(), &public_key)?;
    verifier.update(&hash)?;
    let is_signature_valid = verifier.verify(&signature)?;

    if is_signature_valid {
        println!("\nRSA_verify OK!");
    } else {
        println!("RSA_verify err!");
        return Err(anyhow::anyhow!("Failed to verify the signature"));
    }

    Ok(())
}
