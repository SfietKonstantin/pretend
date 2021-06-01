# pretend

[![Build](https://github.com/SfietKonstantin/pretend/workflows/ci/badge.svg)](https://github.com/SfietKonstantin/pretend/actions)
[![codecov](https://codecov.io/gh/SfietKonstantin/pretend/branch/main/graph/badge.svg)](https://codecov.io/gh/SfietKonstantin/pretend)

`pretend` is a modular, [Feign]-inspired HTTP, client based on macros. It's goal is to decouple
the definition of a REST API from it's implementation.


Some features:
- Declarative
- Asynchronous-first implementations
- HTTP client agnostic
- JSON support thanks to serde

[Feign]: https://github.com/OpenFeign/feign

This repository contains the code for [`pretend`](pretend/README.md) and 
[`pretend-codegen`](pretend-codegen/README.md) as well as [`pretend-reqwest`](pretend-reqwest/README.md) 
[`pretend-ishac`](pretend-isahc/README.md) and [`pretend-awc`](pretend-awc/README.md).

MSRV for the `pretend` ecosystem is Rust **1.44**.
