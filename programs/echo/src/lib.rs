use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("GaKTW6ycDT2yeAbN7siKdJTw9Xz777DpR5uXMMuw4CQZ");

#[program]
pub mod echo {
    use super::*;

    pub fn submit_review(
        ctx: Context<SubmitReview>,
        user_id: String,
        video_id: String,
        rating: u8,
        jwt: String,
    ) -> Results<()> {
        // Validate the JWT token
        if !validate_jwt(&jwt) {
            return Err(ProgramError::InvalidInstructionData.into());
        }

        // Store the review data
        ctx.accounts.review.user_id = user_id;
        ctx.accounts.review.video_id = video_id;
        ctx.accounts.review.rating = rating;

        // Calculate the token reward based on the rating
        let token_reward = calculate_token_reward(rating);

        // Transfer tokens to the user's token account
        let cpi_accounts = token::Transfer {
            from: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.token_mint.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, token_reward)?;

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct Review {
    pub user_id: String,
    pub video_id: String,
    pub rating: u8,
}

#[derive(Accounts)]
pub struct SubmitReview<'info> {
    #[account(init, payer = user, space = 8 + Review::SIZE)]
    pub review: Account<'info, Review>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

fn validate_jwt(jwt: &str) -> bool {
    // Implement JWT validation logic here
    // Return true if the JWT is valid, false otherwise
    // You can use a JWT library or implement your own validation
    true
}

fn calculate_token_reward(rating: u8) -> u64 {
    // Implement token reward calculation logic based on the rating
    // Return the amount of tokens to be rewarded
    match rating {
        1 => 10,
        2 => 10,
        3 => 10,
        4 => 10,
        5 => 10,
        _ => 0,
    }
}