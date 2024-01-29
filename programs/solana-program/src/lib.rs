use anchor_lang::prelude::*;


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


pub mod sysvar_address_checking;
pub mod account_data_matching;
pub mod signer_authorization;
pub mod owner_checks;
pub mod type_cosplay;
pub mod initialization;
pub mod arbitrary_cpi;
pub mod bump_seed_canonicalization;
pub mod closing_accounts;

#[program]
pub mod program_test {

}