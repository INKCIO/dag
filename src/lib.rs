#[macro_use]
extern crate log;
#[macro_use]
extern crate may;
extern crate num_cpus;
extern crate rusqlite;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate tungstenite;
extern crate url;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate base32;
extern crate base64;
extern crate bit_vec;
extern crate may_actor;
extern crate may_waiter;
extern crate rand;
extern crate ripemd160;
extern crate secp256k1;
extern crate sha1;
extern crate sha2;

macro_rules! some_if {
    ($condition:expr, $some:expr) => {{
        match $condition {
            true => Some($some),
            _ => None,
        }
    }};
}

pub mod config;
pub mod db;
#[macro_use]
pub mod error;
pub mod graph;
pub mod header_commissions;
pub mod mc_outputs;
pub mod my_witness;
pub mod network;
pub mod spec;

pub mod catchup;
mod definition;
pub mod joint;
mod obj_ser;
pub mod object_hash;
pub mod signature;
mod storage;
pub mod validation;
pub mod witness_proof;

pub use error::{Result, INKCError};
