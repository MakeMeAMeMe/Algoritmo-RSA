use num_bigint::{BigInt, RandBigInt};
use num_traits::One;
use rand::thread_rng;
use std::fmt;

pub mod random_prime;
use random_prime::euclides_estendido::{euclides_estendido, inverso_modular};
use random_prime::generate_prime;

pub struct ChavePrivada {
    pub p: BigInt,
    pub q: BigInt,
    pub d: BigInt,
}

impl fmt::Display for ChavePrivada {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Chave Privada:\np: {p}\nq: {q}\nd: {d}",
            p = self.p,
            q = self.q,
            d = self.d
        )
    }
}

pub struct ChavePublica {
    pub n: BigInt,
    pub e: BigInt,
}

impl fmt::Display for ChavePublica {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Chave Pública:\nn: {n}\ne: {e}",
            n = self.n,
            e = self.e
        )
    }
}

pub struct Chaves {
    pub privada: ChavePrivada,
    pub publica: ChavePublica,
}

impl fmt::Display for Chaves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{chavep}\n-------------------------------------\n{chavepr}",
            chavep = self.publica,
            chavepr = self.privada
        )
    }
}

pub fn gerar_chaves(bits: u64) -> Chaves {
    // chave publica = (n, e)
    // chave privada = (p, q, d)

    // Gerar primos p e q
    let p = generate_prime(bits / 2); // p e q devem ter a metade do tamanho de n, d e e
    let q = generate_prime(bits / 2); // p e q devem ter a metade do tamanho de n, d e e

    // Gerar n
    let n = &p * &q;

    // Gerar e
    let p_minus: BigInt = p.checked_sub(&One::one()).unwrap();
    let q_minus: BigInt = q.checked_sub(&One::one()).unwrap();
    let z: BigInt = p_minus * q_minus;

    let full_min = BigInt::one() << (bits / 2 - 1);

    let mut rng = thread_rng();
    let mut max = z.clone();
    let mut min: BigInt = rng.gen_bigint_range(&full_min, &max);
    let mut num = min.clone();
    let e: BigInt;
    loop {
        if euclides_estendido(&num, &z).mdc.is_one() {
            e = num;
            break;
        } else if num.eq(&max) {
            max = min.clone();
            min = rng.gen_bigint_range(&full_min, &max);
            num = min.clone();
        } else {
            num = num.checked_add(&One::one()).unwrap();
        }
    }

    // Gerar d
    let d = inverso_modular(&e, &z);
    Chaves {
        publica: ChavePublica { n: n, e: e },
        privada: ChavePrivada { p: p, q: q, d: d },
    }
}
