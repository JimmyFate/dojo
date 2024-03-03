// MIT License

// Copyright (c) 2022 Jonathan LEI

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};
use dojo_world::metadata::dojo_metadata_from_workspace;
use scarb::core::Config;
use starknet_crypto::FieldElement;

use super::options::fee::FeeOptions;
use super::options::signer::SignerOptions;
use super::options::starknet::StarknetOptions;
use crate::ops::account;

#[derive(Debug, Args)]
pub struct AccountArgs {
    #[clap(subcommand)]
    command: AccountCommand,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Subcommand)]
pub enum AccountCommand {
    #[clap(about = "Create a new account configuration without actually deploying.")]
    New {
        #[clap(flatten)]
        signer: SignerOptions,

        #[clap(long, short, help = "Overwrite the account config file if it already exists")]
        force: bool,

        #[clap(help = "Path to save the account config file")]
        output: PathBuf,
    },

    #[clap(about = "Deploy account contract with a DeployAccount transaction.")]
    Deploy {
        #[clap(flatten)]
        starknet: StarknetOptions,

        #[clap(flatten)]
        signer: SignerOptions,

        #[clap(flatten)]
        fee: FeeOptions,

        #[clap(long, help = "Simulate the transaction only")]
        simulate: bool,

        #[clap(long, help = "Provide transaction nonce manually")]
        nonce: Option<FieldElement>,

        #[clap(
            long,
            env = "STARKNET_POLL_INTERVAL",
            default_value = "5000",
            help = "Transaction result poll interval in milliseconds"
        )]
        poll_interval: u64,

        #[clap(help = "Path to the account config file")]
        file: PathBuf,
    },
}

impl AccountArgs {
    pub fn run(self, config: &Config) -> Result<()> {
        let env_metadata = if config.manifest_path().exists() {
            let ws = scarb::ops::read_workspace(config.manifest_path(), config)?;

            dojo_metadata_from_workspace(&ws).and_then(|inner| inner.env().cloned())
        } else {
            None
        };

        config.tokio_handle().block_on(account::execute(self.command, env_metadata))
    }
}
