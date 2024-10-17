use anchor_lang::prelude::*;
use ::borsh::{BorshDeserialize, BorshSerialize};
use std::collections::HashMap;

declare_id!("5zfjxFExnrGcuUoSVJji6DgnXxm1vtq3jtLePVSv5Awy"); // ID del programa

#[program]
pub mod registry_project {
    use super::*;

    pub fn create_registry(ctx: Context<CreateRegistry>, name: String) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.device_count = 0;
        registry.name = name;
        Ok(())
    }

    pub fn add_device(ctx: Context<AddDevice>, name: String, description: String) -> Result<()> {
        let device = &mut ctx.accounts.device;
        device.name = name;
        device.description = description;

        let registry = &mut ctx.accounts.registry;
        registry.device_count += 1;
        registry.device_ids.push(device.key());
        Ok(())
    }

    pub fn set_device_metadata(ctx: Context<SetDeviceMetadata>, device_name: String, metadata: HashMap<String, String>) -> Result<()> {
        let device = &mut ctx.accounts.device;
        device.metadata = metadata;
        Ok(())
    }

    // Nueva función para crear registro y dispositivos
    pub fn create_registry_with_devices(ctx: Context<CreateRegistryWithDevices>, registry_name: String, device1_name: String, device1_description: String, device2_name: String, device2_description: String) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.device_count = 0;
        registry.name = registry_name;
        registry.device_ids = Vec::new(); // Inicializa el vector de IDs de dispositivos

        // Añade el primer dispositivo
        let device1 = &mut ctx.accounts.device1;
        device1.name = device1_name;
        device1.description = device1_description;
        registry.device_count += 1;
        registry.device_ids.push(device1.key());

        Ok(())
    }
}

#[account]
pub struct Registry {
    pub device_count: u64,
    pub name: String,
    pub device_ids: Vec<Pubkey>, // Arreglo de IDs de dispositivos
}

#[account]
pub struct Device {
    pub name: String,
    pub description: String,
    pub metadata: HashMap<String, String>, // Campos adicionales de metadatos
}

#[derive(Accounts)]
pub struct CreateRegistry<'info> {
    #[account(init, payer = user, space = 8 + 8 + 4 + 32 * 100)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddDevice<'info> {
    #[account(init, payer = user, space = 8 + 4 + 32)]
    pub device: Account<'info, Device>,
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetDeviceMetadata<'info> {
    #[account(mut)]
    pub device: Account<'info, Device>,
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Nueva estructura para crear registro y dispositivos
#[derive(Accounts)]
pub struct CreateRegistryWithDevices<'info> {
    #[account(init, payer = user, space = 8 + 8 + 4 + 32 * 100)]
    pub registry: Account<'info, Registry>,
    
    #[account(init, payer = user, space = 8 + 4 + 32)]
    pub device1: Account<'info, Device>,
    
    #[account(init, payer = user, space = 8 + 4 + 32)]
    pub device2: Account<'info, Device>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
