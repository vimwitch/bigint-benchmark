use num_bigint::{BigUint};
use std::time::{Instant, Duration};
use ibig::{modular::{Modulo, ModuloRing}, UBig};
use crypto_bigint::{U128};
use crypto_bigint::{modular::runtime_mod::{DynResidue, DynResidueParams}};

fn main() {
    let p = 1_u128 + 407_u128 * 2_u128.pow(119);
    let g = 85408008396924667383611388730472331217_u128;
    // n modular muls

    // n modular adds

    // n modular subs
    // let (expected, time) = test_num_bigint_modexp(p);
    // println!("bigint modexp: {}", time.as_millis());
    let (expected_bigint_mul, time) = test_num_bigint_mul(p, g);
    println!("bigint mul: {} ms", time.as_millis());
    let (expected_ibig_mul, time) = test_ibig_mul(p, g);
    println!("ibig mul: {} ms", time.as_millis());
    let (expected_crypto_mul, time) = test_crypto_bigint_mul(p, g);
    println!("crypto mul: {} ms", time.as_millis());

    for (i, v) in expected_bigint_mul.iter().enumerate() {
        assert_eq!(*v, expected_ibig_mul[i]);
        assert_eq!(*v, expected_crypto_mul[i]);
    }
}

fn test_crypto_bigint_mul(p_: u128, g_: u128) -> (Vec<u128>, Duration) {
    // let m = residue!()
    let g = U128::from_u128(g_);
    let p = DynResidueParams::new(&U128::from_u128(p_));
    // start timing
    let now = Instant::now();
    let mut muls: Vec<DynResidue<2>> = Vec::new();
    muls.push(DynResidue::new(&g, p));
    for i in 1..10000000_usize {
        muls.push(muls[i-1].mul(&DynResidue::new(&g, p)));
    }
    // end timing
    let elapsed = now.elapsed();
    let out = muls.iter().map(|v| {
        let mut o = 0_u128;
        let words = v.retrieve().to_words();
        o += u128::try_from(words[0]).unwrap();
        o += u128::try_from(words[1]).unwrap() << 64;
        o
    }).collect();
    (out, elapsed)
}

fn test_ibig_mul(p_: u128, g_: u128) -> (Vec<u128>, Duration) {
    let ring = ModuloRing::new(&UBig::from(p_));
    let g = ring.from(g_);
    // start timing
    let now = Instant::now();
    let mut muls: Vec<Modulo> = Vec::new();
    muls.push(g.clone());
    for i in 1..10000000_usize {
        muls.push(&muls[i-1] * &g);
    }
    // end timing
    let elapsed = now.elapsed();
    let out = muls.iter().map(|v| le_bytes_to_u128(&v.residue().to_le_bytes())).collect();
    (out, elapsed)
}

fn test_num_bigint_mul(p_: u128, g_: u128) -> (Vec<u128>, Duration) {
    let p = BigUint::from(p_);
    let g = BigUint::from(g_);
    // start timing
    let now = Instant::now();
    let mut muls: Vec<BigUint> = Vec::new();
    muls.push(g.clone());
    for i in 1..10000000_usize {
        muls.push((&muls[i-1] * &g) % &p);
    }
    // end timing
    let elapsed = now.elapsed();
    let out = muls.iter().map(|v| le_bytes_to_u128(&v.to_bytes_le())).collect();
    (out, elapsed)
}

fn test_num_bigint_modexp(p_: u128) -> (Vec<u128>, Duration) {
    let p = BigUint::from(p_);
    let mut bases: Vec<BigUint> = Vec::new();
    for i in 1..1000 {
        bases.push(BigUint::from(i as u32));
    }
    // start timing
    let now = Instant::now();
    let mut pows: Vec<BigUint> = Vec::new();
    for v in &bases {
        for v2 in &bases {
            pows.push(v.modpow(&v2, &p));
        }
    }
    // end timing
    let elapsed = now.elapsed();
    let out = pows.iter().map(|v| le_bytes_to_u128(&v.to_bytes_le())).collect();
    (out, elapsed)
}

fn le_bytes_to_u128(bytes: &[u8]) -> u128 {
    let mut out = 0_u128;
    for (i, v) in bytes.iter().enumerate() {
        out += (*v as u128) << (8*i);
    }
    out
}
