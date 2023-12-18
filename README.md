# `orouter-wireless` (ōRouter wireless protocol)
----------------------------------------------------------------------------------------
I'm not entirely sure if everything I've put together is technically accurate, as my understanding is limited.

However, I personally find this project file to be incredibly cool, and it excites me once again.

Although I only understood about 5% of it and couldn't utilize many features, I used basic concepts to build it.

In the exe folder, slicer.exe serves to split data, while print.exe reassembles the divided data, even combining chunks in different orders.

Inside the bin folder, slicer.rs contains an example code for splitting, and print.rs holds a simple example for merging.

Theoretically, systems like MessagePool could be used to split and merge larger data files, such as images or short videos.

While writing the code, I sought help not from the somewhat silly ChatGPT 3.5, but from Naver Clova X.


------------------------------------------------------------------------------------------
Defines and implements protocol used on oRouter's physical radio layer (now using LoRa)

Application level message can be theoretically unlimited, but LoRa can only transmit 255B in
one message. This protocol takes care of splitting message to appropriate number of parts with
necessary added information allowing in to be joined back on receiving end when all parts
arrive.

[`crate::MessageSlicer`] takes care of the splitting part and is used before the data is
transmitted using oRouter. [`crate::MessagePool`] is used on receiving end to put parts of the
application level / logical message together to form the original message back. Note that the
parts don't have to arrive in order, only that all parts of the message have to arrive
eventually.

[`crate::WirelessMessagePart`] represents raw chunk of data transmitted/received using oRouters
radio chip. This crate implements and uses following scheme for message part:

| name          | length in bytes | description                                                                               |
|---------------|-----------------|-------------------------------------------------------------------------------------------|
| network bytes | 2               | network bytes. always 0xAA 0xCC (will be configurable in next release)                      |
| hash          | 6               | hash - first 3B are random, second 3B form a prefix grouping parts of the message to one  |
| part_num      | 1               | part number 1, 2 or 3 (only 3-part messages supported)                                    |
| total_count   | 1               | total count of messages with this prefix                                                  |
| length        | 1               | length of data                                                                            |
| msg type      | 1               | overline message type                                                                     |
| data type     | 1               | byte identifying data type, if previous field is data                                     |
| data          | 1 - 240         | actual data                                                                               |
| CRC16         | 2               | CRC16 of the whole message (header + data)                                                |

Example of using a [`crate::MessageSlicer`] to split some data for wireless transmission:

```rust
use orouter_wireless::{MessageSlicer, MessageType, network};

fn main() {
    // VVV in practice provide a good random seed here VVV
    let mut slicer = orouter_wireless::MessageSlicer::new(1234u64, network::DEFAULT);
    let messages = slicer
        .slice(&[0xc0, 0xff, 0xee], MessageType::Data, 0x01).unwrap();
    println!("slices = {:?}", messages);
}
```

Example of using a [`crate::MessagePool`] to assemble data back from received message parts:

```rust
use orouter_wireless::MessagePool;

fn main() {
    let mut message_pool = MessagePool::default();
    // this represents a message part received from oRouter
    //
    // in this example, there is 1 part of total 1 forming the whole message, because the data
    // contained in the message are short
    for part in vec![
        vec![
            0xaa, 0xcc, 0x1b, 0xf2, 0x73, 0x86, 0x80, 0xe1, 0x01, 0x01,
            0x05, 0x01, 0x01, 0x41, 0x48, 0x4f, 0x59, 0x21, 0x53, 0xef
        ]
    ] {
        match message_pool.try_insert(part.clone()) {
            Ok(Some(message)) => assert_eq!(message.data(), b"AHOY!"),
            Ok(None) => {}
            Err(_) => {
                eprintln!(
                    "error while trying to insert message = {:02x?}",
                    part
                )
            }
        }
    }
}
```
-------------------------------------------------------------------------

Learning Rust can be particularly challenging for those who are not native English speakers.

I am grateful for the resources provided by Mithradates' YouTube channel [https://www.youtube.com/@mithradates]

and The Rust Programming Language official documentation [https://doc.rust-lang.org/book/title-page.html]

, which have significantly helped in breaking down these barriers.
