use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::pubkey::Pubkey;
use solana_program_test::*;
use anchor_lang::InstructionData;
use anchor_lang::ToAccountInfos;
use anchor_lang::Accounts;
use anchor_lang::AccountSerialize;
use anchor_spl::token::{self, TokenAccount};
use std::collections::HashMap;

use registry_project::{self, *};

#[tokio::test]
async fn test_create_registry() {
    // Inicializa el contexto de prueba
    let mut program_test = ProgramTest::new(
        "registry_project",
        id(), // ID del programa
        processor!(registry_project::RegistryProject),
    );

    // Inicializa el banco de pruebas y el contexto
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Genera una clave pública para el registro
    let registry_pubkey = Pubkey::new_unique();
    
    // Llama a la instrucción de crear el registro
    let registry_name = "Test Registry".to_string();
    
    let mut registry_data = Registry {
        device_count: 0,
        name: registry_name.clone(),
        device_ids: Vec::new(),
    };

    let transaction = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(registry_pubkey, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data: registry_project::instruction::CreateRegistry {
                name: registry_name.clone(),
            }
            .data(),
        }],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction).await.unwrap();

    // Aquí puedes agregar aserciones para verificar que el registro fue creado correctamente
    // Por ejemplo, podrías verificar el contador de dispositivos, el nombre del registro, etc.
    let account = banks_client.get_account(registry_pubkey).await.unwrap().unwrap();
    let registry: Registry = Registry::try_from_slice(&account.data).unwrap();

    assert_eq!(registry.device_count, 0);
    assert_eq!(registry.name, registry_name);
}

#[tokio::test]
async fn test_add_device_to_registry() {
    // Inicializa el contexto de prueba
    let mut program_test = ProgramTest::new(
        "registry_project",
        id(), // ID del programa
        processor!(registry_project::RegistryProject),
    );

    // Inicializa el banco de pruebas y el contexto
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Crear registro
    let registry_pubkey = Pubkey::new_unique();
    let registry_name = "Test Registry".to_string();
    
    let transaction_create_registry = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(registry_pubkey, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data: registry_project::instruction::CreateRegistry {
                name: registry_name.clone(),
            }
            .data(),
        }],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction_create_registry).await.unwrap();

    // Crear dispositivo
    let device_pubkey = Pubkey::new_unique();
    let device_name = "Test Device".to_string();
    let device_description = "A test device for the registry".to_string();

    let transaction_add_device = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(device_pubkey, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(registry_pubkey, false),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data: registry_project::instruction::AddDevice {
                name: device_name.clone(),
                description: device_description.clone(),
            }
            .data(),
        }],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction_add_device).await.unwrap();

    // Aquí puedes agregar aserciones para verificar que el dispositivo fue agregado correctamente
    let account_registry = banks_client.get_account(registry_pubkey).await.unwrap().unwrap();
    let registry: Registry = Registry::try_from_slice(&account_registry.data).unwrap();

    assert_eq!(registry.device_count, 1);

    let account_device = banks_client.get_account(device_pubkey).await.unwrap().unwrap();
    let device: Device = Device::try_from_slice(&account_device.data).unwrap();

    assert_eq!(device.name, device_name);
    assert_eq!(device.description, device_description);
}

#[tokio::test]
async fn test_register_data() {
    let mut program_test = ProgramTest::new(
        "registry_project",
        id(),
        processor!(registry_project::RegistryProject),
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Crear un registro
    let registry_pubkey = Pubkey::new_unique();
    let registry_name = "My Registry".to_string(); // Nombre del registro

    let transaction_create_registry = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(registry_pubkey, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data: registry_project::instruction::CreateRegistry {
                name: registry_name.clone(),  // Asignar nombre del registro aquí
            }
            .data(),
        }],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction_create_registry).await.unwrap();

    // Agregar un dispositivo
    let device_pubkey = Pubkey::new_unique();
    let device_name = "Google Lampara Inteligente".to_string(); // Nombre del dispositivo
    let device_description = "Lampara de Google IoT".to_string(); // Descripción del dispositivo

    let transaction_add_device = Transaction::new_signed_with_payer(
        &[Instruction {
            program_id: id(),
            accounts: vec![
                AccountMeta::new(device_pubkey, false),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(registry_pubkey, false),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data: registry_project::instruction::AddDevice {
                name: device_name.clone(),          // Asignar nombre del dispositivo aquí
                description: device_description.clone(), // Asignar descripción del dispositivo aquí
            }
            .data(),
        }],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    banks_client.process_transaction(transaction_add_device).await.unwrap();

    // Verificar que el dispositivo se haya agregado correctamente
    let account_registry = banks_client.get_account(registry_pubkey).await.unwrap().unwrap();
    let registry: Registry = Registry::try_from_slice(&account_registry.data).unwrap();

    assert_eq!(registry.device_count, 1);
    assert!(registry.device_ids.contains(&device_pubkey));

    // Obtener y verificar el dispositivo
    let account_device = banks_client.get_account(device_pubkey).await.unwrap().unwrap();
    let device: Device = Device::try_from_slice(&account_device.data).unwrap();

    assert_eq!(device.name, device_name);              // Verificar el nombre del dispositivo
    assert_eq!(device.description, device_description); // Verificar la descripción del dispositivo
}
