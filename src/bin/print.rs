use orouter_wireless::{MessagePool, WirelessMessagePart};
use std::io::{self, Write};
use hex;

fn main() -> Result<(), String> {
    loop {
        let mut message_pool = MessagePool::default();
        let mut complete_message_received = false;

        while !complete_message_received {
            // Receive input for sliced message parts
            print!("Enter sliced message part (type 'exit' to quit): ");
            io::stdout().flush().unwrap();
            let mut message_part_hex = String::new();
            io::stdin().read_line(&mut message_part_hex).unwrap();

            if message_part_hex.trim().eq("exit") {
                return Ok(());
            }

            // Remove commas, spaces, carriage returns, and newlines
            let message_part_hex = message_part_hex.replace(",", "")
                                                  .replace(" ", "")
                                                  .replace("\r", "")
                                                  .replace("\n", "");

            // Convert the input hexadecimal string into a byte array
            let message_part = hex::decode(&message_part_hex)
                .map_err(|e| e.to_string())?;

            // Insert the message part into the MessagePool and attempt reassembly
            match message_pool.try_insert(message_part.into()) {
                Ok(Some(complete_message)) => {
                    println!("Reassembled message: {}", String::from_utf8_lossy(&complete_message.data()));
                    complete_message_received = true;
                    break;
                },
                Ok(None) => println!("More message parts are needed..."),
                Err(e) => println!("Error: {:?}", e),
            }
        }

        // After displaying the reassembled message, wait for the next input
        println!("Processing the next message...");
    }
}
