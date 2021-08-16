mod encriptador;
use encriptador::gerador_chaves::{gerar_chaves, Chaves};

use num_bigint::ToBigInt;

fn main() {
    // Chaves
    let chaves: Chaves = gerar_chaves::<1024u64>();
    println!("{0}", chaves);
    // Encriptar
    let plain_text = 9815273896512u64.to_bigint().unwrap();
    println!("Texto a encriptar {0}", plain_text);

    let text = encriptador::encriptar(&plain_text, &chaves.publica);
    println!("Texto encriptado {0}", text);
    // Decriptar
    let decp = encriptador::decriptar(&text, &chaves);
    println!("Texto descriptografado {0}", decp);
    // força bruta
}
