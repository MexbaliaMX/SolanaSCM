
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct Device {
    pub name: String,
    pub description: String,
    pub metadata: Vec<(String, String)>, // Utilizamos un vector en lugar de UnorderedMap
    pub data: Vec<(String, String)>,
}

impl Device {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            metadata: Vec::new(), // Inicializamos como un vector vacío
            data: Vec::new(),
        }
    }

    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.push((key, value));
    }

    pub fn get_metadata(&self) -> Vec<(String, String)> {
        self.metadata.clone()
    }

    pub fn set_data(&mut self, key: String, value: String) {
        self.data.push((key, value));
    }

    pub fn get_data(&self) -> Vec<(String, String)> {
        self.data.clone()
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // Decodificar los datos del dispositivo desde la cuenta
    let mut device = Device::try_from_slice(&account.data.borrow())?;

    // Log para depuración
    msg!("Dispositivo: {}", device.name);
    
    // Manipular los datos del dispositivo aquí
    // Por ejemplo, agregar metadata
    device.set_metadata("version".to_string(), "1.0".to_string());

    // Serializar los cambios de vuelta a la cuenta
    device.serialize(&mut *account.data.borrow_mut())?;

    Ok(())
}
