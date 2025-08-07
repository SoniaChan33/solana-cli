use std::str::FromStr;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;
use solana_sdk::signer::keypair::read_keypair_file;
use solana_sdk::sysvar;
use solana_sdk::transaction::Transaction;
fn main() {}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum TokenInstruction {
    CreateToken { decimals: u8 },
    Mint { amount: u64 },
}

// 调用合约地址：5DPHC1PjRftRHJLKD4WSSXt83h1AChJy2pHXUqGdKD9X
#[test]
fn test_fn() {
    let rpc_client = RpcClient::new("http://127.0.0.1:8899".to_string());
    let payer = read_keypair_file("/Users/tinachan/.config/solana/id.json")
        .expect("Failed to read keypair file");
    let program_id = Pubkey::from_str("5DPHC1PjRftRHJLKD4WSSXt83h1AChJy2pHXUqGdKD9X").unwrap();

    let mint_account = Keypair::new();
    println!("Mint Account: {}", mint_account.pubkey().to_string());

    create_token(
        &rpc_client,
        &program_id,
        &payer,
        &mint_account,
        &payer.pubkey(),
        6,
    );
}

fn create_token(
    rpc_client: &RpcClient,
    program_id: &Pubkey,
    payer: &Keypair,
    mint_account: &Keypair,
    mint_authority: &Pubkey,
    decimals: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    // 这里可以添加创建代币的逻辑
    // 例如，调用合约的 create_token
    let instruction_data: Vec<u8> =
        borsh::to_vec(&TokenInstruction::CreateToken { decimals }).unwrap();

    // let account_iter = &mut accounts.iter();
    //         let mint_account = next_account_info(account_iter)?;
    //         let mint_authority = next_account_info(account_iter)?;
    //         let payer = next_account_info(account_iter)?;
    //         let rent_sysvar = next_account_info(account_iter)?;
    //         let system_program = next_account_info(account_iter)?;
    //         let token_program = next_account_info(account_iter)?;

    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(mint_account.pubkey(), true),
        AccountMeta::new_readonly(*mint_authority, false),
        AccountMeta::new_readonly(payer.pubkey(), true),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
    ];

    let token_instruction = Instruction {
        program_id: *program_id,
        accounts,
        data: instruction_data,
    };

    let latest_blockhash = rpc_client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[token_instruction],
        Some(&payer.pubkey()),
        &[payer, mint_account],
        latest_blockhash,
    );
    let r = rpc_client.send_and_confirm_transaction(&tx)?;

    println!("Transaction result: {:?}", r);

    println!(
        "Token created successfully with mint account: {}",
        mint_account.pubkey()
    );
    Ok(())
}
