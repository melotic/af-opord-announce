use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};
use opord_parse::opord_parser::OpordParser;

pub fn parse_opord_bench(c: &mut Criterion) {
    let parser = OpordParser::new(Path::new("Week 1.txt"));
    c.bench_function("OpordParser::parse", |b| b.iter(|| parser.parse()));
}

criterion_group!(benches, parse_opord_bench);
criterion_main!(benches);
