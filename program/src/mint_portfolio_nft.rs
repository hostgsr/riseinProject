use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};


use crate::generated::state::{
	Account,
	AccountPDA,
	PortfolioMetadata,
};


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
/// - profession: [String] 
/// - skills: [String] 
/// - json_info: [String] 
pub fn mint_portfolio_nft(
	program_id: &Pubkey,
    for_initialize_mint_2: &[&AccountInfo],
    for_create: &[&AccountInfo],
    for_mint_to: &[&AccountInfo],
    for_set_authority: &[&AccountInfo],
    mint: &Account<spl_token::state::Mint>,
    portfolio: &mut AccountPDA<PortfolioMetadata>,
    funding: &AccountInfo,
    assoc_token_account: &AccountInfo,
    wallet: &AccountInfo,
    owner: &AccountInfo,
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
    // Implement your business logic here...

     // Populate the PortfolioMetadata fields
     portfolio.data.name = name.clone();;
     portfolio.data.symbol = symbol;
     portfolio.data.description = description.clone();;
     portfolio.data.seller_fee_basis_points = seller_fee_basis_points;
     portfolio.data.image = image;
     portfolio.data.collection = collection;
     portfolio.data.properties = properties;
     portfolio.data.profession = profession.clone();;
     portfolio.data.skills = skills.clone();;
     portfolio.data.json_info = json_info;
     portfolio.data.mint = *mint.info.key;
     portfolio.data.assoc_account = Some(*assoc_token_account.key);
 




     // Log messages for debugging
     msg!("Name: {}", name);
     msg!("Profession: {}", profession);
     msg!("Skills: {}", skills);
     msg!("Description: {}", description);

     // Existing token operations
     csl_spl_token::src::cpi::initialize_mint_2(for_initialize_mint_2, 0, *wallet.key, None)?;
     csl_spl_assoc_token::src::cpi::create(for_create)?;
     csl_spl_token::src::cpi::mint_to(for_mint_to, 1)?;
     csl_spl_token::src::cpi::set_authority(for_set_authority, 0, None)?;
 
     Ok(())
}