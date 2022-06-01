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

    pub fn add_asset(ctx:Context<AddAsset>,address:String,location:String,area:u64,description:String,asset_value:u64)->Result<()>{
       
        let asset_account : &mut Account<Asset> = &mut ctx.accounts.asset_account;
        let user_account : &mut Account<User> = &mut ctx.accounts.user_account;
        let authority  : &Signer = &mut ctx.accounts.authority;

        asset_account.address = address;
        asset_account.location = location;
        asset_account.area = area;
        asset_account.description = description;
        asset_account.asset_value = asset_value;
        asset_account.user = user_account.key();
        asset_account.authority = authority.key();
        Ok(())

        // store created post id as current post id in blog account
       

        // emit!(PostEvent {
        //     label: "CREATE".to_string(),
        //     post_id: post_account.key(),
        //     next_post_id: None // same as null
        // }); 
    }

    pub fn update_asset(ctx:Context<UpdateAsset>,description:String,area:u64,asset_value:u64) -> Result <()> {
        let asset_account : &mut Account<Asset> = &mut ctx.accounts.asset_account;

        asset_account.area=area;
        asset_account.description=description;
        asset_account.asset_value = asset_value;
        Ok(())
    }

    pub fn initialize_buy_contract(ctx:Context<InitializeBuyContract>,amount:u64)->Result <()> {
        let buy_contract : &mut Account<BuyContract> = &mut ctx.accounts.buy_contract;
        let asset_account : &mut Account<Asset> = &mut ctx.accounts.asset_account;
        let owner_account :&mut Account<User> = &mut ctx.accounts.owner_account;
        let authority  : &Signer = &mut ctx.accounts.authority;


        if amount == 0 {
            return Err(ErrorCode::IncorrectPaymentAmount.into())
		}

        buy_contract.owner_account = owner_account.key();
        buy_contract.buyer = ctx.accounts.buyer.key();
        buy_contract.asset_account = asset_account.key();
        buy_contract.authority = authority.key();
        Ok(())

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

#[derive(Accounts)]
pub struct AddAsset <'info >{
    #[account(mut, has_one = authority)]
    pub user_account: Account<'info , User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority,space = 8+40+120+32)]
    pub asset_account : Account<'info , Asset>,
    pub system_program : Program<'info, System>
}

#[derive(Accounts)]
pub struct UpdateAsset<'info>{
    #[account(mut, has_one=authority)]
    pub user_account: Account<'info , User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub asset_account:Account<'info ,Asset>
}

#[derive(Accounts)]
pub struct InitializeBuyContract<'info>{
    #[account(mut)]
    pub owner_account: Account<'info , User>,
    /// CHECK
    #[account(mut)]
    pub buyer : AccountInfo<'info>,
    #[account(zero)]
    pub buy_contract : Account<'info , BuyContract>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut)]
    pub asset_account : Account<'info,Asset>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Asset {
pub address:String,
pub location:String,
pub area: u64,
pub description:String,
pub user :Pubkey,
pub authority: Pubkey,
pub asset_value:u64
}
 
#[account]
pub struct BuyContract {
    pub owner_account:Pubkey,
    pub buyer : Pubkey,
    pub amount : u64,
    pub authority:Pubkey,
    pub asset_account:Pubkey
}
#[derive(Copy, Clone)]
pub enum AgreementStatus {
	Uninitialized = 0,
	DepositPending,
	Active,
	Completed,
	Terminated,
}

#[error_code]
pub enum ErrorCode {
	#[msg("Invalid Instruction")]
	InvalidInstruction,

	#[msg("Incorrect Payment Amount")]
	IncorrectPaymentAmount,

	#[msg("Full Rent Already Paid")]
	RentAlreadyFullyPaid,

	#[msg("Security Amount Already Deposited")]
	SecurityAlreadyDeposited,

	#[msg("Rent Agreement Already Terminated")]
	RentAgreementTerminated,

	#[msg("Invalid Agreement Status")]
	InvalidAgreementStatus,

	#[msg("Invalid Instruction Parameter")]
	InvalidInstructionParameter,
}