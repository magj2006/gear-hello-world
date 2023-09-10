#![no_std]
use gstd::{debug, msg, prelude::*};

#[no_mangle]
extern "C" fn handle() {
    msg::reply("Hello", 0).expect("Error in sending a reply message");
    debug!("Program replied \"Hello\" to incoming message")
}

#[no_mangle]
extern "C" fn init() {
    let init_message: String = msg::load().expect("Can't load init message");
    debug!("Program was initialized with message {:?}", init_message);
}
