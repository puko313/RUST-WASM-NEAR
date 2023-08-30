use near_units::parse_near;
use serde_json::json;
use workspaces::prelude::*;
use workspaces::{network::Sandbox, sandbox, Account, Contract, Worker};

const WASM_FILEPATH: &str = "../../out/main.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let sandbox = sandbox().await?;
  let wasm = std::fs::read(WASM_FILEPATH)?;
  let contract = sandbox.dev_deploy(&wasm).await?;

  // create accounts
  let owner = sandbox.root_account();
  let user = owner
    .create_subaccount(&sandbox, "user")
    .initial_balance(parse_near!("30 N"))
    .transact()
    .await?
    .into_result()?;

  test_increment(&user, &contract, &sandbox).await?;
  test_decrement(&user, &contract, &sandbox).await?;
  test_reset(&user, &contract, &sandbox).await?;

  Ok(())
}

async fn test_increment(
  user: &Account,
  contract: &Contract,
  sandbox: &Worker<Sandbox>,
) -> anyhow::Result<()> {
  let start_counter: u64 = user
    .call(&sandbox, contract.id(), "get_num")
    .args_json(json!({}))?
    .transact()
    .await?
    .json()?;

  user
    .call(&sandbox, contract.id(), "increment")
    .args_json(json!({}))?
    .transact()
    .await?;

  let end_counter: u64 = user
    .call(&sandbox, contract.id(), "get_num")
    .args_json(json!({}))?
    .transact()
    .await?
    .json()?;

  assert_eq!(end_counter, start_counter + 1);
  println!("Increment ✅");
  Ok(())
}

async fn test_decrement(
  user: &Account,
  contract: &Contract,
  sandbox: &Worker<Sandbox>,
) -> anyhow::Result<()> {
  user
    .call(&sandbox, contract.id(), "increment")
    .args_json(json!({}))?
    .transact()
    .await?;

  let start_counter: u64 = user
    .call(&sandbox, contract.id(), "get_num")
    .args_json(json!({}))?
    .transact()
    .await?
    .json()?;

  user
    .call(&sandbox, contract.id(), "decrement")
    .args_json(json!({}))?
    .transact()
    .await?;

  let end_counter: u64 = user
    .call(&sandbox, contract.id(), "get_num")
    .args_json(json!({}))?
    .transact()
    .await?
    .json()?;

  assert_eq!(end_counter, start_counter - 1);
  println!("Decrement ✅");
  Ok(())
}

async fn test_reset(
  user: &Account,
  contract: &Contract,
  sandbox: &Worker<Sandbox>,
) -> anyhow::Result<()> {
  user
    .call(&sandbox, contract.id(), "increment")
    .args_json(json!({}))?
    .transact()
    .await?;

  user
    .call(&sandbox, contract.id(), "increment")
    .args_json(json!({}))?
    .transact()
    .await?;

  user
    .call(&sandbox, contract.id(), "reset")
    .args_json(json!({}))?
    .transact()
    .await?;

  let end_counter: u64 = user
    .call(&sandbox, contract.id(), "get_num")
    .args_json(json!({}))?
    .transact()
    .await?
    .json()?;

  assert_eq!(end_counter, 0);
  println!("Reset ✅");
  Ok(())
}
