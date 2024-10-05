use anchor_lang::prelude::*;
mod lib2;
mod lib3;
mod devices;
mod registries;


declare_id!("5zfjxFExnrGcuUoSVJji6DgnXxm1vtq3jtLePVSv5Awy"); //Obtenido de Solana address

#[program]
pub mod registry_project {
    use super::*;

    pub fn create_registry(ctx: Context<CreateRegistry>) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.device_count = 0;
        Ok(())
    }

    pub fn add_device(
        ctx: Context<AddDevice>,
        name: String,
        description: String,
    ) -> Result<()> {
        let device = &mut ctx.accounts.device;
        device.name = name;
        device.description = description;

        let registry = &mut ctx.accounts.registry;
        registry.device_count += 1;
        registry.device_ids.push(device.key());
        Ok(())
    }
}

#[account]
pub struct Registry {
    pub device_count: u64,
    pub device_ids: Vec<Pubkey>, //Arreglo de IDs de dispositivos
} //Estructura de registro

#[account]
pub struct Device {
    pub name: String,
    pub description: String,
} //Estructura de dispositivo

#[derive(Accounts)]
pub struct CreateRegistry<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32 * 100)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
} 

#[derive(Accounts)]
pub struct AddDevice<'info> {
    #[account(init, payer = user, space = 8 + 32 + 32)]
    pub device: Account<'info, Device>,
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
