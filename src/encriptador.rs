pub mod gerador_chaves;
use gerador_chaves::{ChavePublica, Chaves};
use num_bigint::BigInt;
use num_bigint::ToBigInt;
use std::fmt::Write;

pub fn encriptar(content: &BigInt, chave_publica: &ChavePublica) -> BigInt {
    return content.modpow(&chave_publica.e, &chave_publica.n);
}

pub fn encriptar_texto(content: &String, chave_publica: &ChavePublica) -> String {
    let data = content.as_bytes();
    let mut texto_encriptado: String = String::new();
    for byte in data {
        let text_as_num = ToBigInt::to_bigint(byte).unwrap();
        let num_encriptado = encriptar(&text_as_num, chave_publica);
        write!(texto_encriptado, "{0}-", num_encriptado.to_str_radix(16)).ok();
    }
    return texto_encriptado.strip_suffix("-").unwrap().to_owned();
}

pub fn decriptar(content: &BigInt, chaves: &Chaves) -> BigInt {
    return content.modpow(&chaves.privada.d, &chaves.publica.n);
}

pub fn decriptar_texto(content: &String, chaves: &Chaves) -> String {
    let data = content.split("-");
    let mut texto_decriptado = String::new();
    for byte in data {
        let byte_str = byte.to_owned();
        let bytes = byte_str.as_bytes();
        let num_decriptado = decriptar(&BigInt::parse_bytes(bytes, 16).unwrap(), chaves);
        let num = num_decriptado.to_u32_digits().1[0];
        write!(texto_decriptado, "{0}", (num as u8 as char)).ok();
    }
    return texto_decriptado;
}
