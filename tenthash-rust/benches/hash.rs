use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use tenthash::{hash, TentHash};

//----

fn tent_hash_single_call(c: &mut Criterion) {
    let benches = [
        ("10b_message", 10),       // 10-byte input.
        ("100b_message", 100),     // 100-byte input.
        ("1kb_message", 1000),     // 1-kilobyte input.
        ("10kb_message", 10000),   // 10-kilobyte input.
        ("100kb_message", 100000), // 100-kilobyte input.
        ("1mb_message", 1000000),  // 1-megabyte input.
    ];

    let mut group = c.benchmark_group("tent_hash");

    for (name, data_size) in benches.iter() {
        let data: Vec<u8> = b"abcdefghijklmnopqrstuvwxyz"
            .iter()
            .copied()
            .cycle()
            .take(*data_size)
            .collect();
        group.throughput(Throughput::Bytes(*data_size as u64));

        group.bench_function(*name, |bench| {
            bench.iter(|| {
                let _ = hash(&data);
            })
        });
    }
}

fn tent_hash_streaming(c: &mut Criterion) {
    let benches = [
        ("10b_chunks", 10),     // 10-byte chunks.
        ("100b_chunks", 100),   // 100-byte chunks.
        ("1kb_chunks", 1000),   // 1-kilobyte chunks.
        ("10kb_chunks", 10000), // 10-kilobyte chunks.
    ];

    let mut group = c.benchmark_group("tent_hash_streaming");

    for (name, chunk_size) in benches.iter() {
        let data_size = 100000;
        let data: Vec<u8> = b"abcdefghijklmnopqrstuvwxyz"
            .iter()
            .copied()
            .cycle()
            .take(data_size)
            .collect();
        group.throughput(Throughput::Bytes(data_size as u64));

        group.bench_function(*name, |bench| {
            bench.iter(|| {
                let mut hash = TentHash::new();
                for chunk in data.chunks(*chunk_size) {
                    hash.update(chunk);
                }
                hash.finalize();
            })
        });
    }
}

//----

criterion_group!(benches, tent_hash_single_call, tent_hash_streaming);
criterion_main!(benches);
