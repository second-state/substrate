// Copyright 2018-2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use structopt::StructOpt;
use sc_service::{
	Configuration, ChainSpecExtension, RuntimeGenesis,
};

use sc_service::config::OffchainWorkerConfig;
use structopt::clap::arg_enum;


use crate::error;

arg_enum! {
	/// Whether off-chain workers are enabled.
	#[allow(missing_docs)]
	#[derive(Debug, Clone)]
	pub enum OffchainWorkerEnabled {
		Always,
		Never,
		WhenValidating,
	}
}

/// Offchain worker related parameters.
#[derive(Debug, StructOpt, Clone)]
pub struct OffchainWorkerParams {

	/// Should execute offchain workers on every block.
	///
	/// By default it's only enabled for nodes that are authoring new blocks.
	#[structopt(
		long = "offchain-worker",
		value_name = "ENABLED",
		possible_values = &OffchainWorkerEnabled::variants(),
		case_insensitive = true,
		default_value = "WhenValidating"
    )]
    pub enabled: OffchainWorkerEnabled,

	/// Allow writing from the runtime to the offchain worker database directly (buffered).
	/// 
	// TODO the argument is way too long
    #[structopt(
        long = "allow-write-to-offchain-worker-db",
        value_name = "ALLOW_RUNTIME_WRITE_TO_OFFCHAIN_WORKER_DB"
    )]
	pub allow_runtime_write_to_offchain_worker_db: bool,
}

impl OffchainWorkerParams {
	/// Load spec to `Configuration` from `OffchainWorkerParams` and spec factory.
	pub fn update_config<'a, G, E>(
		&self,
		mut config: &'a mut Configuration<G, E>,
        role: sc_service::Roles,
	) -> error::Result<()> where
		G: RuntimeGenesis,
		E: ChainSpecExtension,
	{
        let enabled = match (&self.enabled, role) {
			(OffchainWorkerEnabled::WhenValidating, sc_service::Roles::AUTHORITY) => true,
			(OffchainWorkerEnabled::Always, _) => true,
			(OffchainWorkerEnabled::Never, _) => false,
			(OffchainWorkerEnabled::WhenValidating, _) => false,
		};

        let allow_runtime_write_to_ocw_db = if enabled {
				self.allow_runtime_write_to_offchain_worker_db
			} else {
				false
			};

        config.offchain_worker = OffchainWorkerConfig { enabled, allow_runtime_write_to_ocw_db};

        Ok(())
	}
}