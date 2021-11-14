use anchor_lang::prelude::*;

declare_id!("9sLG4qKC4XPjLcQ56kiJk6a2CrHnYaLYKUHBDuRf2HFz");

#[program]
pub mod solana_app {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;

    base_account.total_images = 0;
    Ok(())
  }

  pub fn add_image(ctx: Context<AddImage>, image_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    // Build the struct.
      let item = ItemStruct {
        image_link: image_link.to_string(),
        user_address: *user.to_account_info().key,
      };
      
      base_account.image_list.push(item);
      base_account.total_images += 1;
      Ok(())
  }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddImage<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub image_link: String,
    pub user_address: Pubkey,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_images: u64,
    pub image_list: Vec<ItemStruct>,
}