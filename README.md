# omniverse_rs

<p align="left">
  <a href="https://crates.io/crates/omniverse">                                        <img alt="crates.io"  src="https://img.shields.io/crates/v/omniverse.svg"></a>
  <!-- <a href="https://docs.rs/omniverse">                                                 <img alt="docs.rs"    src="https://docs.rs/omniverse/badge.svg"></a> -->
  <a href="https://github.com/AndrejOrsula/omniverse_rs/actions/workflows/rust.yml">   <img alt="Rust"       src="https://github.com/AndrejOrsula/omniverse_rs/actions/workflows/rust.yml/badge.svg"></a>
  <a href="https://github.com/AndrejOrsula/omniverse_rs/actions/workflows/docker.yml"> <img alt="Docker"     src="https://github.com/AndrejOrsula/omniverse_rs/actions/workflows/docker.yml/badge.svg"></a>
  <a href="https://deps.rs/repo/github/AndrejOrsula/omniverse_rs">                     <img alt="deps.rs"    src="https://deps.rs/repo/github/AndrejOrsula/omniverse_rs/status.svg"></a>
  <a href="https://codecov.io/gh/AndrejOrsula/omniverse_rs">                           <img alt="codecov.io" src="https://codecov.io/gh/AndrejOrsula/omniverse_rs/branch/main/graph/badge.svg"></a>
</p>

Rust interface for NVIDIA [Omniverse](https://www.nvidia.com/en-us/omniverse).

## Status

This project is in early development and is not ready for production use. Not all of the Omniverse Kit API is currently exposed.

Documentation and examples are currently lacking but will be the focus once the crates are more stable.

## Overview

The workspace contains these packages:

- **[omniverse_sys](omniverse_sys):** Unsafe Rust bindings for Omniverse
- **[omniverse](omniverse):** Safe Rust bindings for Omniverse (WIP)

Most of the bindings are automatically generated from the Omniverse headers using [autocxx](https://github.com/google/autocxx), while some are hand-written via [rust-cpp](https://github.com/mystor/rust-cpp). The Omniverse Kit can be automatically downloaded during the cargo build process if the `vendored` feature is enabled.

## Dependencies

The complete list of dependencies can be found within [`Dockerfile`](Dockerfile).

## Instructions

### <a href="#-rust"><img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="16" height="16"></a> Rust

Add `omniverse` as a Rust dependency to your [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html) manifest.

```toml
[dependencies]
omniverse = { version = "0.2" }
```

Note that the first build might take up to 1 hour because OpenUSD and Omniverse Kit will be automatically downloaded with the `vendored` feature enabled (most of the time is spent compiling OpenUSD). The artifacts will be cached in `OUT_DIR` and reused for subsequent builds.

Alternatively, you can specify the paths to existing OpenUSD and Omniverse Kit installation directories via the following environment variables.

```bash
export OPENUSD_PATH=/path/to/pxr/openusd
export CARB_APP_PATH=/path/to/omniverse/kit
```

It is highly recommended to use `lld` or `mold` linker because `ld` might currently fail.

<details>
<summary><h3><a href="#-docker"><img src="https://www.svgrepo.com/show/448221/docker.svg" width="16" height="16"></a> Docker</h3></summary>

> To install [Docker](https://docs.docker.com/get-docker) on your system, you can run [`.docker/host/install_docker.bash`](.docker/host/install_docker.bash) to configure Docker with NVIDIA GPU support.
>
> ```bash
> .docker/host/install_docker.bash
> ```

By running the Docker container, you are implicitly agreeing to the [NVIDIA Omniverse EULA](https://docs.omniverse.nvidia.com/platform/latest/common/NVIDIA_Omniverse_License_Agreement.html). If you do not agree to this license agreement, do not use this container.

#### Build Image

To build a new Docker image from [`Dockerfile`](Dockerfile), you can run [`.docker/build.bash`](.docker/build.bash) as shown below.

```bash
.docker/build.bash ${TAG:-latest} ${BUILD_ARGS}
```

#### Run Container

To run the Docker container, you can use [`.docker/run.bash`](.docker/run.bash) as shown below.

```bash
.docker/run.bash ${TAG:-latest} ${CMD}
```

#### Run Dev Container

To run the Docker container in a development mode (source code mounted as a volume), you can use [`.docker/dev.bash`](.docker/dev.bash) as shown below.

```bash
.docker/dev.bash ${TAG:-latest} ${CMD}
```

As an alternative, VS Code users familiar with [Dev Containers](https://code.visualstudio.com/docs/devcontainers/containers) can modify the included [`.devcontainer/devcontainer.json`](.devcontainer/devcontainer.json) to their needs. For convenience, [`.devcontainer/open.bash`](.devcontainer/open.bash) script is available to open this repository as a Dev Container in VS Code.

```bash
.devcontainer/open.bash
```

#### Join Container

To join a running Docker container from another terminal, you can use [`.docker/join.bash`](.docker/join.bash) as shown below.

```bash
.docker/join.bash ${CMD:-bash}
```

</details>

## Disclaimer

This project is not affiliated with NVIDIA Corporation.

## License

This project is dual-licensed to be compatible with the Rust project, under either the [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) licenses.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
