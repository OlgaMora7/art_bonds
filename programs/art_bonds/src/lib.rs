use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("11111111111111111111111111111111"); 

#[program]
pub mod art_bonds {
    use super::*;

    // CREATE: Emitir el Bono de Arte (Se crea la PDA)
    pub fn issue_bond(ctx: Context<IssueBond>, principal_amount: u64) -> Result<()> {
        let bond = &mut ctx.accounts.bond_pda;
        let clock = Clock::get()?;

        // Registramos los datos iniciales
        bond.owner = ctx.accounts.user.key();
        bond.principal = principal_amount;
        bond.issue_date = clock.unix_timestamp;
        bond.last_claim_date = clock.unix_timestamp;
        bond.bump = ctx.bumps.bond_pda;

        // Transferimos el SOL a la PDA
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.bond_pda.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, principal_amount)?;

        Ok(())
    }

    // UPDATE: Actualizar y "Reclamar" el rendimiento simulado
    pub fn update_yield(ctx: Context<UpdateYield>) -> Result<()> {
        let bond = &mut ctx.accounts.bond_pda;
        let clock = Clock::get()?;

        let time_passed = clock.unix_timestamp - bond.last_claim_date;
        let simulated_yield = (bond.principal as i64 * time_passed * 10) / 100;
        
        msg!("Rendimiento generado desde el último cobro: {}", simulated_yield);

        bond.last_claim_date = clock.unix_timestamp;

        Ok(())
    }

    // DELETE: Liquidar el bono y recuperar la inversión
    pub fn redeem_bond(ctx: Context<RedeemBond>) -> Result<()> {
        let bond = &ctx.accounts.bond_pda;
        
        msg!("Bono liquidado. Se devolvieron {} lamports al usuario.", bond.principal);

        // Al cerrar la cuenta, Anchor devuelve los fondos al usuario automáticamente.
        Ok(())
    }
}

// READ: La estructura de datos que se guarda en la blockchain
#[account]
pub struct BondState {
    pub owner: Pubkey,
    pub principal: u64,
    pub issue_date: i64,
    pub last_claim_date: i64,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct IssueBond<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 8 + 8 + 1,
        seeds = [b"bond", user.key().as_ref()], // Aquí definimos que es una PDA
        bump
    )]
    pub bond_pda: Account<'info, BondState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateYield<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"bond", user.key().as_ref()],
        bump = bond_pda.bump,
        has_one = owner
    )]
    pub bond_pda: Account<'info, BondState>,
}

#[derive(Accounts)]
pub struct RedeemBond<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user, // Esta instrucción cierra la PDA y hace el "Delete"
        seeds = [b"bond", user.key().as_ref()],
        bump = bond_pda.bump,
        has_one = owner
    )]
    pub bond_pda: Account<'info, BondState>,
} 