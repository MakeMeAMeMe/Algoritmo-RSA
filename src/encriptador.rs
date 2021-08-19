pub mod gerador_chaves;
use gerador_chaves::random_prime::euclides_estendido::inverso_modular;
use gerador_chaves::random_prime::teste_primalidade;
use gerador_chaves::{ChavePrivada, ChavePublica, Chaves};
use num_bigint::{BigInt, ToBigInt};
use num_integer::Integer;
use num_traits::{One, Zero};
use std::fmt::Write;

pub fn encriptar(content: &BigInt, chave_publica: &ChavePublica) -> BigInt {
    return content.modpow(&chave_publica.e, &chave_publica.n); // O(log^2(n))
}

pub fn encriptar_texto(content: &String, chave_publica: &ChavePublica) -> String {
    let data = content.as_bytes();
    let mut texto_encriptado: String = String::new();
    for byte in data {
        let text_as_num = ToBigInt::to_bigint(byte).unwrap();
        let num_encriptado = encriptar(&text_as_num, chave_publica);
        write!(
            texto_encriptado,
            "{0}#",
            num_encriptado.to_str_radix(36)
        )
        .ok();
    }
    return texto_encriptado.strip_suffix("#").unwrap().to_owned();
}

pub fn decriptar(content: &BigInt, chaves: &Chaves) -> BigInt {
    return content.modpow(&chaves.privada.d, &chaves.publica.n);
}

pub fn decriptar_texto(content: &String, chaves: &Chaves) -> String { // O(qtd_chars_original * log^2(n))
    let data = content.split("#"); // O(qtd_chars_original)
    let mut texto_decriptado = String::new(); // O(1)
    for num in data { // O(qtd_chars_original)
        let num_str = num.to_owned(); // O(m)
        let bytes = num_str.as_bytes(); // O(qtd_chars num_str) O(m)
        let num_decriptado = decriptar(&BigInt::parse_bytes(bytes, 36).unwrap(), chaves); // O(log^2(n))
        let num = num_decriptado.to_u32_digits().1[0]; // O(1)
        write!(texto_decriptado, "{0}", (num as u8 as char)).ok(); // O(m)
    }
    return texto_decriptado;
}

pub fn forca_bruta(content: &String, chave: &ChavePublica, size: u64) -> String { // O(2^n)
    let mut q: BigInt = BigInt::one() << ((size - 1) / 2); // O(n) 1000000000000
    q = q.checked_add(&One::one()).unwrap(); // O(n) 1000000000001
    let passo = &2.to_bigint().unwrap(); // O(n)
    while q.lt(&chave.n) { // |chave.n| = n => Qtd de números possíveis entre q e chave.n cresce 2^n 
        // O(n)
        if chave.n.mod_floor(&q).eq(&Zero::zero()) {
            // O(n)
            if teste_primalidade(&q, 20) { // O(qtd_testes * n^2)
                let p = chave.n.div_floor(&q); // O(n^2)
                if teste_primalidade(&p, 20) { // O(qtd_testes * n^2)
                    let q_minus: BigInt = q.checked_sub(&One::one()).unwrap(); // O(n)
                    let p_minus: BigInt = p.checked_sub(&One::one()).unwrap(); // O(n)
                    let z = p_minus * q_minus; // O(n^1.585)
                    let d = inverso_modular(&chave.e, &z); // O(n^2)
                    return decriptar_texto( //O(qtd_chars_original * log^2(n))
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
        q = q.checked_add(passo).unwrap(); // O(n)
    }
    return "Erro".to_owned();
}
