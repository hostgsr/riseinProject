// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use std::str::FromStr;
use borsh::BorshSerialize;
use solana_program::account_info::{AccountInfo, next_account_info, next_account_infos};
use solana_program::borsh0_10::try_from_slice_unchecked;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program::{msg, system_program};
use solana_program::sysvar::Sysvar;
use solana_program::program_pack::Pack;
use crate::generated::errors::PortfolioNftError;
use crate::generated::instructions::PortfolioNftInstruction;

use crate::generated::state::{
	Account,
	AccountPDA,
	PortfolioMetadata,
};
use crate::src::*;

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        let instruction = PortfolioNftInstruction::unpack(data)?;

        match instruction {
			PortfolioNftInstruction::MintPortfolioNft(args) => {
				msg!("Instruction: MintPortfolioNft");
				Self::process_mint_portfolio_nft(
					program_id,
					accounts, 
					args.name,
					args.symbol,
					args.description,
					args.seller_fee_basis_points,
					args.image,
					args.collection,
					args.properties,
					args.profession,
					args.skills,
					args.json_info,
				)
			}
			PortfolioNftInstruction::TransferPortfolioNft => {
				msg!("Instruction: TransferPortfolioNft");
				Self::process_transfer_portfolio_nft(program_id, accounts)
			}
			PortfolioNftInstruction::BurnPortfolioNft => {
				msg!("Instruction: BurnPortfolioNft");
				Self::process_burn_portfolio_nft(program_id, accounts)
			}
        }
    }

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable, signer]` mint: [Mint] 
/// 2. `[writable]` portfolio: [PortfolioMetadata] 
/// 3. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
/// 4. `[writable, signer]` funding: [AccountInfo] Funding account (must be a system account)
/// 5. `[writable]` assoc_token_account: [AccountInfo] Associated token account address to be created
/// 6. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 7. `[]` token_program: [AccountInfo] SPL Token program
/// 8. `[signer]` owner: [AccountInfo] The mint's minting authority.
/// 9. `[]` csl_spl_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
/// 10. `[]` csl_spl_assoc_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplAssocTokenProgram v0.0.0
///
/// Data:
/// - name: [String] 
/// - symbol: [String] 
/// - description: [String] type
/// - seller_fee_basis_points: [u64] 
/// - image: [String] 
/// - collection: [String] 
/// - properties: [String] 
/// - profession: [String] 
/// - skills: [String] 
/// - json_info: [String] 
	pub fn process_mint_portfolio_nft(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		name: String,
		symbol: String,
		description: String,
		seller_fee_basis_points: u64,
		image: String,
		collection: String,
		properties: String,
		profession: String,
		skills: String,
		json_info: String,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let mint_info = next_account_info(account_info_iter)?;
		let portfolio_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;
		let funding_info = next_account_info(account_info_iter)?;
		let assoc_token_account_info = next_account_info(account_info_iter)?;
		let wallet_info = next_account_info(account_info_iter)?;
		let token_program_info = next_account_info(account_info_iter)?;
		let owner_info = next_account_info(account_info_iter)?;
		let csl_spl_token_v_0_0_0_info = next_account_info(account_info_iter)?;
		let csl_spl_assoc_token_v_0_0_0_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (portfolio_pubkey, portfolio_bump) = Pubkey::find_program_address(
			&[b"portfolio", mint_info.key.as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(PortfolioNftError::InvalidSignerPermission.into());
		}

		if mint_info.is_signer != true {
			return Err(PortfolioNftError::InvalidSignerPermission.into());
		}

		if *portfolio_info.key != portfolio_pubkey {
			return Err(PortfolioNftError::NotExpectedAddress.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(PortfolioNftError::NotExpectedAddress.into());
		}

		if funding_info.is_signer != true {
			return Err(PortfolioNftError::InvalidSignerPermission.into());
		}

		if *token_program_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(PortfolioNftError::NotExpectedAddress.into());
		}

		if owner_info.is_signer != true {
			return Err(PortfolioNftError::InvalidSignerPermission.into());
		}

		if *csl_spl_token_v_0_0_0_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(PortfolioNftError::NotExpectedAddress.into());
		}

		if *csl_spl_assoc_token_v_0_0_0_info.key != Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap() {
			return Err(PortfolioNftError::NotExpectedAddress.into());
		}


		// Accounts Initializations
		let space = spl_token::state::Mint::LEN;
		let rent = Rent::get()?;
		let rent_minimum_balance = rent.minimum_balance(space);

		invoke(
			&create_account(
				&fee_payer_info.key,
				&mint_info.key,
				rent_minimum_balance,
				space as u64,
				&Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
			),
			&[fee_payer_info.clone(), mint_info.clone()],
		)?;

		let space = PortfolioMetadata::LEN;
		let rent = Rent::get()?;
		let rent_minimum_balance = rent.minimum_balance(space);

		invoke_signed(
			&create_account(
				&fee_payer_info.key,
				&portfolio_info.key,
				rent_minimum_balance,
				space as u64,
				program_id,
			),
			&[fee_payer_info.clone(), portfolio_info.clone()],
			&[&[b"portfolio", mint_info.key.as_ref(), &[portfolio_bump]]],
		)?;


		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(PortfolioNftError::WrongAccountOwner.into());
		}

		if *mint_info.owner != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(PortfolioNftError::WrongAccountOwner.into());
		}

		if *funding_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(PortfolioNftError::WrongAccountOwner.into());
		}

		if *wallet_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(PortfolioNftError::WrongAccountOwner.into());
		}

		if mint_info.data_len() != spl_token::state::Mint::LEN {
			return Err(PortfolioNftError::InvalidAccountLen.into());
		}

		if portfolio_info.data_len() != PortfolioMetadata::LEN {
			return Err(PortfolioNftError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let mint = Account::new(
			&mint_info,
			spl_token::state::Mint::unpack_from_slice(&mint_info.data.borrow()).unwrap(),
		);
		let portfolio = &mut AccountPDA::new(
			&portfolio_info,
			try_from_slice_unchecked::<PortfolioMetadata>(&portfolio_info.data.borrow()).unwrap(),
			portfolio_bump,
		);

		// Calling STUB
		mint_portfolio_nft::mint_portfolio_nft(
			program_id,
			&vec![
				mint_info,
			],
			&vec![
				funding_info,
				assoc_token_account_info,
				wallet_info,
				mint_info,
				system_program_info,
				token_program_info,
			],
			&vec![
				mint_info,
				assoc_token_account_info,
				owner_info,
				wallet_info,
				token_program_info,
			],
			&vec![
				mint_info,
				owner_info,
			],
			&mint,
			portfolio,
			funding_info,
			assoc_token_account_info,
			wallet_info,
			owner_info,
			name,
			symbol,
			description,
			seller_fee_basis_points,
			image,
			collection,
			properties,
			profession,
			skills,
			json_info,
		)?;

		// Accounts Serialization
		portfolio.data.serialize(&mut &mut portfolio_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[]` mint: [Mint] 
