use std::sync::Arc;

use anchor_lang::prelude::Pubkey;
use cronos_sdk::account::Task;
use solana_clap_utils::keypair::DefaultSigner;
use solana_client_helpers::RpcClient;

use crate::{command::CliCommand, config::CliConfig, error::CliError};

pub fn process_command(
    command: &CliCommand,
    config: &CliConfig,
    _signer: &DefaultSigner,
) -> Result<(), CliError> {
    let client = Arc::new(RpcClient::new_with_timeouts_and_commitment(
        config.json_rpc_url.to_string(),
        config.rpc_timeout,
        config.commitment,
        config.confirm_transaction_initial_timeout,
    ));

    match command {
        &CliCommand::TaskData { address } => process_task_data(&client, &address),
        _ => Err(CliError::CommandNotImplemented(command.to_string())),
    }
}

fn process_task_data(client: &Arc<RpcClient>, address: &Pubkey) -> Result<(), CliError> {
    let data = client
        .get_account_data(address)
        .map_err(|_err| CliError::AccountNotFound(address.to_string()))?;
    let task_data = Task::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!(
        "daemon: {},
id: {},
status: {},
exec_at: {},
stop_at: {},
recurr: {},
ix: {:?},  ",
        task_data.daemon,
        task_data.id,
        task_data.status,
        task_data.exec_at,
        task_data.stop_at,
        task_data.recurr,
        task_data.ix,
    );
    Ok(())
}

// impl std::fmt::Display for Task {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "Task: {{
//                 daemon: {},
//                 id: {},
//                 status: {},
//                 exec_at: {},
//                 stop_at: {},
//                 recurr: {},
//                 ix: {},
//                 data: {},
//             }}",
//             self.daemon,
//             self.id,
//             self.status,
//             self.exec_at,
//             self.stop_at,
//             self.recurr,
//             self.ix,
//             self.data
//         )
//     }
// }
