use std::fs;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use png::{Decoder, Transformations};

fn load_all(c: &mut Criterion) {
    for file in fs::read_dir("tests/benches/").unwrap() {
        if let Ok(entry) = file {
            match entry.path().extension() {
                Some(st) if st == "png" => {}
                _ => continue,
            }

            let data = fs::read(entry.path()).unwrap();
            bench_file_and_normalize(c, &data, entry.file_name().into_string().unwrap());
            bench_file(c, &data, entry.file_name().into_string().unwrap());
        }
    }
}

criterion_group!(benches, load_all);
criterion_main!(benches);

fn bench_file(c: &mut Criterion, data: &[u8], name: String) {
    let mut group = c.benchmark_group("decode");
    group.sample_size(20);

    let decoder = Decoder::new(&*data);
    let mut reader = decoder.read_info().unwrap();
    let mut image = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut image).unwrap();

    group.throughput(Throughput::Bytes(info.buffer_size() as u64));
    group.bench_with_input(name, data, |b, data| {
        b.iter(|| {
            let decoder = Decoder::new(data);
            let mut decoder = decoder.read_info().unwrap();
            decoder.next_frame(&mut image).unwrap();
        })
    });
}

fn bench_file_and_normalize(c: &mut Criterion, data: &[u8], name: String) {
    let mut group = c.benchmark_group("normalized");
    group.sample_size(20);

    let mut decoder = Decoder::new(&*data);
    decoder.set_transformations(Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().unwrap();
    let mut image = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut image).unwrap();

    group.throughput(Throughput::Bytes(info.buffer_size() as u64));
    group.bench_with_input(name, data, |b, data| {
        b.iter(|| {
            let mut decoder = Decoder::new(data);
            decoder.set_transformations(Transformations::normalize_to_color8());
            let mut decoder = decoder.read_info().unwrap();
            decoder.next_frame(&mut image).unwrap();
        })
    });
}
