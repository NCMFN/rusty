use hlcs_core::{binding_bound, las_margin, LasMargin};

pub struct BindingPoint {
    pub q_hash: f64,
    pub collision_bound: f64,
    pub sis_bound_assumed: f64,
}

pub struct LasPoint {
    pub n: usize,
    pub margin: LasMargin,
}

pub struct ExpJResult {
    pub binding_data: Vec<BindingPoint>,
    pub las_data: Vec<LasPoint>,
}

pub fn run(qh_ranges: &[f64], dimensions: &[usize], tau_ms: f64) -> ExpJResult {
    let mut binding_data = Vec::with_capacity(qh_ranges.len());

    for &q_h in qh_ranges {
        let collision = binding_bound(q_h as u64, 512); // fix n=512 for this sweep
                                                        // Assumed SIS bound for n=512, q=12289, sigma=3.2 is computationally intractable
                                                        // We'll use a constant 2^-128 to represent "negligible" for plotting.
        let sis_bound_assumed = 2.0_f64.powi(-128);

        binding_data.push(BindingPoint {
            q_hash: q_h,
            collision_bound: collision,
            sis_bound_assumed,
        });
    }

    let mut las_data = Vec::with_capacity(dimensions.len());
    for &n in dimensions {
        las_data.push(LasPoint {
            n,
            margin: las_margin(n, tau_ms),
        });
    }

    ExpJResult {
        binding_data,
        las_data,
    }
}
