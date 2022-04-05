#[cfg(test)]
mod tests {
    use {
        solana_client_helpers::{Client, RpcClient},
        solana_sdk::{
            commitment_config::CommitmentConfig, instruction::Instruction,
            native_token::LAMPORTS_PER_SOL, signature::Keypair, transaction::Transaction,
        },
        std::sync::Arc,
    };

    fn new_client() -> Arc<Client> {
        let url = "http://localhost:8899";
        let client = Arc::new(Client {
            client: RpcClient::new_with_commitment(url, CommitmentConfig::processed()),
            payer: Keypair::new(),
        });
        client
            .airdrop(&client.payer_pubkey(), LAMPORTS_PER_SOL)
            .unwrap();
        client
    }

    fn sign_and_submit(client: &Arc<Client>, ixs: &[Instruction]) {
        let mut tx = Transaction::new_with_payer(ixs, Some(&client.payer_pubkey()));
        tx.sign(&vec![&client.payer], client.latest_blockhash().unwrap());
        let sig = client.send_and_confirm_transaction(&tx).unwrap();
        println!("Signature: {}", sig);
    }

    #[test]
    #[ignore]
    fn initialize() {
        let client = new_client();

        let authority_pda = cronos_sdk::account::Authority::pda();
        let config_pda = cronos_sdk::account::Config::pda();
        let daemon_pda = cronos_sdk::account::Daemon::pda(authority_pda.0);
        let fee_pda = cronos_sdk::account::Fee::pda(daemon_pda.0);
        let health_pda = cronos_sdk::account::Health::pda();
        let ix = cronos_sdk::instruction::admin_open(
            client.payer_pubkey(),
            authority_pda,
            config_pda,
            daemon_pda,
            fee_pda,
            health_pda,
        );

        sign_and_submit(&client, &[ix]);

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
