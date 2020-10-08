# MKL02Z4 Peripheral access crate

## Introduction

Low-level register mappings for the Kinetis KL02 series microcontrollers in Rust, generated automatically from SVD file for MKL02Z4 from [MCUXpresso SDK] using [svd2rust].

## Usage

Just add crate to the dependencies in Cargo.toml:
```
MKL02Z4 = {git = "https://github.com/siralexeyduke/MKL02Z4"}
```

## Documentation

For the detailed information about microcontroller peripherals please refer to the [KL02 Sub-Family Reference Manual].

[MCUXpresso SDK]: https://mcuxpresso.nxp.com/en/welcome
[svd2rust]: https://crates.io/crates/svd2rust
[KL02 Sub-Family Reference Manual]: https://www.nxp.com/docs/en/reference-manual/KL02P20M48SF0RM.pdf