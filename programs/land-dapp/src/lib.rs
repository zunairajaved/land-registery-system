use anchor_lang::prelude::*;

declare_id!("2HxwY5HULSiudKBeCfooPA3Q48hJYuy7H16mVU49crxK");

#[program]
pub mod land_dapp {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     Ok(())
    // }

    pub fn register_user(ctx:Context<RegisterUser>,name:String,profile:String) -> Result <()> {
        let user_account : &mut Account<User> = &mut ctx.accounts.user_account;
        let authority : &Signer = &mut ctx.accounts.authority;

        user_account.name = name;
        user_account.profile = profile;
        user_account.authority = authority.key();

        Ok(())
    }

    pub fn update_user(ctx:Context<UpdateUser>,name:String,profile:String)->Result<()>{
        let user_account : &mut Account<User> = &mut ctx.accounts.user_account;

        user_account.name = name;
        user_account.profile = profile;
        Ok(())
    }

    pub fn add_asset(ctx:Context<AddAsset>,address:String,location:String,area:u64,description:String)->Result<()>{
       
        let asset_account : &mut Account<Asset> = &mut ctx.accounts.asset_account;
        let user_account : &mut Account<User> = &mut ctx.accounts.user_account;
        let authority  : &Signer = &mut ctx.accounts.authority;


        asset_account.address = address;
        asset_account.location = location;
        asset_account.area = area;
        asset_account.description = description;
        asset_account.user = user_account.key();
        asset_account.authority = authority.key();

        // store created post id as current post id in blog account
       

        // emit!(PostEvent {
        //     label: "CREATE".to_string(),
        //     post_id: post_account.key(),
        //     next_post_id: None // same as null
        // });

        
    }
}

// #[derive(Accounts)]
// pub struct Initialize {}

#[derive(Accounts)]
pub struct RegisterUser <'info>{
    #[account(init, payer = authority, space = 8 + 40 + 120  + 32)]
    pub user_account: Account<'info, User>,
    #[account(mut)]
    pub authority : Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
  pub struct UpdateUser<'info> {
      #[account(mut,has_one = authority)]
      pub user_account: Account<'info, User>,
      #[account(mut)]
      pub authority: Signer<'info>,
  }

#[account]
pub struct User {
    pub name: String,
    pub profile: String,
    pub authority: Pubkey,
}

pub struct AddAsset <'info >{
    #[account(mut, has_one = authority)]
    pub user_account: Account<'info , User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority,space = 8+40+120+32)]
    pub asset_account : Account<'info , Asset>,
    pub system_program : Program<'info, System>
}

#[account]
pub struct Asset {
pub address:String,
pub location:String,
pub area: u64,
pub description:String,
pub user :Pubkey,
pub authority: Pubkey
}