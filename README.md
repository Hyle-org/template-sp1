# Template for a Hyle-SP1 smart contract

This basic implementation is based on "counter" contract, that increment / decrement a value.

## Prerequisites

- [Install Rust](https://www.rust-lang.org/tools/install) (you'll need `rustup` and Cargo).
- [SP1 4.1.x](https://docs.succinct.xyz/getting-started/install.html)
- Run a local devnet node:

Clone the [hyle](https://github.com/Hyle-org/hyle) repo, checkout the version you need, and run:
```sh 
export SP1_PROVER=mock
cargo run -F sp1 -- --pg
```

Note: you need the "sp1" feature on the node: it is enabled with `-F sp1`

## Quickstart

### Build and register the contract

To build and register the smart contract on the local node, run:

```bash
cargo run -- register-contract
```

The expected output on the node is `📝 Registering contract counter`.


### Executing the Project Locally in Development Mode

During development, faster iteration upon code changes can be achieved by leveraging [dev-mode], we strongly suggest activating it during your early development phase. 

```bash
SP1_PROVER=mock cargo run
```

### Execute the contract & send a tx on-chain

```sh
SP1_PROVER=mock cargo run -- increment
```


## Directory Structure

It is possible to organize the files for these components in various ways.
However, in this starter template we use a standard directory structure for zkVM
applications, which we think is a good starting point for your applications.

```text
project_name
├── Cargo.toml
├── lib
│   ├── Cargo.toml
│   └── src
│       └── lib.rs         <-- [Contract code goes here, common to program & script]
├── program
│   ├── Cargo.toml
│   └── src
│       └── main.rs        <-- [Program code goes here (runs in the zkvm)]
└── script
    ├── Cargo.toml
    ├── build.rs
    └── src
        └── bin
            └── main.rs    <-- [Script code goes here (aka application backend)]
```

<!--[bonsai access]: https://bonsai.xyz/apply-->
[rust-toolchain]: rust-toolchain.toml
[rustup]: https://rustup.rs
