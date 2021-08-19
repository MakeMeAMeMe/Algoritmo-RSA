pub mod euclides_estendido;
use euclides_estendido::euclides_estendido;

use num_bigint::{BigInt, RandBigInt, ToBigInt};
use num_traits::One;
use rand::thread_rng;

pub fn generate_prime(size: u64) -> BigInt {
    let mut rng = thread_rng(); // O(1)
    let mut num;
    let min = BigInt::one() << (size - 1); // O(n)
    loop { // rnd
        num = &min + &rng.gen_bigint_range(&One::one(), &min); // O(n)
        if teste_primalidade(&num, 20) {
            break;
        }
    }
    return num;
}

// Teste de Fermat
pub fn teste_primalidade(num: &BigInt, qtd_tests: u32) -> bool { // O(qtd_tests * n^2)
    let mut rng = thread_rng(); // O(1)
    for _ in 1..qtd_tests { // O(qtd_testes) -> 20
        let a: &BigInt = &rng.gen_bigint_range(&3.to_bigint().unwrap(), &num); // O(n)
        let mdc = euclides_estendido(a, num).mdc; // O(n^2)
        if mdc != One::one() { // O(n)
            return false;
        }
        let p_minus = &num.checked_sub(&One::one()).unwrap(); // O(n)
        let iv = a.modpow(p_minus, num); // O(log^2(n)) 
        if iv != One::one() { // O(n)
            return false;
        }
    }
    return true;
}
