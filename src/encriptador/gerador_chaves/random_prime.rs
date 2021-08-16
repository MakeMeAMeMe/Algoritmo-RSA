pub mod euclides_estendido;
use euclides_estendido::euclides_estendido;

use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::One;
use rand::thread_rng;

pub fn generate_prime(size: u64) -> BigInt {
    let mut rng = thread_rng();
    let mut num;
    let min = BigInt::one() << (size - 1);
    loop {
        num = &min + &rng.gen_bigint_range(&One::one(), &min);
        if teste_primalidade(&num, 20) {
            break;
        }
    }
    return num;
}

// Teste de Fernout
fn teste_primalidade(num: &BigInt, qtd_tests: u32) -> bool {
    let mut rng = thread_rng();
    for _ in 1..qtd_tests {
        let a: &BigInt = &rng.gen_bigint_range(&3.to_bigint().unwrap(), &num);
        let mdc = euclides_estendido(a, num).mdc;
        if mdc != One::one() {
            return false;
        }
        let p_minus = &num.checked_sub(&One::one()).unwrap();
        let iv = a.modpow(p_minus, num);
        if iv != One::one() {
            return false;
        }
    }
    return true;
}
