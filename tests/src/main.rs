use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
use rand::seq::SliceRandom;
use rand::Rng;
use std::str::FromStr;
use subxt::{OnlineClient, SubstrateConfig};
use subxt_signer::sr25519::dev::{self};
use subxt_signer::sr25519::Keypair;
use subxt_signer::SecretUri;

/// Stress test module for evaluating the performance of a growing network under heavy transaction loads.
///
/// This module simulates a growing network with one new account added per block and assesses its
/// capability to handle a large number of transactions. The primary limiting factor for the number
/// of transactions is the number of unique "Nonce" values that can be produced, which corresponds
/// to the number of accounts present in the network.

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod science_vault {}

#[tokio::main]
pub async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
    }
}

pub async fn reward(
    api: &OnlineClient<SubstrateConfig>,
    origin: &Keypair,
    beneficiary: &Keypair,
) -> Result<(), Box<dyn std::error::Error>> {
    let amount: u128 = rand::thread_rng().gen();
    let tx = science_vault::tx()
        .reward()
        .reward(beneficiary.public_key().into(), amount);
    let _ = api
        .tx()
        .sign_and_submit_default(&tx, origin)
        .await
        .map(|e| {
            println!("Element rewarded");
            e
        })?;
    Ok(())
}

pub async fn punish(
    api: &OnlineClient<SubstrateConfig>,
    origin: &Keypair,
    beneficiary: &Keypair,
) -> Result<(), Box<dyn std::error::Error>> {
    let amount: u128 = rand::thread_rng().gen();
    let tx = science_vault::tx()
        .reward()
        .punish(beneficiary.public_key().into(), amount);
    let _ = api
        .tx()
        .sign_and_submit_default(&tx, origin)
        .await
        .map(|e| {
            println!("Element punished");
            e
        })?;
    Ok(())
}

pub async fn put_in_vault(
    api: &OnlineClient<SubstrateConfig>,
    origin: &Keypair,
    element: sp_core::H256,
) -> Result<(), Box<dyn std::error::Error>> {
    let tx = science_vault::tx().vault().add_element(element);
    let _ = api
        .tx()
        .sign_and_submit_default(&tx, origin)
        .await
        .map(|e| {
            println!("Element added to vault");
            e
        })?;
    Ok(())
}

pub async fn create_account(
    api: &OnlineClient<SubstrateConfig>,
    signers: &mut Vec<Keypair>,
) -> Result<(), Box<dyn std::error::Error>> {
    let pair: Keypair = Keypair::from_uri(
        &SecretUri::from_str(
            &("//".to_owned()
                + Alphanumeric
                    .sample_string(&mut rand::thread_rng(), 32)
                    .as_str()),
        )
        .unwrap(),
    )
    .unwrap();

    let tx = science_vault::tx().sudo().sudo(
        crate::science_vault::runtime_types::science_vault_runtime::RuntimeCall::Balances(
            crate::science_vault::balances::Call::force_set_balance {
                who: pair.public_key().into(),
                new_free: 100 * 10_000_000_000_000,
            },
        ),
    );
    let _ = api
        .tx()
        .sign_and_submit_default(&tx, &dev::alice())
        .await
        .map(|e| {
            println!("New account created");
            e
        })?;
    let _ = put_in_vault(api, &pair, sp_core::H256::random()).await; // Take one block
    signers.push(pair);
    Ok(())
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<SubstrateConfig>::from_url("ws://127.0.0.1:9944").await?;
    println!("Connection with node established.");

    let mut signers = vec![
        dev::alice(),
        dev::bob(),
        dev::eve(),
        dev::dave(),
        dev::ferdie(),
        dev::charlie(),
    ];

    for i in &signers {
        let _ = put_in_vault(&api, i, sp_core::H256::random()).await;
    }

    loop {
        for _ in 0..100 {
            let submitter = signers.choose(&mut rand::thread_rng()).unwrap();
            let _ = put_in_vault(&api, submitter, sp_core::H256::random()).await;
        }
        for _ in 0..100 {
            let submitter = signers.choose(&mut rand::thread_rng()).unwrap();
            let rewarder = signers.choose(&mut rand::thread_rng()).unwrap();
            let _ = reward(&api, rewarder, submitter).await;
        }
        for _ in 0..100 {
            let submitter = signers.choose(&mut rand::thread_rng()).unwrap();
            let punisher = signers.choose(&mut rand::thread_rng()).unwrap();
            let _ = punish(&api, punisher, submitter).await;
        }
        let _ = create_account(&api, &mut signers).await;
        println!("There are {} accounts", signers.len());
    }
}
