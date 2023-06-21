#![no_std]
use gstd::{msg, prelude::*, debug,ActorId};

#[derive(Encode, Decode, TypeInfo)]
pub enum InputMessages {
    SendHelloTo(ActorId),
    SendHelloReply,
}

static mut GREETING: Option<String> = None;

#[no_mangle]
// It returns the hash of metadata.
// .metahash is generating automatically
// while you are using build.rs
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!("../.metahash");
    msg::reply(metahash, 0)
        .expect("Failed to share metahash");
}

#[no_mangle]
extern "C" fn state() {
    let greeting = unsafe {
        GREETING
            .as_ref()
            .expect("The contract is not initialized")
    };
    msg::reply(greeting, 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn handle() {
    // msg::reply(String::from("Hello"), 0)
    //     .expect("Error in sending a reply message");
    let input_message: InputMessages = msg::load()
        .expect("Error in loading InputMessages");
    let greeting = unsafe {
        GREETING
            .as_mut()
            .expect("The contract is not initialized")
    };
    match input_message {
        InputMessages::SendHelloTo(account) => {
            debug!("Message: SendHelloTo {:?}", account);
            msg::send(account, greeting, 0)
                .expect("Error in sending Hello message to account");
        }
        InputMessages::SendHelloReply => {
            debug!("Message: SendHelloReply");
            msg::reply(greeting, 0)
                .expect("Error in sending reply");
        }
    }
}

#[no_mangle]
extern "C" fn init() {
    let init_message: String = msg::load()
        .expect("Can't load init message");
    debug!("Program was initialized with message {:?}",
        init_message);
    unsafe { GREETING = Some(init_message) };
}

// #[no_mangle]
// unsafe extern "C" fn init() {
//    let greeting = String::from_utf8(msg::load_bytes().expect("Can't load init message"))
//        .expect("Invalid message");
//    debug!("Program was initialized with message {:?}", greeting);
//    GREETING = Some(greeting);
// }

// #[no_mangle]
// unsafe extern "C" fn handle() {
//    let input_message: InputMessages = msg::load().expect("Error in loading InputMessages");
//    let greeting = GREETING.get_or_insert(Default::default());
//    match input_message {
//        InputMessages::SendHelloTo(account) => {
//            debug!("Message: SendHelloTo {:?}", account);
//            msg::send(account, greeting, 0)
//                .expect("Error in sending Hello message to account");
//        }
//        InputMessages::SendHelloReply => {
//            debug!("Message: SendHelloReply");
//            msg::reply(greeting, 0).expect("Error in sending reply");
//        }
//    }
// }