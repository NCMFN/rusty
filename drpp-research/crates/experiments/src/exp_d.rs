use crate::config::Config;
use modalities::classifier::{calculate_metrics, GaussianNaiveBayes, LogisticRegression, Metrics};
use modalities::gesture::generate_gesture_features;
use modalities::knock::generate_knock_features;
use modalities::touch::generate_touch_features;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct ResultD {
    pub modality: String,
    pub classifier: String,
    pub noise_level: f64,
    pub metrics: Metrics,
}

fn standardize(data: &mut [Vec<f64>]) {
    if data.is_empty() {
        return;
    }
    let n_features = data[0].len();

    for j in 0..n_features {
        let mut sum = 0.0;
        for row in data.iter() {
            sum += row[j];
        }
        let mean = sum / data.len() as f64;

        let mut var_sum = 0.0;
        for row in data.iter() {
            var_sum += (row[j] - mean).powi(2);
        }
        let std = (var_sum / data.len() as f64).sqrt().max(1e-9);

        for row in data.iter_mut() {
            row[j] = (row[j] - mean) / std;
        }
    }
}

pub fn run(cfg: &Config) -> anyhow::Result<Vec<ResultD>> {
    let modalities = vec!["knock", "touch", "gesture"];

    let mut tasks = Vec::new();
    for modality in modalities {
        for &noise in &cfg.noise_levels {
            tasks.push((modality.to_string(), noise));
        }
    }

    let results: Vec<ResultD> = tasks
        .into_par_iter()
        .flat_map(|(modality, noise)| {
            let mut rng = StdRng::seed_from_u64(cfg.seed + (noise * 1000.0) as u64);

            let mut x_all = Vec::new();
            let mut y_all = Vec::new();

            let n_legit = cfg.modality_samples / 2;
            let n_spoofed = cfg.modality_samples / 2;

            for _ in 0..n_legit {
                let features = match modality.as_str() {
                    "knock" => generate_knock_features(true, noise, &mut rng),
                    "touch" => generate_touch_features(true, noise, &mut rng),
                    "gesture" => generate_gesture_features(true, noise, &mut rng),
                    _ => unreachable!(),
                };
                x_all.push(features);
                y_all.push(1.0);
            }

            for _ in 0..n_spoofed {
                let features = match modality.as_str() {
                    "knock" => generate_knock_features(false, noise, &mut rng),
                    "touch" => generate_touch_features(false, noise, &mut rng),
                    "gesture" => generate_gesture_features(false, noise, &mut rng),
                    _ => unreachable!(),
                };
                x_all.push(features);
                y_all.push(0.0);
            }

            // Shuffle
            let mut indices: Vec<usize> = (0..cfg.modality_samples).collect();
            indices.shuffle(&mut rng);

            let mut x_shuffled = Vec::with_capacity(cfg.modality_samples);
            let mut y_shuffled = Vec::with_capacity(cfg.modality_samples);
            for &idx in &indices {
                x_shuffled.push(x_all[idx].clone());
                y_shuffled.push(y_all[idx]);
            }

            // 70/30 split
            let train_size = (cfg.modality_samples as f64 * 0.7) as usize;
            let mut x_train = x_shuffled[0..train_size].to_vec();
            let y_train = y_shuffled[0..train_size].to_vec();
            let mut x_test = x_shuffled[train_size..].to_vec();
            let y_test = y_shuffled[train_size..].to_vec();

            standardize(&mut x_train);
            standardize(&mut x_test);

            let mut lr = LogisticRegression::new();
            lr.fit(&x_train, &y_train);
            let lr_probs: Vec<f64> = x_test.iter().map(|x| lr.predict_proba(x)).collect();
            let lr_metrics = calculate_metrics(&y_test, &lr_probs, 0.5);

            let mut gnb = GaussianNaiveBayes::new();
            gnb.fit(&x_train, &y_train);
            let gnb_probs: Vec<f64> = x_test.iter().map(|x| gnb.predict_proba(x)).collect();
            let gnb_metrics = calculate_metrics(&y_test, &gnb_probs, 0.5);

            vec![
                ResultD {
                    modality: modality.clone(),
                    classifier: "LR".to_string(),
                    noise_level: noise,
                    metrics: lr_metrics,
                },
                ResultD {
                    modality: modality.clone(),
                    classifier: "GNB".to_string(),
                    noise_level: noise,
                    metrics: gnb_metrics,
                },
            ]
        })
        .collect();

    Ok(results)
}
