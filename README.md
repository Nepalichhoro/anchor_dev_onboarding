# anchor_dev_onboarding
# Setup a Anchor Solana Dev Environment

# Install Rust and Cargo
- To be able to compile Rust based Solana programs, install the Rust language and Cargo (the Rust package manager) using Rustup:
```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
# Install the Solana CLI
- Install the Solana CLI tool running command
```
    sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
```
# Run your localhost validator 
- The Solana CLI comes with the test validator built in. This command line tool will allow you to run a full blockchain cluster on your machine.
```
    solana-test-validator
```

Configure your Solana CLI to use your localhost validator for all your future terminal commands and Solana program deployment:
```
    solana config set --url localhost
```

# Install Anchor
- Anchor is a framework for Solana development. It is quite similar to hardhat in many respects.

```
    cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
    avm install latest
    avm use latest
```

# Create a new Rust library with Anchor
- Solana programs written in Rust are libraries which are compiled to BPF bytecode and saved in the .so format.

## Initialize a new Rust library named hello_world via the Cargo command line:
```
    anchor init hello_world_anchor
    cd hello_world_anchor
    anchor build
```

## Sync program_id with the anchor key and  run test

```
    anchor keys sync
    anchor test --skip-local-validator
```

# Create your first Anchor program
The code for your Anchor based Solana program will live in your src/lib.rs file. Inside src/lib.rs you will be able to import your Rust crates and define your logic. Open your src/lib.rs file in your favorite editor.


```
use anchor_lang::prelude::*;

declare_id!("...");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!"); 
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

```

# Build your Anchor program
Inside a terminal window, you can build your Solana Rust program by running in the root of your project (i.e. the directory with your Cargo.toml file):

```
    anchor build
```


# Example log output
```
    Streaming transaction logs mentioning EL3zioyKuHaox1ra3PzKKjKvdNtMHh71paufgkYYPr5V. Confirmed commitment
    Transaction executed in slot 33867:
      Signature: K3bNBFm5WXroHKLSPAvciwcshr1c8oxYprEoBEppsDK1avS9CWPomVSWM1gWRkX4XeftWEaLdvfnmpf91xB4jxq
      Status: Ok
      Log Messages:
        Program EL3zioyKuHaox1ra3PzKKjKvdNtMHh71paufgkYYPr5V invoke [1]
        Program log: Instruction: Initialize
        Program log: Hello, world!
        Program EL3zioyKuHaox1ra3PzKKjKvdNtMHh71paufgkYYPr5V consumed 323 of 200000 compute units
        Program EL3zioyKuHaox1ra3PzKKjKvdNtMHh71paufgkYYPr5V success

```
# Remarks

```
If you run into rustup toolchain related issue then please uninstall rust using brew and proceed with curl link

Also while deploying, if you are reminded of not having enough fund then run solana airdrop <desired_amount>
```
