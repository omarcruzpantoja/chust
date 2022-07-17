
use criterion::{criterion_group, criterion_main, Criterion};

fn replace_existing(mut buffer: Vec<u8>, data: Vec<u8>) {
    buffer.splice(0 .. 100_000, data);
}

fn append(mut empty_buffer: Vec<u8>, mut data: Vec<u8>) {
    empty_buffer.append(&mut data);
    // match n {
    //     0 => 1,
    //     1 => 1,
    //     n => fibonacci(n-1) + fibonacci(n-2),
    // }
}

fn criterion_benchmark(c: &mut Criterion) {
    let empty_buffer: Vec<u8> = Vec::new();
    let data = vec![15 as u8; 100_000];

    let buffer = vec![0 as u8; 100_000];
    let data_replace = vec![15 as u8; 100_000];
    c.bench_function("replace existing", |b| b.iter(|| replace_existing(buffer.clone(), data_replace.clone())));
    c.bench_function("appending", |b| b.iter(|| append(empty_buffer.clone(), data.clone())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);