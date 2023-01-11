use anchor_lang::prelude::*;


#[account]
#[derive(Default)]
pub struct ProjectInit {
    pub project_admin: Pubkey,
    
}



#[account]
#[derive(Default)]
pub struct ProjectChortInit {
    pub chort_name : String,
    pub project_name:String,
    pub owner:Pubkey,
    pub bump: u8,
}

