use clap::Parser;
use rust_cli::commands::{self, Args, Commands};
use rust_cli::file_ops;
use rust_cli::wallet::Wallet;

fn main() {
    let args = Args::parse();
    let mut wallet = Wallet::new();
    wallet
        .load_wallet(String::from("wallet"))
        .unwrap_or_else(|_| {
            panic!("Error loading wallet");
        });

    match args.command {
        Commands::Generate { name, cs_id } => {
            let result = wallet.create_ciphersuite(name, cs_id);
            match result {
                Ok(_) => {
                    println!("Persona created successfully");
                }
                Err(e) => {
                    println!("Error creating persona {}", e);
                }
            }
        }
        Commands::Remove { name } => {
            let result = wallet.remove_ciphersuite(&name);
            match result {
                Ok(_) => {
                    println!("Persona removed successfully");
                }
                Err(e) => {
                    println!("Error removing persona: {}", e);
                }
            }
        }
        Commands::Sign { name, file, output } => {
            let cipher_suite = wallet.get_ciphersuite(&name).unwrap();
            let result = cipher_suite.sign(&file, &output);
            match result {
                Ok(_) => {
                    println!("Signature created successfully.");
                }
                Err(e) => println!("Error signing file: {}", e),
            }
        }
        Commands::Verify { name, header } => {
            let cipher_suite = wallet.get_ciphersuite(&name).unwrap();
            let result = cipher_suite.verify(&header);
            match result {
                Ok(_) => println!("Verification successful."),
                Err(e) => println!("Verification failed: {}", e),
            }
        }
        Commands::RemoveSignature { file } => {
            // Directly pass the signature file path to the verify function
            let result = file_ops::remove_signature(&file);
            match result {
                Ok(_) => println!("Removal successful."),
                Err(e) => println!("Removal failed: {}", e),
            }
        }
        Commands::ListSignatures => {
            if let Err(e) = file_ops::list_signature_files() {
                println!("Failed to list signature files: {}", e);
            }
        }
        Commands::ListFiles => {
            if let Err(e) = file_ops::list_files() {
                println!("Failed to list signature files: {}", e);
            }
        }
        Commands::Algorithms => {
            commands::print_ids();
        }
    }
}
