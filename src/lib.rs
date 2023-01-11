#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod models;

pub mod client;
pub mod error;

pub mod playlists;
pub mod resolve;
pub mod tips;
pub mod tracks;
pub mod users;
