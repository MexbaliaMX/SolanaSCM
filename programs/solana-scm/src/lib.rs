use anchor_lang::prelude::*;
use anchor_lang::Accounts; // Asegúrate de que esté importado
use std::io::{self, Read};
declare_id!("A5i8uPKdCycDG3nbGCCAUiLzHEc4ddpfeYGQhPEWuaTJ");

#[program]
pub mod registry_project {
    use super::*;
    pub fn create_registry(ctx: Context<CreateRegistry>, name: String) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.device_count = 0;
        registry.device_ids = Vec::new();
        registry.name = name;
        registry.owner_id = *ctx.accounts.user.key;
        registry.devices = Vec::new();
        Ok(())
    }
    pub fn add_device(
        ctx: Context<AddDevice>,
        name: String,
        description: String,
        metadata: Vec<(String, String)>,
        data: Vec<(String, String)>,
    ) -> Result<()> {
        let device = Device {
            name: name.clone(), // Aquí debería funcionar si 'name' existe en Device
            description,
            metadata,
            data,
        };
        let registry = &mut ctx.accounts.registry;
        registry.device_count += 1;
        registry.device_ids.push(ctx.accounts.device.key());
        registry.devices.push((name, device));
        Ok(())
    }
    pub fn set_device_metadata(
        ctx: Context<SetDeviceMetadata>,
        name: String,
        metadata: Vec<(String, String)>,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let device = registry.devices.iter_mut().find(|(n, _)| *n == name).ok_or(ProgramError::InvalidArgument)?;
        device.1.metadata = metadata;
        Ok(())
    }
    pub fn set_device_data(
        ctx: Context<SetDeviceData>,
        name: String,
        data: Vec<(String, String)>,
    ) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        let device = registry.devices.iter_mut().find(|(n, _)| *n == name).ok_or(ProgramError::InvalidArgument)?;
        device.1.data = data;
        Ok(())
    }
    pub fn set_device_metadata_param(
        ctx: Context<SetDeviceMetadataParam>,
        registry_name: String,
        device_name: String,
        param: String,
        value: String,
    ) -> Result<()> {
        // Usa la referencia inmutable primero para validación
        if !ctx.accounts.contract.validate_before_setting_metadata_param(&registry_name, ctx.accounts.user.key, &device_name) {
            return Err(Error::from(ProgramError::InvalidArgument));
        }
        
        // Después accede a la referencia mutable para modificar los datos
        let contract = &mut ctx.accounts.contract;
        let registry = contract.registries.iter_mut().find(|(n, _)| *n == registry_name).ok_or(Error::from(ProgramError::InvalidArgument))?;
        let device = registry.1.devices.iter_mut().find(|(n, _)| *n == device_name).unwrap();
        device.1.metadata.push((param, value));
        
        Ok(())
    }
}

#[account]
pub struct Registry {
    pub device_count: u64,
    pub device_ids: Vec<Pubkey>,
    pub name: String,
    pub owner_id: Pubkey,
    pub devices: Vec<(String, Device)>,
}

#[account]
pub struct Device {
    pub name: String,
    pub metadata: Vec<(String, String)>,
    pub data: Vec<(String, String)>,
    pub description: String,
}

#[derive(Accounts)] // Asegúrate de que todas las estructuras tengan esta anotación
pub struct CreateRegistry<'info> {
    #[account(init, payer = user, space = 8 + 8 + 4 + 32 * 100 + 40)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddDevice<'info> {
    #[account(init, payer = user, space = 8 + 4 + 32 + 100)]
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
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetDeviceData<'info> {
    #[account(mut)]
    pub registry: Account<'info, Registry>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetDeviceMetadataParam<'info> {
    #[account(mut)]   // Asegúrate de que sea mutable
    pub contract: Account<'info, Contract>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Contract {
    pub registries: Vec<(String, Registry)>,
}

impl Contract {
    pub fn validate_before_setting_metadata_param(
        &self,
        registry_name: &String,
        signer_account: &Pubkey,
        device_name: &String,
    ) -> bool {
        self.validate_registry(registry_name, signer_account)
            && self.validate_exists_registry(registry_name)
            && self.validate_exists_device(
                &self.registries.iter().find(|(n, _)| *n == *registry_name).unwrap().1, 
                device_name
            )
    }

    pub fn validate_owner(&self, registry_name: String, signer_account: &Pubkey) -> bool {
        if let Some((_, registry)) = self.registries.iter().find(|(n, _)| *n == registry_name) {
            &registry.owner_id == signer_account
        } else {
            false
        }
    }

    pub fn validate_exists_registry(&self, registry_name: &String) -> bool {
        self.registries.iter().any(|(n, _)| *n == *registry_name)
    }

    pub fn validate_registry(&self, registry_name: &String, signer_account: &Pubkey) -> bool {
        self.validate_exists_registry(registry_name)
            && self.validate_owner(registry_name.clone(), signer_account)
    }

    pub fn validate_exists_device(&self, registry: &Registry, device_name: &String) -> bool {
        registry.devices.iter().any(|(n, _)| *n == *device_name)
    }
}
