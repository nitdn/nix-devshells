use criterion::{Criterion, criterion_group, criterion_main};

fn bench_string_concat_tech(c: &mut Criterion) {
    let mut group = c.benchmark_group("String concatenation technologies");
    let str_0 = "hello";
    let str_1 = " world";
    let input = &(str_0, str_1);
    group.bench_with_input("Plus operator", input, |b, (strclosure, str)| {
        b.iter(|| strclosure.to_string() + str)
    });
    group.bench_with_input("Format macro", input, |b, (string_1, str_1)| {
        b.iter(|| format!("{string_1}{str_1}"))
    });
    group.bench_with_input("What even is this", input, |b, (str0, str1)| {
        b.iter(|| -> String { [*str0, *str1].concat() })
    });
    group.finish();
}

criterion_group!(benches, bench_string_concat_tech);
criterion_main!(benches);
