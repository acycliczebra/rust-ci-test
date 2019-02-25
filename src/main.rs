// Crates
//extern crate actix;
//extern crate actix_web;
//extern crate argonautica;
//extern crate base64;
//extern crate bcrypt;
//extern crate bigdecimal;
//extern crate byteorder;
//extern crate bytes;
//extern crate chrono;
//extern crate dirs;
//extern crate env_logger;
//#[macro_use]
//extern crate failure;
//extern crate futures;
//extern crate json;
//extern crate jsonwebtoken;
//extern crate linked_hash_map;
//#[macro_use]
//extern crate log;
//extern crate num_cpus;
//extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
//extern crate openssl;
//extern crate tempfile;
//#[macro_use]
//extern crate time_test;
//extern crate tokio;
//extern crate tokio_core;
//extern crate uuid;
//#[macro_use]
//extern crate diesel;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Defaults {
    Table(String),
    Query(String),
    Script(String),
    //TODO: view
    TableData(String), //TODO: this is tricky since the filter / query can go in as well
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Channels {
    Defaults(Defaults),
    Subscribers(Defaults),
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_channel() {
        let channel = Channels::Defaults(Defaults::Table("test".to_string()));

        let repr = serde_json::to_value(&channel).unwrap();

        println!("repr: {:?}", &repr);
    }
}

