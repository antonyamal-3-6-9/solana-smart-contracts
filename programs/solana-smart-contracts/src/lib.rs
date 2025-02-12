use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Transfer};
use mpl_token_metadata::instruction::{create_metadata_accounts_v2, create_master_edition_v3};
use solana_program::program::invoke;


declare_id!("6iPxdjHimmNrLemHX7RX6NwvEJiBAaChVLA9ivQtUy3b");

##[derive(Accounts)]
pub struct MintNftWithSwapCoin<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,  // NFT Mint Account

    #[account(mut)]
    pub payer_swapcoin_account: Account<'info, TokenAccount>, // Payer's SwapCoin Account

    #[account(mut)]
    pub payer_nft_account: Account<'info, TokenAccount>, // Payer's NFT Token Account (where NFT is received)

    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,  // Metadata Account

    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,  // Master Edition Account

    #[account(mut)]
    pub payer: Signer<'info>,  // NFT minter and recipient

    #[account(mut)]
    pub swapcoin_treasury: Account<'info, TokenAccount>,  // Treasury receiving SwapCoin

    pub token_program: Program<'info, Token>,  // SPL Token Program
    pub token_metadata_program: UncheckedAccount<'info>,  // Metaplex Metadata Program
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}



pub fn mint_nft_with_swapcoin(
    ctx: Context<MintNftWithSwapCoin>,
    metadata_title: String,
    metadata_symbol: String,
    metadata_uri: String,
    swapcoin_amount: u64,
) -> Result<()> {
    // 1️⃣ Transfer SwapCoin from payer to treasury
    let transfer_instruction = Transfer {
        from: ctx.accounts.payer_swapcoin_account.to_account_info(),
        to: ctx.accounts.swapcoin_treasury.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let transfer_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_instruction);
    token::transfer(transfer_context, swapcoin_amount)?;

    // 2️⃣ Mint the NFT to the payer's NFT account
    let mint_to_instruction = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.payer_nft_account.to_account_info(),  // Send NFT to payer
        authority: ctx.accounts.payer.to_account_info(),
    };
    let mint_to_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), mint_to_instruction);
    token::mint_to(mint_to_context, 1)?;  // Mint exactly 1 NFT

    // 3️⃣ Create NFT metadata
    let metadata_instruction = create_metadata_accounts_v2(
        ctx.accounts.token_metadata_program.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.payer.key(),
        metadata_title,
        metadata_symbol,
        metadata_uri,
        None,
        0,
        true,
        false,
        None,
        None,
    );

    let metadata_accounts = vec![
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];
    invoke(&metadata_instruction, &metadata_accounts)?;

    // 4️⃣ Create Master Edition (ensuring NFT uniqueness)
    let master_edition_instruction = create_master_edition_v3(
        ctx.accounts.token_metadata_program.key(),
        ctx.accounts.master_edition.key(),
        ctx.accounts.mint.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.payer.key(),
        ctx.accounts.metadata.key(),
        ctx.accounts.payer.key(),
        Some(1), // Max supply = 1
    );

    let master_edition_accounts = vec![
        ctx.accounts.master_edition.to_account_info(),
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.metadata.to_account_info(),
        ctx.accounts.token_metadata_program.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.rent.to_account_info(),
    ];
    invoke(&master_edition_instruction, &master_edition_accounts)?;

    Ok(())
}
