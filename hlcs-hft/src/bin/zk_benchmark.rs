use hlcs_hft::{
    commit_simple, setup,
    zk::{proof_size_bytes, prove_agg, verify_agg}, Q,
};
use rand::{rngs::StdRng, SeedableRng};
use std::time::Instant;
fn main() {
    let mut rng = StdRng::seed_from_u64(42);
    println!("n,prove_ms,verify_ms,proof_KB,soundness_bits,ok");
    for &n in &[128, 256, 512, 768, 1024] {
        let pp = setup(&mut rng, n);
        let m = b"EUR/USD 1.0950";
        let (c, h) = commit_simple(&pp, m, &mut rng);
        let r: Vec<u16> =
            h.r.iter()
                .map(|&x| if x < 0 { (x + Q) as u16 } else { x as u16 })
                .collect();
        let e: Vec<u16> =
            h.e.iter()
                .map(|&x| if x < 0 { (x + Q) as u16 } else { x as u16 })
                .collect();
        let s = Instant::now();
        let pf = prove_agg(&pp, &c, &r, &e, &mut rng);
        let pt = s.elapsed().as_micros() as f64 / 1000.0;
        let s = Instant::now();
        let ok = verify_agg(&pp, &c, &pf);
        let vt = s.elapsed().as_micros() as f64 / 1000.0;
        println!(
            "{},{:.3},{:.3},{:.1},{},{}",
            n,
            pt,
            vt,
            proof_size_bytes(&pp) as f64 / 1024.0,
            136,
            ok
        );
    }
}
