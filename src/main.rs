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
use spl_associated_token_account::get_associated_token_address;
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

    _ = create_token(
        &rpc_client,
        &program_id,
        &payer,
        &mint_account,
        &payer.pubkey(),
        6,
    );

    _ = mint(
        &rpc_client,
        &program_id,
        &payer,
        &mint_account,
        &payer.pubkey(),
        100_000_000,
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

/**
 * 铸币
 */
fn mint(
    rpc_client: &RpcClient,
    program_id: &Pubkey,
    payer: &Keypair,
    mint_account: &Keypair,
    mint_authority: &Pubkey,
    amount: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    // ata 账户
    let ata_account = get_associated_token_address(&payer.pubkey(), &mint_account.pubkey());
    println!("ATA Account: {}", ata_account.to_string());
    // 这里可以添加铸币的逻辑
    // 例如，调用合约的 mint
    let instruction_data: Vec<u8> = borsh::to_vec(&TokenInstruction::Mint { amount }).unwrap();

    let accounts: Vec<AccountMeta> = vec![
        AccountMeta::new(mint_account.pubkey(), true), // Mint账户
        AccountMeta::new(ata_account, false),          // ATA账户 (添加)
        AccountMeta::new_readonly(sysvar::rent::id(), false), // 租金系统变量 (添加)
        AccountMeta::new_readonly(payer.pubkey(), true), // 支付账户
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false), // 系统程序
        AccountMeta::new_readonly(spl_token::id(), false), // Token程序
        AccountMeta::new_readonly(spl_associated_token_account::id(), false), // ATA程序
    ];

    // 创建指令
    let token_instruction = Instruction {
        program_id: *program_id,
        accounts,
        data: instruction_data,
    };

    // 发送交易
    // 获取最新区块哈希
    let latest_blockhash = rpc_client.get_latest_blockhash()?;
    // 创建交易
    let tx = Transaction::new_signed_with_payer(
        &[token_instruction],
        Some(&payer.pubkey()),
        &[payer, mint_account],
        latest_blockhash,
    );
    let r = rpc_client.send_and_confirm_transaction(&tx)?;

    println!("Transaction result: {:?}", r);

    println!(
        "Token minted successfully to account: {}",
        mint_account.pubkey()
    );
    Ok(())
}

#[test]
fn check_token_balance() {
    let rpc_client = RpcClient::new("http://127.0.0.1:8899".to_string());

    // 使用测试中生成的地址
    let mint_account = Pubkey::from_str("BKSxNFXEkT99cc3ah8ALguSck2GpCz1j3vqkMoE9BJ7P").unwrap();
    let ata_account = Pubkey::from_str("32cVBcs5GGAv2ad92gDxv8j9dqnqdth9665X5Qp2tmof").unwrap();

    // 查看Mint账户信息
    match rpc_client.get_account(&mint_account) {
        Ok(account) => {
            println!("Mint账户信息:");
            println!("  Owner: {}", account.owner);
            println!("  Lamports: {}", account.lamports);
            println!("  Data length: {}", account.data.len());
        }
        Err(e) => println!("获取Mint账户信息失败: {}", e),
    }

    // 查看ATA账户信息
    match rpc_client.get_account(&ata_account) {
        Ok(account) => {
            println!("\nATA账户信息:");
            println!("  Owner: {}", account.owner);
            println!("  Lamports: {}", account.lamports);
            println!("  Data length: {}", account.data.len());

            // 如果账户存在且有数据，说明Token账户已创建
            if account.data.len() > 0 {
                println!("  ✅ Token账户已创建");
            }
        }
        Err(e) => println!("获取ATA账户信息失败: {}", e),
    }

    // 尝试获取Token余额
    match rpc_client.get_token_account_balance(&ata_account) {
        Ok(balance) => {
            println!("\n🎉 Token余额信息:");
            println!("  数量: {}", balance.amount);
            println!("  小数位数: {}", balance.decimals);
            println!("  UI数量: {}", balance.ui_amount_string);
        }
        Err(e) => println!("获取Token余额失败: {}", e),
    }

    // 获取Mint供应量信息
    match rpc_client.get_token_supply(&mint_account) {
        Ok(supply) => {
            println!("\n📊 Token供应量信息:");
            println!("  总供应量: {}", supply.amount);
            println!("  小数位数: {}", supply.decimals);
            println!("  UI供应量: {}", supply.ui_amount_string);
        }
        Err(e) => println!("获取Token供应量失败: {}", e),
    }
}
