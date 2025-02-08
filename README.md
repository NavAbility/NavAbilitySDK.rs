# NavAbilitySDK.rs

Copyright 2025, NavAbility(TM) Contributors.  This repo is licensed according to Apache 2.0.  See the LICENSE file.

[![CI](https://github.com/NavAbility/NavAbilitySDK.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/NavAbility/NavAbilitySDK.rs/actions/workflows/ci.yml)

## Introduction

Access NavAbility(TM) Accelerator features from Rust.  See related multi-language SDKs at Github.com/NavAbility/NavAbilitySDK.*.

## Docs

Documentation for [Python](https://navability.github.io/NavAbilitySDK.py/) or [Julia](https://navability.github.io/NavAbilitySDK.jl/dev/) versions exist, work in progress to port Docs for Rust crates (25Q1).

## Compiling

Get the schema with NVA_API_URL and NVA_API_TOKEN args/env var set:
```shell
make install-deps # modifies system cargo crates
make fetch-schema
```

Compile for either native or wasm:
```shell
make build-wasm
make build-tokio
```

## Running tests

```shell
make test-tokio
```

## Exporting Shared Library

Build the shared library:
```shell
make build-lib
```

Or run the `test/test.c` file with
```shell
make test-capi
```