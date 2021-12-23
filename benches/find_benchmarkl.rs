use criterion::{criterion_group, criterion_main, Criterion};
use find::find_impl::*;

fn file_exist() {
    let where_to_find = String::from("/home/mikhail/git");
    let what_to_find = String::from("init.sh");

    let expected: Vec<&str> = vec!["/home/mikhail/git/dotfiles/init.sh"];

     // find(Config{path: where_to_find.as_ref(), target: &*what_to_find });
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("find file", |b| b.iter(|| file_exist()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

