mod gerador_chaves;
use gerador_chaves::{Chaves, gerar_chaves};

fn main() {
    

    // Chaves
    let chaves: Chaves = gerar_chaves::<1024u64>();
    
    println!("{0}", chaves);
    // Encriptar

    // Decriptar

    // força bruta
}
