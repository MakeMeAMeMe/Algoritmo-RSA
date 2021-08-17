pub mod gerador_chaves;
use gerador_chaves::random_prime::euclides_estendido::inverso_modular;
use gerador_chaves::random_prime::teste_primalidade;
use gerador_chaves::{ChavePrivada, ChavePublica, Chaves};
use num_bigint::BigInt;
use num_bigint::ToBigInt;
use num_integer::Integer;
use num_traits::{One, Zero};
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
        write!(texto_encriptado, "{0}-", num_encriptado.to_str_radix(36)).ok();
    }
    return texto_encriptado.strip_suffix("-").unwrap().to_owned();
}

pub fn decriptar(content: &BigInt, chaves: &Chaves) -> BigInt {
    return content.modpow(&chaves.privada.d, &chaves.publica.n);
}

pub fn decriptar_texto(content: &String, chaves: &Chaves) -> String {
    let data = content.split("-");
    let mut texto_decriptado = String::new();
    for num in data {
        let num_str = num.to_owned();
        let bytes = num_str.as_bytes();
        let num_decriptado = decriptar(&BigInt::parse_bytes(bytes, 36).unwrap(), chaves);
        let num = num_decriptado.to_u32_digits().1[0];
        write!(texto_decriptado, "{0}", (num as u8 as char)).ok();
    }
    return texto_decriptado;
}

pub fn forca_bruta(content: &String, chave: &ChavePublica, size: u64) -> String {
    let mut q: BigInt = BigInt::one() << ((size - 1) / 2);
    q = q.checked_add(&One::one()).unwrap();
    let passo = &2.to_bigint().unwrap();
    while q.lt(&chave.n) {
        if chave.n.mod_floor(&q).eq(&Zero::zero()) {
            if teste_primalidade(&q, 20) {
                let p = chave.n.div_floor(&q);
                if teste_primalidade(&p, 20) {
                    let q_minus: BigInt = q.checked_sub(&One::one()).unwrap();
                    let p_minus: BigInt = p.checked_sub(&One::one()).unwrap();
                    let z = p_minus * q_minus;
                    let d = inverso_modular(&chave.e, &z);
                    return decriptar_texto(
                        content,
                        &Chaves {
                            publica: ChavePublica {
                                e: chave.e.clone(),
                                n: chave.n.clone(),
                            },
                            privada: ChavePrivada { d: d, p: p, q: q },
                        },
                    );
                }
            }
        }
        // println!("Teste falhou para p: {0}", p);
        q = q.checked_add(passo).unwrap();
    }
    return "Erro".to_owned();
}
