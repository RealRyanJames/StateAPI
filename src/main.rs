use crate::routes_visit::Routes;
use crate::{_population::POPULATION, picked_state::STATE};
use STATE::_NH;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use dotenv::dotenv;
use std::env;

pub mod _population {
    pub struct POPULATION {
        pub _pop: i32,
    }
}

impl _population::POPULATION {
    pub fn new(&self) -> Self {
        Self {
            _pop: 1_415_342 as i32,
        }
    }
}

mod states {
    pub struct StateInfo {
        pub _state_id: i32,
        pub _population: String,
        pub _name: String,
        pub _capital_name: String,
    }
}

mod picked_state {
    pub enum STATE {
        _NH,
        _MA,
        _NJ,
        _MAINE,
    }

    impl STATE {
        pub fn get(&self) -> String {
            match self {
                STATE::_NH => {
                    println!("{}", "Welcome to NH Info");
                    format!("{}", String::from("nh".to_uppercase()))
                }
                STATE::_MA => {
                    println!("{}", "Welcome to MA Info");
                    format!("{}", "ma".to_uppercase())
                }

                STATE::_NJ => {
                    println!("{}", "Welcome to NJ Info");
                    format!("{}", "nj".to_uppercase())
                }

                STATE::_MAINE => {
                    println!("{}", "Welcome to MAINE Info");
                    format!("{}", "maine".to_uppercase())
                }
            }
        }
    }
}

#[get("/Name")]
async fn get_name() -> impl Responder {
    dotenv().ok();
    let e_args = env::var("NAME");
    let s = e_args.clone();
    let pop = env::var("POPULATION");

    let data_state_1 = &states::StateInfo {
        _state_id: 2,
        _population: pop.unwrap(),
        _name: s.unwrap().to_string(),
        _capital_name: String::from("Concord"),
    };

    HttpResponse::Ok().body(String::from(&data_state_1._name.to_uppercase()))
}

mod routes_visit {
    pub enum Routes {
        HOME,
        NAMES,
    }
}

fn get_home(routes: Routes) -> String {
    let s = String::from("/");
    let name = String::from("/Name");
    match routes {
        Routes::HOME => {
            println!("{:?}", _NH.get());
            s
        }

        Routes::NAMES => name,
    }
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

    let data_state = &states::StateInfo {
        _state_id: 1,
        _population: e_pop.unwrap(),
        _name: String::from(""),
        _capital_name: String::from("Concord"),
    };

    println!("{}", "Connected To Port".to_uppercase());

    HttpResponse::Ok().body(String::from(&data_state._capital_name))
}

#[get("/NH:name")]
async fn get_pop() -> impl Responder {
    let state: STATE = STATE::_NH;
    HttpResponse::Ok().body(format!("State: {:?}", state.get()))
}

#[get("/NH:pop")]
async fn get_nh() -> impl Responder {
    let state: STATE = STATE::_NH;

    let pop_nh: _population::POPULATION = POPULATION {
        _pop: 1_415_342 as i32,
    };

    let pop = _population::POPULATION::new(&pop_nh);

    HttpResponse::Ok().body(format!(
        "State: {}, Population: {}",
        state.get(),
        pop.new()._pop
    ))
}

async fn get_setup() -> std::io::Result<()> {
    's: loop {
        HttpServer::new(|| {
            App::new()
                .service(get_nh)
                .route("/NH:pop", web::get())
                .service(main_route)
                .route(get_home(Routes::HOME).as_str(), web::get())
                .service(get_name)
                .route(get_home(Routes::NAMES).as_str(), web::get())
        })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
        .expect("No Port Found");
        break 's;
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    get_setup().await
}