/// 2. `[writable]` portfolio: [PortfolioMetadata] 
/// 3. `[writable, signer]` funding: [AccountInfo] Funding account (must be a system account)
/// 4. `[writable]` assoc_token_account: [AccountInfo] Associated token account address to be created
/// 5. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 6. `[]` system_program: [AccountInfo] System program
/// 7. `[]` token_program: [AccountInfo] SPL Token program
/// 8. `[writable]` source: [AccountInfo] The source account.
/// 9. `[writable]` destination: [AccountInfo] The destination account.
/// 10. `[signer]` authority: [AccountInfo] The source account's owner/delegate.
/// 11. `[]` csl_spl_assoc_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplAssocTokenProgram v0.0.0
/// 12. `[]` csl_spl_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
	pub fn process_transfer_portfolio_nft(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let mint_info = next_account_info(account_info_iter)?;
		let portfolio_info = next_account_info(account_info_iter)?;
		let funding_info = next_account_info(account_info_iter)?;
		let assoc_token_account_info = next_account_info(account_info_iter)?;
		let wallet_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;
		let token_program_info = next_account_info(account_info_iter)?;
		let source_info = next_account_info(account_info_iter)?;
		let destination_info = next_account_info(account_info_iter)?;
		let authority_info = next_account_info(account_info_iter)?;
		let csl_spl_assoc_token_v_0_0_0_info = next_account_info(account_info_iter)?;
		let csl_spl_token_v_0_0_0_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (portfolio_pubkey, portfolio_bump) = Pubkey::find_program_address(
			&[b"portfolio", mint_info.key.as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(PortfolioNftError::InvalidSignerPermission.into());
		}

		if *portfolio_info.key != portfolio_pubkey {
			return Err(PortfolioNftError::NotExpectedAddress.into());
		}

		if funding_info.is_signer != true {
			return Err(PortfolioNftError::InvalidSignerPermission.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(PortfolioNftError::NotExpectedAddress.into());
		}

		if *token_program_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(PortfolioNftError::NotExpectedAddress.into());
		}

		if authority_info.is_signer != true {
			return Err(PortfolioNftError::InvalidSignerPermission.into());
		}

		if *csl_spl_assoc_token_v_0_0_0_info.key != Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap() {
			return Err(PortfolioNftError::NotExpectedAddress.into());
		}

		if *csl_spl_token_v_0_0_0_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(PortfolioNftError::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(PortfolioNftError::WrongAccountOwner.into());
		}

		if *mint_info.owner != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(PortfolioNftError::WrongAccountOwner.into());
		}

		if *funding_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(PortfolioNftError::WrongAccountOwner.into());
		}

		if *wallet_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(PortfolioNftError::WrongAccountOwner.into());
		}

		if mint_info.data_len() != spl_token::state::Mint::LEN {
			return Err(PortfolioNftError::InvalidAccountLen.into());
		}

		if portfolio_info.data_len() != PortfolioMetadata::LEN {
			return Err(PortfolioNftError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let mint = Account::new(
			&mint_info,
			spl_token::state::Mint::unpack_from_slice(&mint_info.data.borrow()).unwrap(),
		);
		let portfolio = &mut AccountPDA::new(
			&portfolio_info,
			try_from_slice_unchecked::<PortfolioMetadata>(&portfolio_info.data.borrow()).unwrap(),
			portfolio_bump,
		);

		// Calling STUB
		transfer_portfolio_nft::transfer_portfolio_nft(
			program_id,
			&vec![
				funding_info,
				assoc_token_account_info,
				wallet_info,
				mint_info,
				system_program_info,
				token_program_info,
			],
			&vec![
				source_info,
				mint_info,
				destination_info,
				authority_info,
			],
			&mint,
			portfolio,
			funding_info,
			assoc_token_account_info,
			wallet_info,
			source_info,
			destination_info,
			authority_info,
		)?;

		// Accounts Serialization
		portfolio.data.serialize(&mut &mut portfolio_info.data.borrow_mut()[..])?;		
		Ok(())
	}

/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] Auto-generated, default fee payer
/// 1. `[writable]` mint: [Mint] 
/// 2. `[writable]` portfolio: [PortfolioMetadata] 
/// 3. `[writable]` account: [Account] The account to burn from.
/// 4. `[signer]` owner: [AccountInfo] The account's owner/delegate.
/// 5. `[]` wallet: [AccountInfo] Wallet address for the new associated token account
/// 6. `[]` token_program: [AccountInfo] SPL Token program
/// 7. `[]` csl_spl_token_v_0_0_0: [AccountInfo] Auto-generated, CslSplTokenProgram v0.0.0
	pub fn process_burn_portfolio_nft(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let mint_info = next_account_info(account_info_iter)?;
		let portfolio_info = next_account_info(account_info_iter)?;
		let account_info = next_account_info(account_info_iter)?;
		let owner_info = next_account_info(account_info_iter)?;
		let wallet_info = next_account_info(account_info_iter)?;
		let token_program_info = next_account_info(account_info_iter)?;
		let csl_spl_token_v_0_0_0_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (portfolio_pubkey, portfolio_bump) = Pubkey::find_program_address(
			&[b"portfolio", mint_info.key.as_ref()],
			program_id,
		);
		let (account_pubkey, account_bump) = Pubkey::find_program_address(
			&[wallet_info.key.as_ref(), token_program_info.key.as_ref(), mint_info.key.as_ref()],
			&Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(PortfolioNftError::InvalidSignerPermission.into());
		}

		if *portfolio_info.key != portfolio_pubkey {
			return Err(PortfolioNftError::NotExpectedAddress.into());
		}

		if *account_info.key != account_pubkey {
			return Err(PortfolioNftError::NotExpectedAddress.into());
		}

		if owner_info.is_signer != true {
			return Err(PortfolioNftError::InvalidSignerPermission.into());
		}

		if *token_program_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(PortfolioNftError::NotExpectedAddress.into());
		}

		if *csl_spl_token_v_0_0_0_info.key != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(PortfolioNftError::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(PortfolioNftError::WrongAccountOwner.into());
		}

		if *mint_info.owner != Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap() {
			return Err(PortfolioNftError::WrongAccountOwner.into());
		}

		if *wallet_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(PortfolioNftError::WrongAccountOwner.into());
		}

		if mint_info.data_len() != spl_token::state::Mint::LEN {
			return Err(PortfolioNftError::InvalidAccountLen.into());
		}

		if portfolio_info.data_len() != PortfolioMetadata::LEN {
			return Err(PortfolioNftError::InvalidAccountLen.into());
		}

		if account_info.data_len() != spl_token::state::Account::LEN {
			return Err(PortfolioNftError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let mint = Account::new(
			&mint_info,
			spl_token::state::Mint::unpack_from_slice(&mint_info.data.borrow()).unwrap(),
		);
		let portfolio = &mut AccountPDA::new(
			&portfolio_info,
			try_from_slice_unchecked::<PortfolioMetadata>(&portfolio_info.data.borrow()).unwrap(),
			portfolio_bump,
		);
		let account = AccountPDA::new(
			&account_info,
			spl_token::state::Account::unpack_from_slice(&account_info.data.borrow()).unwrap(),
			account_bump,
		);

		// Calling STUB
		burn_portfolio_nft::burn_portfolio_nft(
			program_id,
			&vec![
				account_info,
				mint_info,
				owner_info,
				wallet_info,
				token_program_info,
			],
			&mint,
			portfolio,
			&account,
			owner_info,
			wallet_info,
		)?;

		// Accounts Serialization
		portfolio.data.serialize(&mut &mut portfolio_info.data.borrow_mut()[..])?;		
		Ok(())
	}
}