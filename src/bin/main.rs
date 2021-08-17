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
    println!("Texto encriptado: {0}", text);

    // Decriptar
    let decp = encriptador::decriptar_texto(&text, &chaves);
    println!("Texto descriptografado: {0}", decp);

    // Chaves bf
    let chavesbf: Chaves = gerar_chaves(32);
    println!("{0}", chavesbf);

    // Encriptar
    let plain_text_bf = "Texto Secreto".to_owned();
    println!("Texto a encriptar: {0}", plain_text_bf);
    let text_bf = encriptador::encriptar_texto(&plain_text_bf, &chavesbf.publica);
    println!("Texto encriptado: {0}", text_bf);

    // Decriptar
    let decp_text_bf = encriptador::decriptar_texto(&text_bf, &chavesbf);
    println!("Texto descriptografado: {0}", decp_text_bf);

    // força bruta
    let decp_bf = encriptador::forca_bruta(&text_bf, &chavesbf.publica, 32);
    println!("Texto descriptografado por força bruta: {0}", decp_bf);
}
