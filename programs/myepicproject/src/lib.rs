use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialise total gifs
    base_account.total_gifs = 0;
    Ok(())
  }

  // Function now accepts a gif link param from the user
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        // Get a reference to the account & increment total_gifs
        let base_account = &mut ctx.accounts.base_account;

        // Build the struct
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *base_account.to_account_info().key,
        };

        // Add it to the gif list vector
        base_account.gif_list.push(item);
        base_account.total_gifs +=1;
        Ok(())
    }
}



// Attach certain variables to the StartStuffOff context
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Specify what data you want in the AddGif Context
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

// Create a custom struct for us to work with
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

// Tell Solana what we want to store on this account
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a vector of type ItemStruct to the account
    pub gif_list: Vec<ItemStruct>,
}