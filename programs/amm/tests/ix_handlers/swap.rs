use {
    anchor_lang::{
        prelude::*, solana_program::instruction::Instruction, InstructionData, ToAccountMetas,
    },
    anchor_spl::associated_token::{self, ID as ASSOCIATED_TOKEN_PROGRAM_ID},
    litesvm::LiteSVM,
    litesvm_token::spl_token::ID as TOKEN_PROGRAM_ID,
    solana_keypair::Keypair,
    // solana_pubkey::pubkey,
    solana_signer::Signer,
};
pub fn create_swap_ix(
    mut _svm: &mut LiteSVM,
    payer: &Keypair,
    mint_x: Pubkey,
    mint_y: Pubkey,
    config: Pubkey,
    mint_lp: Pubkey,
    vault_x: Pubkey,
    vault_y: Pubkey,
) -> Instruction {
    let user = payer.pubkey();
    let user_x = associated_token::get_associated_token_address(&user, &mint_x);
    let user_y = associated_token::get_associated_token_address(&user, &mint_y);

    Instruction::new_with_bytes(
        amm::id(),
        &amm::instruction::Swap {
            is_x: true,
            amount_in: 10_000_000,
            min_amount_out: 5_000_000,
        }
        .data(),
        amm::accounts::Swap {
            user,
            user_x,
            user_y,
            mint_x,
            mint_y,
            mint_lp,
            vault_x,
            vault_y,
            config,
            token_program: TOKEN_PROGRAM_ID,
            associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None),
    )
}
