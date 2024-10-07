// Importaciones de Borsh
use ::borsh::{BorshDeserialize, BorshSerialize}; // Desambiguación para evitar conflictos
// Importaciones de Solana
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
// Importaciones de Anchor
use anchor_lang::prelude::*;
// Importaciones de Serde
use serde_json; // Importa lo que necesites
// Importaciones de ThisError
use thiserror::Error;
// Importación estándar
use std::collections::HashMap;
use std::result::Result; 


// Definición del tipo de error
#[derive(Error, Debug)]
pub enum MyError {
    #[error("Formato de datos no válido")]
    InvalidDataFormat,
    #[error("No se encontró el registro")]
    RegistryNotFound,
    #[error("No se encontró el dispositivo")]
    DeviceNotFound,
    #[error("No está en el registro")]
    NotAllowed,
}

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

    pub fn set_metadata(&mut self, metadata: HashMap<String, String>) {
        self.metadata = metadata;
    }

    pub fn get_metadata(&self) -> Option<&HashMap<String, String>>{
        if self.data.is_empty(){
            None // Devuelve None si no hay datos
        } else{
            Some(&self.data)  // Devuelve una referencia a los datos si hay
        }
    }

    pub fn set_data(&mut self, data: HashMap<String, String>) {
        self.data = data;
    }

    pub fn get_data(&self) -> Option<&HashMap<String, String>> {
        if self.data.is_empty() {
            None // Devuelve None si no hay datos
        } else {
            Some(&self.data) // Devuelve una referencia a los datos si hay
        }
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
        self.registries.get(registry_name)?.get_device(device_name)?.get_data()
    }

    pub fn get_device_metadata(&self, registry_name: &str, device_name: &str) -> Option<&HashMap<String, String>> {
        self.registries.get(registry_name)?.get_device(device_name)?.get_metadata()
    }

    pub fn get_device_metadata_param(
        &self,
        ctx: Context<ValidateRegistry>, // Usar el contexto correcto
        device_name: String,
        param: String,
    ) -> String {
        let registry_name = ctx.accounts.registry.name.clone(); // Obtener el nombre del registro del contexto
        // Validación de registro
        if self.validate_registry(&ctx, registry_name.as_str()).is_err() { // Pasa referencia
            return "Not registry or not allowed".to_string();
        }
        let current_registry = self.registries.get(&registry_name).unwrap();
        // Validación de existencia de dispositivo
        if self.validate_exists_device(&ctx, &device_name).is_err() { // Usar una referencia a ctx
            return "Not device".to_string();
        }
        let current_device = current_registry.devices.get(&device_name).unwrap();
        current_device.get_metadata_param(param)
    }

    pub fn set_device_data(
        &mut self,
        ctx: Context<ValidateRegistry>,
        device_name: String,
        data: String,
    ) -> Result<(), MyError> { // Cambia a tu tipo de error
        let registry_name = ctx.accounts.registry.name.clone();
        // Validación de registro
        if self.validate_registry(&ctx, registry_name.as_str()).is_err() {
            return Err(MyError::NotAllowed);
        }
        let current_registry = self.registries.get_mut(&registry_name)
            .ok_or(MyError::RegistryNotFound)?;
        // Validación de existencia de dispositivo
        if self.validate_exists_device(&ctx, &device_name).is_err() {
            return Err(MyError::DeviceNotFound);
        }
        let current_device = current_registry.devices.get_mut(&device_name)
            .ok_or(MyError::DeviceNotFound)?;
        let aux_map: HashMap<String, String> = serde_json::from_str(&data)
            .map_err(|_| MyError::InvalidDataFormat)?;
        current_device.set_data(aux_map);
        Ok(())
    }

    pub fn set_device_metadata(
        &mut self,
        ctx: Context<ValidateRegistry>, // Cambiar a usar Context
        device_name: String,
        metadata: String,
    ) -> Result<(), MyError> {
        let registry_name = ctx.accounts.registry.name.clone(); // Obtener el nombre del registro
        // Validación de registro
        if self.validate_registry(&ctx, registry_name.as_str()).is_err() {
            return Err(MyError::NotAllowed);
        }
        let current_registry = self.registries.get_mut(&registry_name)
            .ok_or(MyError::RegistryNotFound)?; // Manejo del error si no se encuentra el registro
        // Validación de existencia de dispositivo
        if self.validate_exists_device(&ctx, &device_name).is_err() {
            return Err(MyError::DeviceNotFound);
        }
        let current_device = current_registry.devices.get_mut(&device_name)
            .ok_or(MyError::DeviceNotFound)?; // Manejo del error si no se encuentra el dispositivo
        let aux_map: HashMap<String, String> = serde_json::from_str(&metadata)
            .map_err(|_| MyError::InvalidDataFormat)?; // Manejo del error si hay un problema de formato
        current_device.set_metadata(aux_map);
        Ok(()) // Retorna Ok si todo salió bien
    }

    pub fn set_device_data_param(
        &mut self,
        ctx: Context<ValidateRegistry>, // Usar Context
        device_name: String,
        param: String,
        value: String,
    ) -> Result<(), MyError> { // Cambiar a Result
        let registry_name = ctx.accounts.registry.name.clone(); // Obtener el nombre del registro
        // Validación de registro
        if self.validate_registry(&ctx, registry_name.as_str()).is_err() {
            return Err(MyError::NotAllowed);
        }
        let current_registry = self.registries.get_mut(&registry_name)
            .ok_or(MyError::RegistryNotFound)?; // Manejo del error si no se encuentra el registro
        // Validación de existencia de dispositivo
        if self.validate_exists_device(&ctx, &device_name).is_err() {
            return Err(MyError::DeviceNotFound);
        }
        let current_device = current_registry.devices.get_mut(&device_name)
            .ok_or(MyError::DeviceNotFound)?; // Manejo del error si no se encuentra el dispositivo
        current_device.set_data_param(param, value);
        Ok(()) // Retorna Ok si todo salió bien
    }


    pub fn set_device_metadata_param(
        &mut self,
        ctx: Context<ValidateRegistry>, // Cambiar a usar Context
        device_name: String,
        param: String,
        value: String,
    ) -> Result<(), MyError> { // Cambiar a Result
        let registry_name = ctx.accounts.registry.name.clone(); // Obtener el nombre del registro
        // Validación de registro
        if self.validate_registry(&ctx, registry_name.as_str()).is_err() {
            return Err(MyError::NotAllowed);
        }
        let current_registry = self.registries.get_mut(&registry_name)
            .ok_or(MyError::RegistryNotFound)?; // Manejo del error si no se encuentra el registro
        // Validación de existencia de dispositivo
        if self.validate_exists_device(&ctx, &device_name).is_err() {
            return Err(MyError::DeviceNotFound);
        }
        let current_device = current_registry.devices.get_mut(&device_name)
            .ok_or(MyError::DeviceNotFound)?; // Manejo del error si no se encuentra el dispositivo
        current_device.set_metadata_param(param, value);
        Ok(()) // Retorna Ok si todo salió bien
    }
    /*
    //#[private] Rust no reconoce el atributo #[private].
    fn validate_owner(&self, registry_name: String) -> bool {
        let registry = self.registries.get(&registry_name).unwrap();
        registry.owner_id == env::signer_account_id().to_string() // a función signer_account_id pertenece al contexto de NEAR
    }

    //#[private]
    fn validate_exists_registry(&self, registry_name: String) -> bool {
        self.registries.get(&registry_name).is_some()
    }

   // #[private]
    fn validate_registry(&self, registry_name: String) -> bool {
        self.validate_exists_registry(registry_name.clone())
            && self.validate_owner(registry_name.clone())
    }

   // #[private]
    fn validate_exists_device(&self, registry: &Registry, device_name: String) -> bool {
        registry.devices.get(&device_name).is_some()
    } */

    fn validate_owner(&self, ctx: Context<ValidateOwner>, registry_name: String) -> Result<bool, MyError> {
        let registry = &ctx.accounts.registry;
        let owner = &ctx.accounts.owner;
        // owner.key() retorna un Pubkey
        if registry.owner_id == owner.key() {
            Ok(true)
        } else {
            Err(MyError::NotAllowed) // Define un error adecuado si el propietario no coincide
        }
    }

    fn validate_exists_registry(&self, ctx: Context<ValidateExistsRegistry>, registry_name: String) -> Result<bool, MyError> {
        let registry = &ctx.accounts.registry;
        if registry.name.is_empty() {
            Err(MyError::RegistryNotFound) // Error personalizado
        } else {
            Ok(true)
        }
    }

    fn validate_registry(&self, ctx: &Context<ValidateRegistry>, registry_name: &str) -> Result<bool, MyError> {
        let registry = &ctx.accounts.registry; // Accede a la cuenta del registro
        Ok(registry.name == registry_name)
    }

    fn validate_exists_device(&self, ctx: &Context<ValidateRegistry>, device_name: &str) -> Result<bool, MyError> {
        let registry = &ctx.accounts.registry;
        if registry.devices.contains_key(device_name) {
            Ok(true)
        } else {
            Err(MyError::DeviceNotFound) //Error personalizado si el dispositivo no existe
        }
    }

}
    // Definiciones de cuentas
#[derive(Accounts)]
pub struct ValidateOwner<'info> {
    pub registry: Account<'info, Registry>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ValidateExistsRegistry<'info> {
    pub registry: Account<'info, Registry>,
}

#[derive(Accounts)]
pub struct ValidateRegistry<'info> {
    pub registry: Account<'info, Registry>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ValidateExistsDevice<'info> {
    pub registry: Account<'info, Registry>,
    }



// Entrypoint
entrypoint!(process_instruction);
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
