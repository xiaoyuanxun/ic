use criterion::*;
use ic_crypto_internal_threshold_sig_ecdsa::*;

fn poly_bench(c: &mut Criterion) {
    let curve = EccCurveType::K256;

    let mut rng = rand::thread_rng();

    for degree in [8, 16, 32] {
        let poly = Polynomial::random(curve, degree, &mut rng);

        let x = EccScalar::random(curve, &mut rng);

        c.bench_function(
            &format!("poly evaluate_at({}, degree {})", curve, degree),
            |b| {
                b.iter(|| {
                    let _ = poly.evaluate_at(&x);
                })
            },
        );

        let mut samples = Vec::with_capacity(degree + 1);
        for _i in 0..degree + 1 {
            let r = EccScalar::random(curve, &mut rng);
            let p_r = poly.evaluate_at(&r).unwrap();
            samples.push((r, p_r));
        }

        c.bench_function(
            &format!("poly interpolate({}, degree {})", curve, degree),
            |b| {
                b.iter(|| {
                    let p = Polynomial::interpolate(curve, &samples).unwrap();
                    assert_eq!(p, poly);
                })
            },
        );

        let poly_b = Polynomial::random(curve, degree, &mut rng);

        c.bench_function(
            &format!("poly simple commitment({}, degree {})", curve, degree),
            |b| {
                b.iter(|| {
                    let _ = SimpleCommitment::create(&poly, degree).unwrap();
                })
            },
        );

        c.bench_function(
            &format!("poly Pedersen commitment({}, degree {})", curve, degree),
            |b| {
                b.iter(|| {
                    let _ = PedersenCommitment::create(&poly, &poly_b, degree).unwrap();
                })
            },
        );
    }
}

criterion_group!(benches, poly_bench);
criterion_main!(benches);
