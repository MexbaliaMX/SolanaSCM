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

#[derive(BorshDeserialize, BorshSerialize, Clone)]
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

    pub fn get_name(&self) -> String {
        self.name.clone()  // Usamos clone para devolver una nueva String
    }

    pub fn get_description(&self) -> String {
        self.description.clone()  // Usamos clone para devolver una nueva String
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

    pub fn get_metadata_param(&self, param: String) -> String {
        self.metadata.get(&param).unwrap_or(&"".to_string()).clone()
    }

    pub fn set_metadata_param(&mut self, param: String, value: String) {
        self.metadata.insert(param, value);
    }

    pub fn get_data_param(&self, param: String) -> String {
        self.data.get(&param).unwrap_or(&"".to_string()).clone()
    }

    pub fn set_data_param(&mut self, param: String, value: String) {
        self.data.insert(param, value);
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
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

#[derive(BorshDeserialize, BorshSerialize, Clone)]
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
            let new_device = Device::new(device_name.clone(), description.clone());
            
            // Aquí puedes utilizar los campos si es necesario
            println!("Adding device: {} - {}", new_device.get_name(), new_device.get_description());
    
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

    pub fn exists(&self, registry_name: &str, device_name: &str) -> bool {
        if let Some(registry) = self.registries.get(registry_name) {
            registry.exists(device_name)
        } else {
            false
        }
    }
    

    pub fn get_device_data(&self, registry_name: &str, device_name: &str) -> Option<&HashMap<String, String>> {
        let registry = self.registries.get(registry_name)?;
        let device = registry.get_device(device_name)?;
        Some(device.get_data())
    }   

    pub fn get_device_metadata(&self, registry_name: &str, device_name: &str) -> Option<&HashMap<String, String>> {
        let registry = self.registries.get(registry_name)?;
        let device = registry.get_device(device_name)?;
        Some(device.get_metadata())
    }
    

    pub fn get_device_metadata_param(
        &self,
        registry_name: String,
        device_name: String,
        param: String,
        signer_account: &Pubkey, // Agregado
    ) -> String {
        if !self.validate_registry(registry_name.clone(), signer_account) {
            return "Not registry or not allowed".to_string();
        }
        let current_registry = self.registries.get(&registry_name).unwrap();
        if !self.validate_exists_device(&current_registry, device_name.clone()) {
            return "Not device".to_string();
        }
        let current_device = current_registry.devices.get(&device_name).unwrap();
        current_device.get_metadata_param(param)
    }

    pub fn validate_before_setting_data(
        &self,
        registry_name: &String,
        signer_account: &Pubkey,
        device_name: &String,
    ) -> bool {
        self.validate_registry(registry_name.clone(), signer_account) &&
        self.validate_exists_registry(registry_name.clone()) &&
        self.validate_exists_device(&self.registries.get(registry_name).unwrap(), device_name.clone())
    }
    
    pub fn set_device_data(
        &mut self,
        registry_name: String,
        device_name: String,
        data: String,
        signer_account: &Pubkey,
    ) -> bool {
        // Validar antes de cualquier operación mutable
        if !self.validate_before_setting_data(&registry_name, signer_account, &device_name) {
            return false;
        }
        // Obtener una referencia mutable al registro
        let current_registry = self.registries.get_mut(&registry_name).unwrap();
        // Obtener una referencia mutable al dispositivo
        let current_device = current_registry.devices.get_mut(&device_name).unwrap();
        // Convertir el string de datos a un HashMap
        let aux_map: HashMap<String, String> = serde_json::from_str(&data).unwrap();
        // Establecer los datos del dispositivo
        current_device.set_data(aux_map);
        true
    }

    fn validate_before_setting_metadata(
        &self,
        registry_name: &String,
        signer_account: &Pubkey,
        device_name: &String,
    ) -> bool {
        self.validate_registry(registry_name.clone(), signer_account) &&
        self.validate_exists_registry(registry_name.clone()) &&
        self.validate_exists_device(&self.registries.get(registry_name).unwrap(), device_name.clone())
    }    
    
    pub fn set_device_metadata(
        &mut self,
        registry_name: String,
        device_name: String,
        metadata: String,
        data: String, // Aquí estás definiendo `data` como un parámetro de la función
        signer_account: &Pubkey,
    ) -> bool {
        // Validar antes de cualquier operación mutable
        if !self.validate_before_setting_metadata(&registry_name, signer_account, &device_name) {
            return false;
        }
        // Obtener una referencia mutable al registro
        let current_registry = self.registries.get_mut(&registry_name).unwrap();
        // Obtener una referencia mutable al dispositivo
        let current_device = current_registry.devices.get_mut(&device_name).unwrap();
        // Convertir el string de metadatos a un HashMap
        let aux_map: HashMap<String, String> = match serde_json::from_str(&data) {
            Ok(map) => map,
            Err(_) => return false,
        };
        // Establecer los metadatos del dispositivo
        current_device.set_metadata(aux_map);

        true
    }

    fn validate_before_setting_data_param(
        &self,
        registry_name: &String,
        signer_account: &Pubkey,
        device_name: &String,
    ) -> bool {
        self.validate_registry(registry_name.clone(), signer_account) &&
        self.validate_exists_registry(registry_name.clone()) &&
        self.validate_exists_device(&self.registries.get(registry_name).unwrap(), device_name.clone())
    }
    
    pub fn set_device_data_param(
        &mut self,
        registry_name: String,
        device_name: String,
        param: String,
        value: String,
        signer_account: &Pubkey,
    ) -> bool {
        // Validar antes de cualquier operación mutable
        if !self.validate_before_setting_data_param(&registry_name, signer_account, &device_name) {
            return false;
        }
        // Obtener una referencia mutable al registro
        let current_registry = self.registries.get_mut(&registry_name).unwrap();
        // Obtener una referencia mutable al dispositivo
        let current_device = current_registry.devices.get_mut(&device_name).unwrap();
        // Establecer el parámetro de datos del dispositivo
        current_device.set_data_param(param, value);
        true
    }
    
    fn validate_before_setting_metadata_param(
        &self,
        registry_name: &String,
        signer_account: &Pubkey,
        device_name: &String,
    ) -> bool {
        self.validate_registry(registry_name.clone(), signer_account) &&
        self.validate_exists_registry(registry_name.clone()) &&
        self.validate_exists_device(&self.registries.get(registry_name).unwrap(), device_name.clone())
    }
    
    pub fn set_device_metadata_param(
        &mut self,
        registry_name: String,
        device_name: String,
        param: String,
        value: String,
        signer_account: &Pubkey,
    ) -> bool {
        // Validar antes de cualquier operación mutable
        if !self.validate_before_setting_metadata_param(&registry_name, signer_account, &device_name) {
            return false;
        }
        // Obtener una referencia mutable al registro
        let current_registry = self.registries.get_mut(&registry_name).unwrap();
        // Obtener una referencia mutable al dispositivo
        let current_device = current_registry.devices.get_mut(&device_name).unwrap();
        // Establecer el parámetro de metadatos del dispositivo
        current_device.set_metadata_param(param, value);
        true
    }
    
    fn validate_owner(&self, registry_name: String, signer_account: &Pubkey) -> bool {
        if let Some(registry) = self.registries.get(&registry_name) {
            &registry.owner_id == signer_account
        } else {
            false
        }
    }
    
    fn validate_exists_registry(&self, registry_name: String) -> bool {
        self.registries.get(&registry_name).is_some()
    }
    
    fn validate_registry(&self, registry_name: String, signer_account: &Pubkey) -> bool {
        self.validate_exists_registry(registry_name.clone())
            && self.validate_owner(registry_name.clone(), signer_account)
    }
    
    fn validate_exists_device(&self, registry: &Registry, device_name: String) -> bool {
        registry.devices.get(&device_name).is_some()
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

/*-------------------UNIT TESTS -----------------------*/

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;
    use solana_program::pubkey::Pubkey;
    use solana_program::account_info::AccountInfo;
    use solana_program::sysvar;
}

#[test]
fn test_add_device_to_registry() {
    let mut contract = Contract::new();
    let owner_id = Pubkey::new_unique();
    let registry_name = "TestRegistry".to_string();
    contract.create_registry(registry_name.clone(), owner_id);
    
    let device_name = "TestDevice".to_string();
    let description = "Test Description".to_string();
    
    assert!(contract.add_device_to_registry(&registry_name, device_name.clone(), description));
    assert!(contract.exists(&registry_name, &device_name));
}