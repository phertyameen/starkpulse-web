#![no_std]

use soroban_sdk::{contracttype, contractclient, Address, Env, Symbol, Bytes};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Notification {
    pub source: Address,
    pub event_type: Symbol,
    pub data: Bytes,
}

#[contractclient(name = "NotificationReceiverClient")]
pub trait NotificationReceiverTrait {
    fn on_notify(env: Env, notification: Notification);
}
