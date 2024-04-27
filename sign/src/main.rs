use anyhow::anyhow;
use openssl::{hash::MessageDigest, pkey::PKey, rsa::Padding, sign::Signer};
use std::{
    fs::{self, File},
    io::{Read, Write},
};
fn main() -> anyhow::Result<()> {
    let mut file = File::open("hello.txt")?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    println!("{}", String::from_utf8_lossy(&data));

    let private_key_pem = fs::read("./rsa_private_key.pem")?;
    let private_key = PKey::private_key_from_pem_passphrase(&private_key_pem, b"123456")?;

    if private_key.private_key_to_pem_pkcs8()?.is_empty() {
        println!("read private key err!");
        return Err(anyhow!("Failed to read private key"));
    } else {
        println!("read private key ok!");
    }

    let hash = openssl::hash::hash(MessageDigest::md5(), &data)?;

    let mut signer = Signer::new(MessageDigest::md5(), &private_key)?;
    signer.set_rsa_padding(Padding::PKCS1)?;
    signer.update(&hash)?;
    let signature = signer.sign_to_vec()?;

    let mut file = File::create("hello.sign")?;
    file.write_all(&signature)?;

    println!("OK");
    Ok(())
}
