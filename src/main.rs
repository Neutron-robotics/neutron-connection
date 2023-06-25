pub mod network;

// #![deny(warnings)]

use network::ws_proxy::{make_ws_context};

#[tokio::main]
async fn main() {
    make_ws_context().await;
}

// extern crate dotenv;
// // use database::redis::get_connection;
// use dotenv::dotenv;
// use utils::args::Args;

// pub mod database;
// pub mod utils;

// use clap::Parser;

// fn main() {
//     dotenv().ok();

//     let args = Args::parse();
//     utils::args::print_args(args);

//     // let mut conn = get_connection();

//     // let _: () = redis::cmd("SET")
//     //     .arg("foo")
//     //     .arg("bar")
//     //     .query(&mut conn)
//     //     .expect("failed to execute SET for 'foo'");

//     // let bar: String = redis::cmd("GET")
//     //     .arg("foo")
//     //     .query(&mut conn)
//     //     .expect("failed to execute GET for 'foo'");

//     // println!("value for 'foo' = {}", bar);
// }
