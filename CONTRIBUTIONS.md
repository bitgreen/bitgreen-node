# ![Bitgreen](./doc/images/bitgreen-logo.png)

## Contributions

Bitgreen primarily uses GitHub Pull Requests to coordinate code changes. If you wish to propose a
contribution, open a pull request and review the template, which explains how to document your
proposal.

### Code style

Bitgreen is following the [Substrate code style](https://github.com/paritytech/polkadot-sdk/blob/master/docs/STYLE_GUIDE.md).

In addition, we incorporate several tools to improve code quality. These are integrated into our CI
and are expected to pass before a PR is considered mergeable. They can also be run locally.

* [clippy](https://github.com/rust-lang/rust-clippy) - run with `cargo clippy --release --workspace`
* [rustfmt](https://github.com/rust-lang/rustfmt) - run with `cargo fmt -- --check`
* [prettier](https://prettier.io/) - run with `npx prettier --check --ignore-path .gitignore '**/*.(yml|js|ts|json)'` (runs against `typescript` code)

### Directory Structure

The following is a list of directories of interest in development.

|Directory              |Purpose                                                                     |
| --------------------- | -------------------------------------------------------------------------- |
|client/                | Debug & Trace related code (rust)                                          |
|docker/                | Dockerfiles for running Bitgreen                                           |
|parachain/             | Bitgreen's main node (rust)                                                |
|pallets/               | Bitgreen's Substrate runtime pallets (rust)                                |
|primitives/            | Base types used in runtime                                                 |
|runtime/               | Bitgreen's runtime (on-chain) code (rust, compiled to WASM)                |
|scripts/               | Utilities for launching and interacting with a Bitgreen chain (typescript) |
|tools/                 | Various tools generally related to development (typescript)                |

### PR labels conventions

Any PR must indicate whether the changes should be part of the runtime changelog or the binary changelog or neither.

If the changes are to be listed in the runtime changelog, associate the label `B7-runtimenoteworthy` with your PR.

If the changes should be listed in the binary changelog, associate the label `B5-clientnoteworthy` with your PR.

If the changes are not to be listed in any changelog, associate the label `B0-silent` with your PR.

# Git branch conventions

For branch conventions related to this git repository,
see [Git branch conventions](docs/git-branches-conventions.md).