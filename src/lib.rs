#![no_std]
use gstd::{debug, msg, prelude::*, ActorId};

static mut GREETING: Option<String> = None;

#[derive(Encode, Decode, TypeInfo)]
pub enum InputMessages {
    SendHelloTo(ActorId),
    SendHelloReply,
}

#[no_mangle]
extern "C" fn handle() {
    let input_message: InputMessages = msg::load().expect("Error in loading InputMessages");
    let greeting = unsafe { GREETING.get_or_insert(Default::default()) };
    match input_message {
        InputMessages::SendHelloTo(account) => {
            debug!("Message: SendHelloTo {:?}", account);
            msg::send(account, greeting, 0).expect("Error in sending Hello message to account");
        }
        InputMessages::SendHelloReply => {
            debug!("Message: SendHelloReply");
            msg::reply(greeting, 0).expect("Error in sending reply");
        }
    }
}

#[no_mangle]
extern "C" fn init() {
    let greeting: String = msg::load().expect("Can't load init message");
    debug!("Program was initialized with message {:?}", greeting);
    unsafe { GREETING = Some(greeting) };
}
