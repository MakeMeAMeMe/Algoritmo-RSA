use trabalho::encriptador;
use trabalho::encriptador::gerador_chaves::{gerar_chaves, Chaves};

fn main() {
    // Chaves
    let chaves: Chaves = gerar_chaves(256u64);
    println!("{0}", chaves);
    // Encriptar
    let plain_text = "Texto Secreto".to_owned();
    println!("Texto a encriptar: {0}", plain_text);
    let text = encriptador::encriptar_texto(&plain_text, &chaves.publica);
    println!("Texto encriptado {0}", text);
    // Decriptar
    let decp = encriptador::decriptar_texto(&text, &chaves);
    println!("Texto descriptografado {0}", decp);
    // força bruta
}
