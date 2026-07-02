# Deception-Resistant Presence Proof (DRPP) Simulation

This repository contains the simulation, evaluation framework, and results generation for the "Deception-Resistant Presence Proof (DRPP): A Cryptographic Protocol for Human-Centric Authentication" paper extension.

## Prerequisites

- Rust toolchain (stable ≥ 1.78) via `rustup`.

## Getting Started

To build and run all experiments, generating figures, tables, and the final results report deterministically (randomness is seeded):

```bash
make all
```

To run unit tests:

```bash
make test
```

To clean generated outputs:

```bash
make clean
```

## Figure and Table Index

See `report/results_report.md` for a full breakdown of the generated figures and tables. All randomness is seeded for full reproducibility.
