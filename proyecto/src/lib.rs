use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::program::invoke;

declare_id!("FL7CDVdaisWQH7bKJ5AbncuDuf2cBGMKRUJWZs9Luik6");

#[program]
pub mod micro_grant {
    use super::*;

    pub fn crear_proyecto(ctx: Context<CrearProyecto>, nombre: String, meta: u64) -> Result<()> {
        let proyecto = &mut ctx.accounts.proyecto;
        proyecto.autor = *ctx.accounts.autor.key;
        proyecto.nombre = nombre;
        proyecto.meta = meta;
        proyecto.recaudado = 0;
        msg!("🚀 Proyecto '{}' creado. Meta: {} lamports", proyecto.nombre, proyecto.meta);
        Ok(())
    }

    pub fn donar(ctx: Context<Donar>, cantidad: u64) -> Result<()> {
        let proyecto = &mut ctx.accounts.proyecto;
        
        // Transferencia de SOL del donador a la PDA del proyecto
        let ix = system_instruction::transfer(
            &ctx.accounts.donador.key(),
            &proyecto.key(),
            cantidad,
        );
        
        invoke(
            &ix,
            &[
                ctx.accounts.donador.to_account_info(),
                proyecto.to_account_info(),
            ],
        )?;

        proyecto.recaudado += cantidad;
        msg!("✅ Donación de {} lamports recibida!", cantidad);
        Ok(())
    }

    pub fn retirar_fondos(ctx: Context<RetirarFondos>) -> Result<()> {
        let proyecto = &ctx.accounts.proyecto;
        let recaudado = proyecto.recaudado;

        // Verificamos que se haya alcanzado la meta
        if recaudado < proyecto.meta {
            return err!(ErrorCode::MetaNoAlcanzada);
        }

        // Transferencia de la PDA al autor
        **proyecto.to_account_info().try_borrow_mut_lamports()? -= recaudado;
        **ctx.accounts.autor.try_borrow_mut_lamports()? += recaudado;

        msg!("💰 Fondos retirados exitosamente por el autor.");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct CrearProyecto<'info> {
    #[account(
        init, 
        payer = autor, 
        space = 8 + 32 + 4 + nombre.len() + 8 + 8,
        seeds = [b"grant", nombre.as_bytes()], 
        bump
    )]
    pub proyecto: Account<'info, Project>,
    #[account(mut)]
    pub autor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Donar<'info> {
    #[account(mut)]
    pub proyecto: Account<'info, Project>,
    #[account(mut)]
    pub donador: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RetirarFondos<'info> {
    #[account(mut, has_one = autor)]
    pub proyecto: Account<'info, Project>,
    #[account(mut)]
    pub autor: Signer<'info>,
}

#[account]
pub struct Project {
    pub autor: Pubkey,
    pub nombre: String,
    pub meta: u64,
    pub recaudado: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Aún no se ha alcanzado la meta de recaudación.")]
    MetaNoAlcanzada,
}
