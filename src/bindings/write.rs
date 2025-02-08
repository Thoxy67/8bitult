use std::error::Error;
use tokio::time::Duration;

use btleplug::api::{Characteristic, Peripheral};

use crate::bindings::{
    commands::SET_BINDINGS_COMMANDS, types::KEY_BINDINGS, utils::create_binding_command,
};

pub async fn write_new_bindings(
    peripheral: &btleplug::platform::Peripheral,
    characteristic: &Characteristic,
) -> Result<(), Box<dyn Error>> {
    println!("\n--- Modification des bindings ---");

    // Première commande (touches 1-8)
    let cmd1 = create_binding_command(&SET_BINDINGS_COMMANDS[0], &KEY_BINDINGS[0..8], 29, 61);
    for retry in 0..3 {
        println!("Envoi commande 1 pour modification (touches 1-8)");
        match peripheral
            .write(
                &characteristic,
                &cmd1,
                btleplug::api::WriteType::WithResponse,
            )
            .await
        {
            Ok(_) => break,
            Err(e) => {
                if retry == 2 {
                    return Err(e.into());
                }
                println!("Retry {} - Write error: {}", retry + 1, e);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    // Deuxième commande (touches 9-16)
    let cmd2 = create_binding_command(&SET_BINDINGS_COMMANDS[1], &KEY_BINDINGS[8..16], 24, 56);
    for retry in 0..3 {
        println!("Envoi commande 2 pour modification (touches 9-16)");
        match peripheral
            .write(
                &characteristic,
                &cmd2,
                btleplug::api::WriteType::WithResponse,
            )
            .await
        {
            Ok(_) => break,
            Err(e) => {
                if retry == 2 {
                    return Err(e.into());
                }
                println!("Retry {} - Write error: {}", retry + 1, e);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }

    // Commandes finales
    println!("Envoi des commandes finales");
    for cmd in &SET_BINDINGS_COMMANDS[2..] {
        for retry in 0..3 {
            match peripheral
                .write(&characteristic, cmd, btleplug::api::WriteType::WithResponse)
                .await
            {
                Ok(_) => break,
                Err(e) => {
                    if retry == 2 {
                        return Err(e.into());
                    }
                    println!("Retry {} - Write error: {}", retry + 1, e);
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }
    }
    Ok(())
}
