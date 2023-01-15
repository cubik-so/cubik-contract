use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod cubic_contract {
    use super::*;

    // pub fn create_project(ctx: Context<ProjectsChortInit>,_projectname:String,chortname:String) -> Result<()>{
    //     project_chort_init::handler(ctx,_projectname,chortname)
    // }
}
