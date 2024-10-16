use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::collections::HashMap;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Device {
    pub metadata: HashMap<String, String>,
    pub data: HashMap<String, String>,
    pub description: String,
    pub name: String,
}

impl Device {
    pub fn new(name: String, description: String) -> Self {
        Self {
            metadata: HashMap::new(),
            data: HashMap::new(),
            description,
            name,
        }
    }

    pub fn set_metadata(&mut self, metadata: HashMap<String, String>) {
        self.metadata = metadata;
    }

    pub fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    pub fn set_data(&mut self, data: HashMap<String, String>) {
        self.data = data;
    }

    pub fn get_data(&self) -> &HashMap<String, String> {
        &self.data
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Registry {
    pub name: String,
    pub devices: HashMap<String, Device>,
    pub owner_id: Pubkey,
}

impl Registry {
    pub fn new(name: String, owner_id: Pubkey) -> Self {
        Self {
            name,
            devices: HashMap::new(),
            owner_id,
        }
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.insert(device.name.clone(), device);
    }

    pub fn get_device(&self, device_name: &str) -> Option<&Device> {
        self.devices.get(device_name)
    }

    pub fn exists(&self, device_name: &str) -> bool {
        self.devices.contains_key(device_name)
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub registries: HashMap<String, Registry>,
}

impl Contract {
    pub fn new() -> Self {
        Self {
            registries: HashMap::new(),
        }
    }

    pub fn create_registry(&mut self, registry_name: String, owner_id: Pubkey) -> bool {
        if self.registries.contains_key(&registry_name) {
            return false;
        }
        let new_registry = Registry::new(registry_name, owner_id);
        self.registries.insert(new_registry.name.clone(), new_registry);
        true
    }

    pub fn delete_registry(&mut self, registry_name: &str) -> bool {
        self.registries.remove(registry_name).is_some()
    }

    pub fn add_device_to_registry(
        &mut self,
        registry_name: &str,
        device_name: String,
        description: String,
    ) -> bool {
        if let Some(registry) = self.registries.get_mut(registry_name) {
            if registry.exists(&device_name) {
                return false;
            }
            let new_device = Device::new(device_name, description);
            registry.add_device(new_device);
            return true;
        }
        false
    }

    pub fn delete_device_from_registry(&mut self, registry_name: &str, device_name: &str) -> bool {
        if let Some(registry) = self.registries.get_mut(registry_name) {
            return registry.devices.remove(device_name).is_some();
        }
        false
    }

    pub fn get_device_data(&self, registry_name: &str, device_name: &str) -> Option<&HashMap<String, String>> {
        Some(self.registries.get(registry_name)?.get_device(device_name)?.get_data())
    }

    pub fn get_device_metadata(&self, registry_name: &str, device_name: &str) -> Option<&HashMap<String, String>> {
        Some(self.registries.get(registry_name)?.get_device(device_name)?.get_metadata())
    }
}

//entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("Account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Aquí iría la lógica para procesar las instrucciones específicas
    Ok(())
}
