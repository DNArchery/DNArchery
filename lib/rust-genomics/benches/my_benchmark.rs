use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rust_genomics::{Sequence, FASTA};

pub fn criterion_benchmark(c: &mut Criterion) {
    /*c.bench_function("gen seq", |b| b.iter(|| {
        let long_sequence = Sequence::gen_random_seq(black_box(100000));
    }));*/

    /*let long_sequence = Sequence::gen_random_seq(black_box(100000));

    c.bench_function("normal lorf", |b| b.iter(|| {
        long_sequence.find_lorf();
    }));
    c.bench_function("concurrent lorf", |b| b.iter(|| {
        long_sequence.concurrent_find_lorf();
    }));*/

    /*let sequences = [
        Sequence::gen_random_seq(black_box(100)), // concurrent: 32.756 us, 34.096 us
        Sequence::gen_random_seq(black_box(1000)), // concurrent: 40.622 us, 41.343 us
        Sequence::gen_random_seq(black_box(10000)), // concurrent: 107.72 us, 116.41 us
        Sequence::gen_random_seq(black_box(100000)), // concurrent: 1.8189 ms, 2.1511 ms
        Sequence::gen_random_seq(black_box(100000)), // concurrent: 1.8598 ms, norm: 2.1466 ms
    ];

    let mut norm_group = c.benchmark_group("normal lorf group");
    for seq in sequences.iter() {
        norm_group.bench_with_input(BenchmarkId::from_parameter(seq), seq, |b, seq| {
            b.iter(|| {
                seq.concurrent_find_lorf();
            });
        });
    }
    norm_group.finish();

    let mut con_group = c.benchmark_group("concurrent lorf group");
    for seq in sequences.iter() {
        con_group.bench_with_input(BenchmarkId::from_parameter(seq), seq, |b, seq| {
            b.iter(|| {
                seq.concurrent_find_lorf();
            });
        });
    }
    con_group.finish();*/

    /*c.bench_function("slow fasta", |b| b.iter(|| {
        FASTA::slow_read_fasta("data/haha-1.fasta"); //2.9770 ms
    }));

    c.bench_function("normal fasta", |b| b.iter(|| {
        FASTA::read_fasta("data/haha-1.fasta"); //441.20 us
    }));

    c.bench_function("rayon fasta", |b| b.iter(|| {
        FASTA::rayon_read_fasta("data/haha-1.fasta"); //2.5719 ms
    }));*/

    let mut fasta = FASTA::rayon_read_fasta("data/sars_cov_2.fa");

    // typically the larger the file, the more worth to use concurrency 
    c.bench_function("integration", |b| b.iter(|| {
        fasta.find_lorfs(false); 
    }));

    c.bench_function("integration concurrent", |b| b.iter(|| {
        fasta.find_lorfs(true); 
    }));
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
