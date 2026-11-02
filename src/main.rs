use std::env;
use actix_web::{get, HttpServer, App, Responder, HttpResponse, web};
use dotenv::dotenv;

struct StateInfo {
    pub _state_id: i32,
    pub _population: String,
    pub _name: String,
    pub _capital_name: String,
}

#[get("/Name")]
async fn get_name() -> impl Responder {
    dotenv().ok();
    let e_args = env::var("NAME");
    let s = e_args.clone();

    let pop = env::var("POPULATION");


    let data_state_1 = &StateInfo {
        _state_id: 2,
        _population: pop.unwrap(),
        _name: s.unwrap().to_string(),
        _capital_name: String::from("Concord"),
    };

    HttpResponse::Ok().body(String::from(&data_state_1._name.to_uppercase()))
}

#[get("/")]
async fn main_route() -> impl Responder {
    dotenv().ok();

    let e_args = env::var("STATE");

    match e_args {
        Ok(val) => println!("{}", val),
        Err(e) => println!("{}", e),
    };

    let e_pop = env::var("POPULATION");

    let data_state = &StateInfo {
        _state_id: 1,
        _population: e_pop.unwrap(),
        _name: String::from(""),
        _capital_name: String::from("Concord"),
    };

    HttpResponse::Ok().body(String::from(&data_state._capital_name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(main_route).route("/", web::get()).service(get_name).route("/Name", web::get())
    }).bind(("127.0.0.1", 3000))?.run().await
}