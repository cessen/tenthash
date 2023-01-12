use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tenthash::TentHasher;

//----

fn tent_hash(c: &mut Criterion) {
    let benches = [
        ("10b", 10, 10),                      // 10-byte input.
        ("100b", 100, 100),                   // 100-byte input.
        ("1kb", 1000, 1000),                  // 1kb input.
        ("100kb_1kb_chunks", 100000, 1000),   // 100kb input, processed in 1kb chunks.
        ("100kb_10kb_chunks", 100000, 10000), // 100kb input, processed in 10kb chunks.
    ];

    let mut group = c.benchmark_group("tent_hash");

    for (name, data_size, chunk_size) in benches.iter() {
        let data: Vec<u8> = b"abcdefghijklmnopqrstuvwxyz"
            .iter()
            .copied()
            .cycle()
            .take(*data_size)
            .collect();
        group.throughput(Throughput::Bytes(*data_size as u64));

        group.bench_function(*name, |bench| {
            bench.iter(|| {
                let mut hash = TentHasher::new();
                for chunk in data.chunks(*chunk_size) {
                    hash.update(chunk);
                }
                hash.finalize();
            })
        });
    }
}

//----

criterion_group!(benches, tent_hash);
criterion_main!(benches);
