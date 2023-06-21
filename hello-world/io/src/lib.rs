#![no_std]
use gstd::{prelude::*, ActorId};
pub struct PMetadata;

use gmeta::{In,InOut,Metadata};
impl Metadata for PMetadata{
    type Reply = ();
    type Others = ();
    type Signal = ();
    type Init = In<String>;
    type Handle = InOut<InputMessages,String>;
    type State = String;
}

use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[derive(Encode, Decode, TypeInfo)]
pub enum InputMessages {
    SendHelloTo(ActorId),
    SendHelloReply,
}