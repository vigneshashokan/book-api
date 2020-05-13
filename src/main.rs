#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]
// #![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use routes::*;
use std::env;

mod database;
mod models;
mod routes;
mod schema;
mod static_files;

fn rocket() -> rocket::Rocket {
    // Check if the .env file is present
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = database::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount(
            "/api/v1/",
            routes![index, new, show, delete, author, update],
        )
        .mount("/", routes![static_files::all, static_files::index])
}

fn main() {
    rocket().launch();
    // // Check if the .env file is present
    // dotenv().ok();

    // let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    // let conn = PgConnection::establish(&database_url).unwrap();

    // let book = models::NewBook {
    //     title: String::from("Gravity's Rainbow"),
    //     author: String::from("Thomas Pynchon"),
    //     published: true,
    // };

    // if models::Book::insert(book, &conn) {
    //     println!("success!");
    // } else {
    //     println!("failed");
    // }
}
