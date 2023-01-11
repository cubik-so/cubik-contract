use crate::state::*;
use anchor_lang::prelude::*;


#[derive(Accounts)]
#[instruction(_projectname : String)]
pub struct ProjectsChortInit<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"project".as_ref(),_projectname.as_ref(),authority.key().as_ref()],

        bump = project_account.bump
    )]
    pub project_account: Account<'info, ProjectChortInit>,
}

pub fn handler(ctx: Context<ProjectsChortInit>,_projectname:String,chortname:String) -> Result<()> {
  let project_account = &mut ctx.accounts.project_account;
    project_account.chort_name =  chortname;
    project_account.project_name = _projectname ;
    project_account.bump = *ctx.bumps.get("project_account").unwrap();
    Ok(())
}