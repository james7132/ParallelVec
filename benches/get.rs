use criterion::{black_box, criterion_group, criterion_main, Criterion};

use parallel_vec::ParallelVec;
use rand::*;

#[derive(Clone, Copy, Default)]
struct Small(u32);
#[derive(Clone, Copy, Default)]
struct Big([u64; 32]);

trait Inc {
    fn inc(&mut self);
}

impl Inc for Small {
    fn inc(&mut self) {
        self.0 += 1;
    }
}

impl Inc for Big {
    fn inc(&mut self) {
        for i in 0..32 {
            self.0[i] += 1;
        }
    }
}

fn bench_get_2(c: &mut Criterion, size: usize) {
    let small = (Small(0), Small(1));
    let mut rng = rand::thread_rng();
    let mut vec = Vec::from(vec![small]).repeat(size);
    c.bench_function(&format!("get_vec_small_2x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
            }
        })
    });
    let mut vec = ParallelVec::from(vec![small]).repeat(size);
    c.bench_function(&format!("get_parallelvec_small_2x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
            }
        })
    });
    let mixed = (Big::default(), Small(1));
    let mut vec = Vec::from(vec![mixed]).repeat(size);
    c.bench_function(&format!("get_vec_mixed_2x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
            }
        })
    });
    let mut vec = ParallelVec::from(vec![mixed]).repeat(size);
    c.bench_function(&format!("get_parallelvec_mixed_2x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
            }
        })
    });
    let big = (Big::default(), Big::default());
    let mut vec = Vec::from(vec![big]).repeat(size);
    c.bench_function(&format!("get_vec_big_2x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
            }
        })
    });
    let mut vec = ParallelVec::from(vec![big]).repeat(size);
    c.bench_function(&format!("get_parallelvec_big_2x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
            }
        })
    });
}

fn bench_get_3(c: &mut Criterion, size: usize) {
    let small = (Small(0), Small(1), Small(2));
    let mut rng = rand::thread_rng();
    let mut vec = Vec::from(vec![small]).repeat(size);
    c.bench_function(&format!("get_vec_small_3x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
            }
        })
    });
    let mut vec = ParallelVec::from(vec![small]).repeat(size);
    c.bench_function(&format!("get_parallelvec_small_3x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
            }
        })
    });
    let mixed = (Big::default(), Small(1), Big::default());
    let mut vec = Vec::from(vec![mixed]).repeat(size);
    c.bench_function(&format!("get_vec_mixed_3x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
            }
        })
    });
    let mut vec = ParallelVec::from(vec![mixed]).repeat(size);
    c.bench_function(&format!("get_parallelvec_mixed_3x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
            }
        })
    });
    let big = (Big::default(), Big::default(), Big::default());
    let mut vec = Vec::from(vec![big]).repeat(size);
    c.bench_function(&format!("get_vec_big_3x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
            }
        })
    });
    let mut vec = ParallelVec::from(vec![big]).repeat(size);
    c.bench_function(&format!("get_parallelvec_big_3x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
            }
        })
    });
}

fn bench_get_4(c: &mut Criterion, size: usize) {
    let small = (Small(0), Small(1), Small(2), Small(3));
    let mut rng = rand::thread_rng();
    let mut vec = Vec::from(vec![small]).repeat(size);
    c.bench_function(&format!("get_vec_small_4x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c, d)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
                black_box(d).inc();
            }
        })
    });
    let mut vec = ParallelVec::from(vec![small]).repeat(size);
    c.bench_function(&format!("get_parallelvec_small_4x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c, d)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
                black_box(d).inc();
            }
        })
    });
    let mixed = (Big::default(), Small(1), Big::default(), Small(2));
    let mut vec = Vec::from(vec![mixed]).repeat(size);
    c.bench_function(&format!("get_vec_mixed_4x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c, d)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
                black_box(d).inc();
            }
        })
    });
    let mut vec = ParallelVec::from(vec![mixed]).repeat(size);
    c.bench_function(&format!("get_parallelvec_mixed_4x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c, d)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
                black_box(d).inc();
            }
        })
    });
    let big = (Big::default(), Big::default(), Big::default(), Big::default());
    let mut vec = Vec::from(vec![big]).repeat(size);
    c.bench_function(&format!("get_vec_big_4x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c, d)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
                black_box(d).inc();
            }
        })
    });
    let mut vec = ParallelVec::from(vec![big]).repeat(size);
    c.bench_function(&format!("get_parallelvec_big_4x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c, d)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
                black_box(d).inc();
            }
        })
    });
}

fn bench_get_5(c: &mut Criterion, size: usize) {
    let small = (Small(0), Small(1), Small(2), Small(3), Small(4));
    let mut rng = rand::thread_rng();
    let mut vec = Vec::from(vec![small]).repeat(size);
    c.bench_function(&format!("get_vec_small_5x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c, d, e)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
                black_box(d).inc();
                black_box(e).inc();
            }
        })
    });
    let mut vec = ParallelVec::from(vec![small]).repeat(size);
    c.bench_function(&format!("get_parallelvec_small_5x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c, d, e)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
                black_box(d).inc();
                black_box(e).inc();
            }
        })
    });
    let mixed = (Big::default(), Small(1), Big::default(), Small(2), Big::default());
    let mut vec = Vec::from(vec![mixed]).repeat(size);
    c.bench_function(&format!("get_vec_mixed_5x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c, d, e)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
                black_box(d).inc();
                black_box(e).inc();
            }
        })
    });
    let mut vec = ParallelVec::from(vec![mixed]).repeat(size);
    c.bench_function(&format!("get_parallelvec_mixed_5x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c, d, e)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
                black_box(d).inc();
                black_box(e).inc();
            }
        })
    });
    let big = (Big::default(), Big::default(), Big::default(), Big::default(), Big::default());
    let mut vec = Vec::from(vec![big]).repeat(size);
    c.bench_function(&format!("get_vec_big_5x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c, d, e)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
                black_box(d).inc();
                black_box(e).inc();
            }
        })
    });
    let mut vec = ParallelVec::from(vec![big]).repeat(size);
    c.bench_function(&format!("get_parallelvec_big_5x_{}", size), |b| {
        b.iter(|| {
            if let Some((a, b, c, d, e)) = black_box(vec.get_mut(rng.next_u32() as usize % size)) {
                black_box(a).inc();
                black_box(b).inc();
                black_box(c).inc();
                black_box(d).inc();
                black_box(e).inc();
            }
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    for size in [1000, 100000, 1000000] {
        bench_get_2(c, size);
        bench_get_3(c, size);
        bench_get_4(c, size);
        bench_get_5(c, size);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
