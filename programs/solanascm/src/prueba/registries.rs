use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program::{invoke_signed},
    system_instruction,
    system_program,
};
use crate::devices::Device; //Importamos para poder usar

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Registry {
    pub device_count: u64,        // Lleva un conteo del número de dispositivos
    pub device_ids: Vec<Pubkey>,  // Guarda las claves públicas (cuentas) de los dispositivos registrados
}

impl Registry {
    pub fn new() -> Self {
        Self {
            device_count: 0,
            device_ids: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_pubkey: Pubkey) {
        self.device_ids.push(device_pubkey);
        self.device_count += 1;
    }

    pub fn create_device_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    description: String,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?; // El que paga por la cuenta de dispositivo
    let system_program = next_account_info(accounts_iter)?;
    let registry_account = next_account_info(accounts_iter)?; // Cuenta de registro

    // Generar PDA para el nuevo dispositivo
    let length = registry_account.data.borrow().len() as u8; // Obtén la longitud como u8
    let (device_pubkey, bump_seed) = Pubkey::find_program_address(
    &[
        b"device",
        &registry_account.key.to_bytes(),
        &length.to_le_bytes(), // Convierte a un slice de bytes
    ],
    program_id,
);


    // Crear la instrucción para crear la cuenta del dispositivo
    let rent = Rent::get()?; //Obtener instancia de Rent para acceder a sus metodos - ? Prooaga cualquier error 
    let minimum_balance = rent.minimum_balance(100); // Ajustar el tamaño según tus necesidades

    let create_device_account_instruction = system_instruction::create_account(
        payer.key,
        &device_pubkey,
        minimum_balance.max(1), // Asegúrate de que el valor mínimo sea positivo
        100, // Tamaño de la cuenta (ajustar según necesidad)
        program_id,
);

    invoke_signed(
        &create_device_account_instruction,
        &[payer.clone(), registry_account.clone(), system_program.clone()],
        &[&[b"device", registry_account.key.as_ref(), &[bump_seed]]], // Seeds para la firma
    )?;

    // Inicializar el dispositivo
    let device = Device::new(name, description);
    device.serialize(&mut &mut registry_account.data.borrow_mut()[..])?; // Guardar datos del dispositivo

    Ok(())
}

}

