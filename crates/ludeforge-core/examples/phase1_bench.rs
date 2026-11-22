use ludeforge_core::{load, normalize, validate};
use std::hint::black_box;
use std::time::Instant;

const LANTERN: &[u8] = include_bytes!("../../../examples/lantern-academy/project.json");

fn measure(mut operation: impl FnMut()) -> Vec<u128> {
    operation();
    (0..7)
        .map(|_| {
            let start = Instant::now();
            operation();
            start.elapsed().as_micros()
        })
        .collect()
}

fn report(name: &str, n: usize, bytes: usize, samples: &[u128]) {
    let mut sorted = samples.to_vec();
    sorted.sort_unstable();
    println!(
        "{name}: recruits={n} bytes={bytes} median_us={} samples_us={samples:?}",
        sorted[sorted.len() / 2]
    );
}

fn main() {
    for n in [100, 1000, 10000] {
        let mut p = load(LANTERN).expect("original fixture");
        let recruit = p
            .objects
            .iter()
            .find(|o| o.id == "neri")
            .expect("fixture recruit")
            .clone();
        p.objects.retain(|o| o.id != "neri");
        for i in 0..n {
            let mut row = recruit.clone();
            row.id = format!("recruit{i}");
            p.objects.push(row);
        }
        let canonical = normalize(&p).expect("valid generated fixture");
        let decoded = load(&canonical).expect("round trip");
        assert_eq!(normalize(&decoded).unwrap(), canonical);
        let validate_us = measure(|| validate(black_box(&decoded)).unwrap());
        let load_us = measure(|| {
            black_box(load(black_box(&canonical)).unwrap());
        });
        let normalize_us = measure(|| {
            black_box(normalize(black_box(&decoded)).unwrap());
        });
        report("validate", n, canonical.len(), &validate_us);
        report("decode_validate", n, canonical.len(), &load_us);
        report("normalize", n, canonical.len(), &normalize_us);
    }
}
