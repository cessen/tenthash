use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use digest::Digest;
use meowhash::MeowHasher;

const DATA_SIZE: usize = 1000000;

//----

fn hash_bench(c: &mut Criterion) {
    let hashes: &[(&str, &dyn Fn(&[u8]))] = &[
        ("TentHash", &|bytes| {
            let _ = tenthash::hash(bytes);
        }),
        ("Murmur3 128", &|bytes| {
            let _ = fastmurmur3::murmur3_x64_128(bytes, 123456789);
        }),
        ("MeowHash 0.5", &|bytes| {
            let mut hash = MeowHasher::new();
            hash.update(bytes);
            hash.finalize();
        }),
        ("Blake3", &|bytes| {
            let _ = blake3::hash(bytes);
        }),
    ];

    let mut group = c.benchmark_group("tent_hash");

    let data: Vec<u8> = b"abcdefghijklmnopqrstuvwxyz"
        .iter()
        .copied()
        .cycle()
        .take(DATA_SIZE)
        .collect();
    for (name, hash) in hashes.iter() {
        group.throughput(Throughput::Bytes(data.len() as u64));

        group.bench_function(*name, |bench| bench.iter(|| hash(&data)));
    }
}

//----

criterion_group!(benches, hash_bench);
criterion_main!(benches);
