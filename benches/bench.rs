use criterion::{criterion_group, criterion_main, Criterion};
use dirs::home_dir;
use stonks::parse::parse_toml_file;
use stonks::utils::rem_first_and_last_char;

fn criterion_benchmark(c: &mut Criterion) {
    let home: String = format!("{:?}", home_dir().unwrap());
    let path: String = format!("{}/.config/stonks.toml", rem_first_and_last_char(home));
    c.bench_function("parse_toml_file", |p| {
        p.iter(|| parse_toml_file(path.clone()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
