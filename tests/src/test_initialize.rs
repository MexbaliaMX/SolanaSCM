#[cfg(test)] // Indica que este módulo solo se compilará en modo de prueba.
mod tests {
    use super::*; // Importa todo el contenido del módulo superior (el módulo principal).
    use anchor_lang::prelude::*; // Importa las utilidades de Anchor.
    use anchor_lang::Key; // Importa el trait Key para trabajar con claves públicas.
    use solana_program::pubkey::Pubkey; // Importa la estructura Pubkey de la biblioteca de Solana.
    use solana_program_test::*; // Importa utilidades para probar programas en Solana.
    use std::str::FromStr; // Importa la trait FromStr para convertir cadenas a otros tipos.
    use solana_program::account_info::AccountInfo; // Importa AccountInfo para representar cuentas en la prueba.

    // Función principal de prueba que se ejecutará de forma asíncrona.
    #[tokio::test] 
    async fn test_create_registry_and_add_device() {
        // Define el ID del programa en la blockchain (usado para instanciar el programa).
        let program_id = Pubkey::from_str("A5i8uPKdCycDG3nbGCCAUiLzHEc4ddpfeYGQhPEWuaTJ").unwrap();

        // Inicializa el contexto de prueba con el nombre del proyecto y el ID del programa.
        let mut context = ProgramTest::new(
            "registry_project", 
            program_id,
            processor!(process_instruction), // Indica la función que maneja las instrucciones del programa.
        );

        // Crea un par de claves para simular un usuario (un firmante de la transacción).
        let (user_keypair, user) = create_user(); // Crea un usuario

        // Crea la cuenta de usuario como AccountInfo para la prueba.
        let user_account_info = AccountInfo::new(
            &user_keypair.pubkey(), // La clave pública del usuario
            false, // Indica si la cuenta puede ser firmada (no puede, porque es solo una cuenta de prueba)
            true,  // Indica si la cuenta es editable
            &mut 1_000_000_000, // Cantidad de lamports (la unidad de moneda de Solana)
            &mut vec![0; 0], // Datos iniciales (no se necesitan datos aquí)
            &system_program::id(), // Propietario de la cuenta (ID del programa del sistema de Solana)
            false, // Indica que la cuenta no es ejecutable
            0 // Época de alquiler (para el manejo del alquiler de cuentas)
        );

        // Agrega la cuenta de usuario al contexto de prueba.
        context.add_account(
            user_keypair.pubkey(), // Clave pública del usuario
            user_account_info, // Información de la cuenta del usuario
        );

        // Crea la cuenta del registro (registro de dispositivos).
        let (registry_keypair, registry) = create_registry_account(); // Crea la cuenta del registro

        // Simula el contexto de ejecución del programa.
        let (mut banks_client, payer, recent_blockhash) = context.start_with_context().await;

        // Crea la instrucción para crear un nuevo registro.
        let create_registry_instruction = registry_project::instructions::create_registry(
            CreateRegistry {
                registry: registry.clone(), // Clona la cuenta del registro
                user: user.clone(), // Clona la cuenta del usuario
                system_program: system_program::id(), // ID del programa del sistema
            },
            "Registro 1".to_string(), // Nombre del registro que se está creando
        );

        // Ejecuta la instrucción para crear el registro.
        let _ = banks_client.process_transaction(create_registry_instruction).await.unwrap();

        // Agrega un dispositivo al registro.
        let (device_keypair, device) = create_device_account(); // Crea la cuenta del dispositivo

        // Crea la instrucción para agregar un dispositivo al registro.
        let add_device_instruction = registry_project::instructions::add_device(
            AddDevice {
                device: device.clone(), // Clona la cuenta del dispositivo
                registry: registry.clone(), // Clona la cuenta del registro
                user: user.clone(), // Clona la cuenta del usuario
                system_program: system_program::id(), // ID del programa del sistema
            },
            "Lampara Google IoT".to_string(), // Nombre del dispositivo que se está agregando
            "Lampara de Google de IoT".to_string(), // Descripción del dispositivo
            vec![("key".to_string(), "value".to_string())], // Metadata del dispositivo
            vec![("data_key".to_string(), "data_value".to_string())], // Datos adicionales del dispositivo
        );

        // Ejecuta la instrucción para agregar el dispositivo.
        let _ = banks_client.process_transaction(add_device_instruction).await.unwrap();

        // Aquí puedes agregar verificaciones para confirmar que los registros y dispositivos se han agregado correctamente.
    }

    // Función para crear un usuario, retorna el par de claves y la clave pública.
    fn create_user() -> (Keypair, Pubkey) {
        let keypair = Keypair::new(); // Genera un nuevo par de claves
        let pubkey = keypair.pubkey(); // Obtiene la clave pública
        (keypair, pubkey) // Retorna el par de claves y la clave pública
    }

    // Función para crear una cuenta de registro, retorna el par de claves y la clave pública.
    fn create_registry_account() -> (Keypair, Pubkey) {
        let keypair = Keypair::new(); // Genera un nuevo par de claves
        let pubkey = keypair.pubkey(); // Obtiene la clave pública
        (keypair, pubkey) // Retorna el par de claves y la clave pública
    }

    // Función para crear una cuenta de dispositivo, retorna el par de claves y la clave pública.
    fn create_device_account() -> (Keypair, Pubkey) {
        let keypair = Keypair::new(); // Genera un nuevo par de claves
        let pubkey = keypair.pubkey(); // Obtiene la clave pública
        (keypair, pubkey) // Retorna el par de claves y la clave pública
    }
}
