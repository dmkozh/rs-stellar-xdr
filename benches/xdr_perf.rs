//! Lightweight micro-benchmarks targeting the specific codegen optimizations.
//!
//! These are deliberately simple and NOT representative of end-to-end system
//! performance — a win here may not be visible e2e. They exist only to confirm
//! that an optimization moves the needle on the operation it targets. Absolute
//! numbers are noisy on machines with CPU frequency scaling; only trust
//! comparisons made within a single run (same clock).
//!
//! Run with: `cargo bench --bench xdr_perf`

use std::hint::black_box;
use std::rc::Rc;
use std::time::Instant;

use stellar_xdr::{ContractCodeEntry, LedgerHeader, Limits, ReadXdr, ReadXdrRc, WriteXdr};

/// Reports the *minimum* per-iteration time over `ROUNDS` rounds. The minimum
/// is the least-interrupted run, far more stable than the mean on a noisy
/// machine: a scheduler hiccup can only ever make a round slower.
fn bench<T>(name: &str, iters: u32, mut f: impl FnMut() -> T) {
    const ROUNDS: u32 = 8;
    for _ in 0..iters.min(10_000) {
        black_box(f());
    }
    let mut best = f64::INFINITY;
    for _ in 0..ROUNDS {
        let start = Instant::now();
        for _ in 0..iters {
            black_box(f());
        }
        let per = start.elapsed().as_nanos() as f64 / f64::from(iters);
        best = best.min(per);
    }
    println!("{name:<42} {best:>12.1} ns/iter (min of {ROUNDS})");
}

fn main() {
    let limits = Limits::none;

    // --- Large bytes (Wasm-like): the RcBytes backing + zero-copy decode ---
    let mut cce = ContractCodeEntry::default();
    cce.code = vec![0xABu8; 100_000].try_into().unwrap();
    let cce_bytes = cce.to_xdr(limits()).unwrap();
    let cce_buf: Rc<[u8]> = Rc::from(cce_bytes.as_slice());
    println!("# ContractCodeEntry, code = {} bytes", cce.code.len());
    bench("ContractCodeEntry stream decode (copies)", 5_000, || {
        ContractCodeEntry::from_xdr(&cce_bytes, limits()).unwrap()
    });
    bench("ContractCodeEntry buffer decode (zero-copy)", 50_000, || {
        ContractCodeEntry::from_xdr_with_buffer(Rc::clone(&cce_buf), limits()).unwrap()
    });
    bench("ContractCodeEntry encode", 5_000, || {
        cce.to_xdr(limits()).unwrap()
    });
    bench("ContractCodeEntry clone (now O(1))", 1_000_000, || cce.clone());

    // --- Scalar/nesting heavy: depth-hoist + inline target ---
    let lh = LedgerHeader::default();
    let lh_bytes = lh.to_xdr(limits()).unwrap();
    println!("# LedgerHeader, encoded = {} bytes", lh_bytes.len());
    bench("LedgerHeader stream decode", 200_000, || {
        LedgerHeader::from_xdr(&lh_bytes, limits()).unwrap()
    });
    bench("LedgerHeader encode", 200_000, || lh.to_xdr(limits()).unwrap());
    bench("LedgerHeader clone", 1_000_000, || lh.clone());
}
