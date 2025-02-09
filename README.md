# NavAbilitySDK.rs

Copyright 2025, NavAbility(TM) Contributors.  This repo is licensed according to Apache 2.0.  See the LICENSE file.

[![CI](https://github.com/NavAbility/NavAbilitySDK.rs/actions/workflows/ci-rs.yml/badge.svg)](https://github.com/NavAbility/NavAbilitySDK.rs/actions/workflows/ci-rs.yml)

## Introduction

Access NavAbility(TM) Accelerator features from Rust.  See related multi-language SDKs at Github.com/NavAbility/NavAbilitySDK.*.

## Docs

Documentation for [Python](https://navability.github.io/NavAbilitySDK.py/) or [Julia](https://navability.github.io/NavAbilitySDK.jl/dev/) versions exist, work in progress to port Docs for Rust crates (25Q1).

## Compiling

Get deps
```shell
make install-deps # modifies system cargo crates
```

Set required NVA_API_URL and NVA_API_TOKEN args/env variables and compile for either native or wasm:
```shell
make build-wasm
make build-tokio
```

## Running tests

```shell
make test-tokio
```
