use orouter_wireless::{MessageSlicer, MessageType, network};
use std::io::{self, Write};
use hex;
use std::convert::TryInto;

fn main() -> Result<(), String> {
    loop {
        // Enter the Seed value
        print!("Enter the Seed value (type 'exit' to quit): ");
        io::stdout().flush().unwrap();
        let mut seed_str = String::new();
        io::stdin().read_line(&mut seed_str).unwrap();

        if seed_str.trim().eq("exit") {
            break;
        }

        let seed = seed_str.trim().parse::<u64>().map_err(|e| e.to_string())?;

        // Enter the Network value (e.g., AACC)
        print!("Enter the Network value (e.g., AACC): ");
        io::stdout().flush().unwrap();
        let mut network_str = String::new();
        io::stdin().read_line(&mut network_str).unwrap();
        let network: [u8; 2] = hex::decode(network_str.trim())
            .map_err(|e| e.to_string())?
            .try_into()
            .map_err(|_| "Invalid Network value".to_string())?;

        // Enter the data
        print!("Enter the message: ");
        io::stdout().flush().unwrap();
        let mut data = String::new();
        io::stdin().read_line(&mut data).unwrap();

        // Message slicing
        let mut slicer = MessageSlicer::new(seed, network);
        let messages = slicer
            .slice(data.trim().as_bytes(), MessageType::Data, 0x01)
            .map_err(|e| format!("Slicing failed: {:?}", e))?;

        // Display sliced messages
        println!("Sliced messages:");
        for (i, message) in messages.iter().enumerate() {
            println!("{}: {:02x?}", i, message);
        }
    }

    Ok(())
}
