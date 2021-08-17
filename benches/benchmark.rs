use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, SamplingMode};
// use std::time::Duration;
use trabalho::encriptador::gerador_chaves::{gerar_chaves, Chaves};
use trabalho::encriptador::{decriptar_texto, encriptar_texto, forca_bruta};

fn keys_gen_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Key gen sizes");
    group.significance_level(0.1).sample_size(10);
    group.sampling_mode(SamplingMode::Flat);
    // group.measurement_time(Duration::new(22, 0));

    let tamanho_min = 64u64;
    let tamanho_max = 1024u64;
    for tamanho in (tamanho_min..=tamanho_max).step_by(64usize) {
        group.bench_with_input(
            BenchmarkId::new("Benchmark KeyGen ", tamanho),
            &tamanho,
            |b, tamanho| b.iter(|| gerar_chaves(black_box(*tamanho))),
        );
    }
    group.finish();
}

fn encript_benchmarck(c: &mut Criterion) {
    let mut group = c.benchmark_group("Encript");
    group.significance_level(0.1).sample_size(10);
    group.sampling_mode(SamplingMode::Flat);
    // group.measurement_time(Duration::new(15, 0));

    let tamanho_min = 64u64;
    let tamanho_max = 1024u64;
    let mensagem:&String = &"Lorem ipsum quisque vestibulum quisque semper suscipit netus molestie, tempus tristique at ultricies nisl quam primis nec, per suscipit odio cursus rhoncus mattis inceptos. tellus mattis est dapibus hac quisque pharetra posuere vivamus, est porttitor nam erat lectus enim aliquet vulputate malesuada, himenaeos donec amet libero tincidunt donec class. per fringilla pulvinar quam eu rhoncus molestie fermentum praesent netus phasellus, libero potenti vivamus bibendum curabitur fames neque orci habitasse commodo taciti, phasellus litora consectetur accumsan proin accumsan hac quisque orci. vehicula condimentum turpis tempor posuere at maecenas facilisis, cursus tortor nisi justo tempus nibh, inceptos eleifend donec in sollicitudin donec.".to_owned();
    for tamanho in (tamanho_min..=tamanho_max).step_by(64usize) {
        let chave: Chaves = gerar_chaves(tamanho);
        group.bench_function(BenchmarkId::new("Encript ", tamanho), |b| {
            b.iter(|| encriptar_texto(black_box(mensagem), black_box(&chave.publica)))
        });
    }
    group.finish();
}

fn decript_benchmarck(c: &mut Criterion) {
    let mut group = c.benchmark_group("Decript");
    group.significance_level(0.1).sample_size(10);
    group.sampling_mode(SamplingMode::Flat);
    // group.measurement_time(Duration::new(15, 0));

    let tamanho_min = 64u64;
    let tamanho_max = 1024u64;
    let mensagem:&String = &"Lorem ipsum quisque vestibulum quisque semper suscipit netus molestie, tempus tristique at ultricies nisl quam primis nec, per suscipit odio cursus rhoncus mattis inceptos. tellus mattis est dapibus hac quisque pharetra posuere vivamus, est porttitor nam erat lectus enim aliquet vulputate malesuada, himenaeos donec amet libero tincidunt donec class. per fringilla pulvinar quam eu rhoncus molestie fermentum praesent netus phasellus, libero potenti vivamus bibendum curabitur fames neque orci habitasse commodo taciti, phasellus litora consectetur accumsan proin accumsan hac quisque orci. vehicula condimentum turpis tempor posuere at maecenas facilisis, cursus tortor nisi justo tempus nibh, inceptos eleifend donec in sollicitudin donec.".to_owned();
    for tamanho in (tamanho_min..=tamanho_max).step_by(64usize) {
        let chave: &Chaves = &gerar_chaves(tamanho);
        let mensagem_enc = encriptar_texto(mensagem, &chave.publica);
        group.bench_function(BenchmarkId::new("Decript ", tamanho), |b| {
            b.iter(|| decriptar_texto(&mensagem_enc, chave));
        });
    }
    group.finish();
}

fn brute_force_benchmarck(c: &mut Criterion) {
    let mut group = c.benchmark_group("Brute Force");
    group.significance_level(0.1).sample_size(5);
    group.sampling_mode(SamplingMode::Flat);
    // group.measurement_time(Duration::new(15, 0));

    let tamanho_min = 8u64;
    let tamanho_max = 56u64;
    let mensagem:&String = &"Lorem ipsum quisque vestibulum quisque semper suscipit netus molestie, tempus tristique at ultricies nisl quam primis nec, per suscipit odio cursus rhoncus mattis inceptos. tellus mattis est dapibus hac quisque pharetra posuere vivamus, est porttitor nam erat lectus enim aliquet vulputate malesuada, himenaeos donec amet libero tincidunt donec class. per fringilla pulvinar quam eu rhoncus molestie fermentum praesent netus phasellus, libero potenti vivamus bibendum curabitur fames neque orci habitasse commodo taciti, phasellus litora consectetur accumsan proin accumsan hac quisque orci. vehicula condimentum turpis tempor posuere at maecenas facilisis, cursus tortor nisi justo tempus nibh, inceptos eleifend donec in sollicitudin donec.".to_owned();
    for tamanho in (tamanho_min..=tamanho_max).step_by(8usize) {
        let chave: &Chaves = &gerar_chaves(tamanho);
        let mensagem_enc = encriptar_texto(mensagem, &chave.publica);
        group.bench_with_input(
            BenchmarkId::new("Brute Force ", tamanho),
            &tamanho,
            |b, tamanho| {
                b.iter(|| forca_bruta(&mensagem_enc, &chave.publica, *tamanho));
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    keys_gen_benchmark,
    encript_benchmarck,
    decript_benchmarck,
    brute_force_benchmarck
);
criterion_main!(benches);
