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

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;
    use anchor_lang::solana_program::{pubkey::Pubkey, system_program};
    use anchor_lang::ProgramTest;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_create_registry_and_add_devices() {
        // Configura el entorno de prueba
        let mut program_test = ProgramTest::new(
            "registry_project", // Nombre de tu programa
            Pubkey::from_str("5zfjxFExnrGcuUoSDgnXxm1vtq3jtLePVSv5Awy").unwrap(),
            processor!(process_instruction), // Cambia esto si tienes un procesador específico
        );

        // Crea un nuevo usuario
        let user_keypair = Keypair::new();
        program_test.add_account(user_keypair.pubkey(), 1_000_000_000, &system_program::ID);

        // Crear cuentas para el registro y el dispositivo
        let (registry_pubkey, _) = Pubkey::find_program_address(&[b"registry"], &program_test.program_id);
        let (device_pubkey_1, _) = Pubkey::find_program_address(&[b"device1"], &program_test.program_id);
        let (device_pubkey_2, _) = Pubkey::find_program_address(&[b"device2"], &program_test.program_id);

        // Añadir cuentas para el registro
        program_test.add_account(
            registry_pubkey,
            Account {
                lamports: 0,
                data: vec![0; std::mem::size_of::<Registry>()], // Espacio para el registro
                owner: program_test.program_id,
                executable: false,
                rent_epoch: 0,
            },
        );

        let mut banks_client = program_test.start().await.unwrap();

        // Crear el registro
        let create_registry_ix = registry_project::create_registry(
            Context::new(
                &mut banks_client,
                CreateRegistry {
                    registry: AccountInfo::new(
                        &registry_pubkey,
                        false,
                        true,
                        &mut 0,
                        &mut vec![0; std::mem::size_of::<Registry>()],
                        &program_test.program_id,
                        false,
                        0,
                    ),
                    user: user_keypair.pubkey(),
                    system_program: system_program::ID,
                },
            ),
            "MyRegistry".to_string(), // Nombre del registro
        );

        assert!(create_registry_ix.is_ok());

        // Verificar que el registro fue creado
        let account = banks_client
            .get_account(registry_pubkey)
            .await
            .unwrap()
            .unwrap();

        let registry_data = Registry::try_from_slice(&account.data).unwrap();
        assert_eq!(registry_data.device_count, 0);

        // Añadir el primer dispositivo al registro
        let add_device_ix_1 = registry_project::add_device(
            Context::new(
                &mut banks_client,
                AddDevice {
                    device: AccountInfo::new(
                        &device_pubkey_1,
                        false,
                        true,
                        &mut 0,
                        &mut vec![0; std::mem::size_of::<Device>()],
                        &program_test.program_id,
                        false,
                        0,
                    ),
                    registry: registry_pubkey,
                    user: user_keypair.pubkey(),
                    system_program: system_program::ID,
                },
            ),
            "TestDevice1".to_string(),
            "Test Description 1".to_string(),
        );

        assert!(add_device_ix_1.is_ok());

        // Verificar que el primer dispositivo fue agregado
        let registry_account = banks_client
            .get_account(registry_pubkey)
            .await
            .unwrap()
            .unwrap();

        let updated_registry_data = Registry::try_from_slice(&registry_account.data).unwrap();
        assert_eq!(updated_registry_data.device_count, 1);

        // Añadir el segundo dispositivo al registro
        let add_device_ix_2 = registry_project::add_device(
            Context::new(
                &mut banks_client,
                AddDevice {
                    device: AccountInfo::new(
                        &device_pubkey_2,
                        false,
                        true,
                        &mut 0,
                        &mut vec![0; std::mem::size_of::<Device>()],
                        &program_test.program_id,
                        false,
                        0,
                    ),
                    registry: registry_pubkey,
                    user: user_keypair.pubkey(),
                    system_program: system_program::ID,
                },
            ),
            "TestDevice2".to_string(),
            "Test Description 2".to_string(),
        );

        assert!(add_device_ix_2.is_ok());

        // Verificar que el segundo dispositivo fue agregado
        let updated_registry_account = banks_client
            .get_account(registry_pubkey)
            .await
            .unwrap()
            .unwrap();

        let final_registry_data = Registry::try_from_slice(&updated_registry_account.data).unwrap();
        assert_eq!(final_registry_data.device_count, 2);

        // Verificar que el primer dispositivo fue creado correctamente
        let device_account_1 = banks_client
            .get_account(device_pubkey_1)
            .await
            .unwrap()
            .unwrap();

        let device_data_1 = Device::try_from_slice(&device_account_1.data).unwrap();
        assert_eq!(device_data_1.name, "TestDevice1");
        assert_eq!(device_data_1.description, "Test Description 1");

        // Verificar que el segundo dispositivo fue creado correctamente
        let device_account_2 = banks_client
            .get_account(device_pubkey_2)
            .await
            .unwrap()
            .unwrap();

        let device_data_2 = Device::try_from_slice(&device_account_2.data).unwrap();
        assert_eq!(device_data_2.name, "TestDevice2");
        assert_eq!(device_data_2.description, "Test Description 2");
    }
}
