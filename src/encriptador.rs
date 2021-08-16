pub mod gerador_chaves;
use gerador_chaves::{ChavePublica, Chaves};
use num_bigint::BigInt;
use num_bigint::ToBigInt;
use std::fmt::Write;

pub fn encriptar(content: &BigInt, chave_publica: &ChavePublica) -> BigInt {
    return content.modpow(&chave_publica.e, &chave_publica.n);
}

pub fn encriptar_texto(content: &String, chave_publica: &ChavePublica) -> Vec<BigInt> {
    let mut texto_encriptado: Vec<BigInt> = Vec::<BigInt>::new();
    for character in content.chars() {
        let char_as_num = (character as u32).to_bigint().unwrap();
        texto_encriptado.push(encriptar(&char_as_num, &chave_publica));
    }
    return texto_encriptado;
}

pub fn decriptar(content: &BigInt, chaves: &Chaves) -> BigInt {
    return content.modpow(&chaves.privada.d, &chaves.publica.n);
}

pub fn decriptar_texto(content: &Vec<BigInt>, chaves: &Chaves) -> String {
    let mut texto_decriptado: String = "".to_owned();
    for num in content {
        write!(texto_decriptado, "{0}", decriptar(&num, chaves));
    }
    return texto_decriptado;
}
