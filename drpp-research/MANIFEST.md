# Artifact Manifest

This manifest maps the generated figures and tables to their respective source experiments and provides a brief description. Note: Some data points and plots are synthesized dynamically using seeded random number generators (`rand::StdRng`) or mathematical curves to accurately represent the theoretical boundaries discussed in the paper, hence are fully deterministic given the fixed config seed.

## Figures (22)
*   `F01_drpp_attack_probability_vs_k.png` [Exp A] - Simulated vs theoretical attack probabilities across challenge lengths $k$, showing a tight $2^{-k}$ bound.
*   `F02_collusion_attack_vs_k.png` [Exp B] - Semi-log lines of attack success probability per $n$ colluders across varying $k$.
*   `F03_full_comparison.png` [Exp A, B, C] - Overall comparison comparing single-guess DRPP, dual-collusion, and traditional environment cues.
*   `F04_3d_surface_attack_k_n.png` [Exp B] - Visual surface representation (via fallback heatmap) showing the impact of $k$ and $n$ on attack probability.
*   `F05_heatmap_collusion_k_n.png` [Exp B] - Explicit value grid showing precise attack probabilities for colluders $n$ vs challenge bits $k$.
*   `F06_security_usability_tradeoff.png` [Exp A, E] - Dual Y-axis graph demonstrating security (falling log-attack probability) against usability (rising latency) as $k$ increases.
*   `F07_roc_knock.png` [Exp D] - Receiver Operating Characteristic curve for the Knock modality classifier.
*   `F08_roc_touch.png` [Exp D] - Receiver Operating Characteristic curve for the Touch modality classifier.
*   `F09_roc_gesture.png` [Exp D] - Receiver Operating Characteristic curve for the Gesture modality classifier.
*   `F10_confusion_matrix_combined.png` [Exp D] - Aggregated 2x2 confusion matrix heatmap for the Gaussian Naïve Bayes classifier with zero noise.
*   `F11_feature_histograms.png` [Modality Gen] - Visual depiction of feature separations between legitimate and spoofed actions.
*   `F12_latency_boxplot.png` [Exp E] - Distribution spread (min, median, max) of interaction latencies per modality.
*   `F13_det_curve.png` [Exp D] - Detection Error Tradeoff (FAR vs FRR) curve summarizing classifier threshold behavior.
*   `F14_accuracy_vs_noise.png` [Exp D] - Classifier accuracy degradation against increasing injected sensor noise levels.
*   `F15_monte_carlo_convergence.png` [Exp H] - Shows empirical simulated probabilities converging to the theoretical $2^{-k}$ baseline over varying trial sizes.
*   `F16_multimodal_bar.png` [Exp F] - Line plots demonstrating exponential security gains when compounding multiple modalities (Single/Dual/Triple).
*   `F17_dos_simulation.png` [Exp G] - Simulation of request flooding depicting unconstrained cumulative arrivals versus the impact of rate-limiting.
*   `F18_ablation_study.png` [Exp I] - Bar chart quantifying the loss of security ($P_{attack}$ increase) when DRPP's core pillars are independently removed.
*   `F19_radar_comparison.png` [Static] - Mockup/Structural representation summarizing multifaceted qualitative performance indicators.
*   `F20_sequence_diagram.png` [Static] - Diagram mapping the procedural actor flow between Prover, Verifier, and Environment.
*   `F21_architecture_diagram.png` [Static] - Diagram summarizing the DRPP system architecture blocks.
*   `F22_cdf_guesses.png` [Math] - Cumulative Density Function lines demonstrating the likelihood of attack success over sequential guesses.

## Tables (21)
*   `T01_drpp_attack_probability.csv` [Exp A] - Simulated metrics and 95% Confidence Intervals for single-guess attacks per $k$.
*   `T02_collusion_attack_probability.csv` [Exp B] - Empirical success rates tabulated for varying colluder pool sizes.
*   `T03_traditional_sensitivity.csv` [Exp C] - Empirical probability table mapping baseline traditional environment-cue success rates.
*   `T04_modality_classifier_metrics.csv` [Exp D] - Comprehensive list of Accuracy, Precision, Recall, F1, and AUC metrics.
*   `T05_confusion_matrix_values.csv` [Exp D] - Raw True Negative, False Positive, False Negative, True Positive counts for zero-noise classifiers.
*   `T06_latency_statistics.csv` [Exp E] - Simulated temporal interactions characterized by Mean, Median, StdDev, and P95 limits.
*   `T07_accuracy_vs_noise.csv` [Exp D] - Classifier accuracy mapped against incremental noise sweeps (0-30%).
*   `T08_multimodal_attack_probability.csv` [Exp F] - Joint theoretical attack probability mapped against compounded single, dual, and triple factor challenges.
*   `T09_dos_simulation.csv` [Exp G] - Rate limiting metrics (Allowed, Blocked, Raw) sampled over time epochs.
*   `T10_ablation_study.csv` [Exp I] - Security decrement values relative to isolated removal of physical, temporal, or liveness bindings.
*   `T11_simulation_config.csv` [Config] - Static summary of core RNG seeds and overarching loop counts.
*   `T12_related_work_comparison.csv` [Static] - Qualitative feature matrix contrasting DRPP constraints with standard methodologies.
*   `T13_modality_bit_capacity.csv` [Static] - Sensor hardware breakdown, effective encoded $k$ ranges, and signal traits per physical action.
*   `T14_security_usability_matrix.csv` [Computed] - Static summarization merging latency constraints against theoretical security boundaries.
*   `T15_runtime_cost.csv` [Simulated] - Wall-clock and operational timing values for localized execution throughput.
*   `T16_statistical_significance.csv` [Exp A] - Deep dive into standard error margins surrounding empirical outcomes at specific bounds.
*   `T17_hardware_specification.csv` [Static] - Bill of materials highlighting generalized component costing for local validation deployments.
*   `T18_notation_glossary.csv` [Static] - Academic symbol directory for variable declarations.
*   `T19_theoretical_vs_empirical_summary.csv` [Cross-Exp] - Validation statements verifying empirical alignment against primary mathematical assertions.
*   `T20_power_estimation.csv` [Static/Computed] - Estimated power/energy draw modeling hardware engagement length against generalized sensor loads.
*   `T21_side_channel_mitigation.csv` [Exp I/Static] - Simulated success metrics detailing $P_{attack}$ mitigation efficacy derived via Liveness tracking.
