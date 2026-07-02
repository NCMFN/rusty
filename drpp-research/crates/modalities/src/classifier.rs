#![allow(clippy::needless_range_loop)]
#![allow(clippy::useless_vec)]
use std::f64::consts::PI;

pub struct LogisticRegression {
    weights: Vec<f64>,
    bias: f64,
}

impl LogisticRegression {
    #[allow(clippy::new_without_default)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            weights: Vec::new(),
            bias: 0.0,
        }
    }

    pub fn fit(&mut self, x_train: &[Vec<f64>], y_train: &[f64]) {
        if x_train.is_empty() {
            return;
        }

        let n_features = x_train[0].len();
        self.weights = vec![0.0; n_features];
        self.bias = 0.0;

        let lr = 0.1;
        let epochs = 500;
        let lambda = 0.01; // L2 regularization
        let m = x_train.len() as f64;

        for _ in 0..epochs {
            let mut dw = vec![0.0; n_features];
            let mut db = 0.0;

            for i in 0..x_train.len() {
                let z = self.dot(&x_train[i], &self.weights) + self.bias;
                let a = sigmoid(z);
                let dz = a - y_train[i];

                for j in 0..n_features {
                    dw[j] += x_train[i][j] * dz;
                }
                db += dz;
            }

            for j in 0..n_features {
                dw[j] = (dw[j] / m) + (lambda / m) * self.weights[j];
                self.weights[j] -= lr * dw[j];
            }
            db /= m;
            self.bias -= lr * db;
        }
    }

    pub fn predict_proba(&self, x: &[f64]) -> f64 {
        sigmoid(self.dot(x, &self.weights) + self.bias)
    }

    fn dot(&self, a: &[f64], b: &[f64]) -> f64 {
        a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
    }
}

pub struct GaussianNaiveBayes {
    class_priors: Vec<f64>,
    class_means: Vec<Vec<f64>>,
    class_vars: Vec<Vec<f64>>,
}

impl GaussianNaiveBayes {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            class_priors: Vec::new(),
            class_means: Vec::new(),
            class_vars: Vec::new(),
        }
    }

    pub fn fit(&mut self, x_train: &[Vec<f64>], y_train: &[f64]) {
        if x_train.is_empty() {
            return;
        }

        let n_features = x_train[0].len();

        let mut class_counts = vec![0; 2];
        let mut class_sums = vec![vec![0.0; n_features]; 2];

        for (i, x) in x_train.iter().enumerate() {
            let c = y_train[i] as usize;
            class_counts[c] += 1;
            for j in 0..n_features {
                class_sums[c][j] += x[j];
            }
        }

        self.class_priors = class_counts
            .iter()
            .map(|&c| c as f64 / x_train.len() as f64)
            .collect();
        self.class_means = vec![vec![0.0; n_features]; 2];
        for c in 0..2 {
            for j in 0..n_features {
                if class_counts[c] > 0 {
                    self.class_means[c][j] = class_sums[c][j] / class_counts[c] as f64;
                }
            }
        }

        self.class_vars = vec![vec![1e-9; n_features]; 2]; // small epsilon for variance
        for (i, x) in x_train.iter().enumerate() {
            let c = y_train[i] as usize;
            for j in 0..n_features {
                let diff = x[j] - self.class_means[c][j];
                self.class_vars[c][j] += diff * diff;
            }
        }
        for c in 0..2 {
            for j in 0..n_features {
                if class_counts[c] > 1 {
                    self.class_vars[c][j] /= (class_counts[c] - 1) as f64;
                }
            }
        }
    }

    pub fn predict_proba(&self, x: &[f64]) -> f64 {
        let mut log_probs = vec![0.0; 2];
        for c in 0..2 {
            log_probs[c] = self.class_priors[c].ln();
            for j in 0..x.len() {
                let var = self.class_vars[c][j];
                let mean = self.class_means[c][j];
                let _std = var.sqrt();

                // log of Gaussian PDF
                let term1 = -0.5 * ((2.0 * PI).ln() + var.ln());
                let term2 = -((x[j] - mean).powi(2)) / (2.0 * var);
                log_probs[c] += term1 + term2;
            }
        }

        // softmax
        let max_log = log_probs[0].max(log_probs[1]);
        let exp_probs: Vec<f64> = log_probs.iter().map(|&p| (p - max_log).exp()).collect();
        let sum_exp = exp_probs[0] + exp_probs[1];

        exp_probs[1] / sum_exp
    }
}

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

#[derive(Debug, Clone, Default)]
pub struct Metrics {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1: f64,
    pub auc: f64,
    pub tp: u32,
    pub tn: u32,
    pub fp: u32,
    pub fn_: u32,
}

pub fn calculate_metrics(y_true: &[f64], y_prob: &[f64], threshold: f64) -> Metrics {
    let mut tp = 0;
    let mut tn = 0;
    let mut fp = 0;
    let mut fn_ = 0;

    for (&true_val, &prob) in y_true.iter().zip(y_prob.iter()) {
        let pred = if prob >= threshold { 1.0 } else { 0.0 };
        if true_val == 1.0 && pred == 1.0 {
            tp += 1;
        } else if true_val == 0.0 && pred == 0.0 {
            tn += 1;
        } else if true_val == 0.0 && pred == 1.0 {
            fp += 1;
        } else {
            fn_ += 1;
        }
    }

    let accuracy = (tp + tn) as f64 / (tp + tn + fp + fn_) as f64;
    let precision = if tp + fp > 0 {
        tp as f64 / (tp + fp) as f64
    } else {
        0.0
    };
    let recall = if tp + fn_ > 0 {
        tp as f64 / (tp + fn_) as f64
    } else {
        0.0
    };
    let f1 = if precision + recall > 0.0 {
        2.0 * (precision * recall) / (precision + recall)
    } else {
        0.0
    };

    // AUC via Trapezoidal rule on ROC
    let mut threshold_points = Vec::new();
    let num_thresholds = 200;
    for i in 0..=num_thresholds {
        let t = i as f64 / num_thresholds as f64;
        let mut t_tp = 0;
        let mut t_tn = 0;
        let mut t_fp = 0;
        let mut t_fn = 0;

        for (&true_val, &prob) in y_true.iter().zip(y_prob.iter()) {
            let pred = if prob >= t { 1.0 } else { 0.0 };
            if true_val == 1.0 && pred == 1.0 {
                t_tp += 1;
            } else if true_val == 0.0 && pred == 0.0 {
                t_tn += 1;
            } else if true_val == 0.0 && pred == 1.0 {
                t_fp += 1;
            } else {
                t_fn += 1;
            }
        }

        let tpr = if t_tp + t_fn > 0 {
            t_tp as f64 / (t_tp + t_fn) as f64
        } else {
            0.0
        };
        let fpr = if t_fp + t_tn > 0 {
            t_fp as f64 / (t_fp + t_tn) as f64
        } else {
            0.0
        };
        threshold_points.push((fpr, tpr));
    }

    threshold_points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut auc = 0.0;
    for i in 1..threshold_points.len() {
        let (fpr_prev, tpr_prev) = threshold_points[i - 1];
        let (fpr_curr, tpr_curr) = threshold_points[i];
        auc += (fpr_curr - fpr_prev) * (tpr_curr + tpr_prev) / 2.0;
    }

    Metrics {
        accuracy,
        precision,
        recall,
        f1,
        auc,
        tp,
        tn,
        fp,
        fn_,
    }
}
