use openssl::{rsa::Rsa, symm::Cipher};
use std::{fs::File, io::Write};
fn main() -> anyhow::Result<()> {
    let rsa = Rsa::generate(2048)?;

    let public_key_pem = rsa.public_key_to_pem()?;
    let private_key_pem = rsa.private_key_to_pem_passphrase(Cipher::des_ede3_cbc(), b"123456")?;

    let mut file = File::create("rsa_public_key.pem")?;
    file.write_all(&public_key_pem)?;

    let mut file = File::create("rsa_private_key.pem")?;
    file.write_all(&private_key_pem)?;

    Ok(())
}
