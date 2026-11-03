use crate::routes_visit::Routes;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use dotenv::dotenv;

use std::env;

pub mod _population {
    pub struct NHPOPULATION {
        pub _pop: i32,
    }

    pub struct MAPOPULATION {
        pub _pop: i32,
    }

    pub struct NJPOPULATION {
        pub _pop: i32,
    }
}

impl _population::NHPOPULATION {
    pub fn new(&self) -> Self {
        Self {
            _pop: 1_415_342 as i32,
        }
    }
}

impl _population::MAPOPULATION {
    pub fn new(&self) -> Self {
        Self {
            _pop: 7_154_084 as i32,
        }
    }
}

impl _population::NJPOPULATION {
    pub fn new(&self) -> Self {
        Self {
            _pop: 9_548_215 as i32,
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
            println!("{:?}", picked_state::STATE::_NH.get());
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

/////////////////////////// NEW Jersery ////////////////////////////
#[get("/NJ")]
async fn get_nj() -> impl Responder {
    let state_nj = picked_state::STATE::_NJ;

    let state_new_j = _population::NJPOPULATION { _pop: 9_548_215 };

    let state_name = state_nj.get();
    let pop = state_new_j.new()._pop;
    HttpResponse::Ok().body(format!("State Name: {}, Population: {}", state_name, pop))
}

/////////////////////////// NEW HAMPSHIRE ////////////////////////////

#[get("/NH/")]
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

#[get("/NH/pop")]
async fn get_nh() -> impl Responder {
    let state: picked_state::STATE = picked_state::STATE::_NH;

    let pop_nh: _population::NHPOPULATION = _population::NHPOPULATION {
        _pop: 1_415_342 as i32,
    };

    let pop = _population::NHPOPULATION::new(&pop_nh);

    HttpResponse::Ok().body(format!(
        "State: {}, Population: {}",
        state.get(),
        pop.new()._pop
    ))
}

/////////////////////////// MASSACHUSETTS ////////////////////////////

#[get("/MA/")]
async fn get_ma() -> impl Responder {
    let state_ma = picked_state::STATE::_MA;

    let pop_ma: _population::MAPOPULATION = _population::MAPOPULATION {
        _pop: 7_154_084 as i32,
    };

    let _client: _connection::_ClientInfo = _connection::_ClientInfo {
        uri: String::from("mongodb://localhost:27017"),
        _len: 100,
        database_name: String::from("States"),
        coll_name: String::from("State"),
    };

    let num = pop_ma.new()._pop.to_string();
    let _ = _connection::connect_client(num).await;

    HttpResponse::Ok().body(format!(
        "State: {}, Population: {}",
        state_ma.get(),
        pop_ma.new()._pop
    ))
}

mod _connection {
    use mongodb::{
        Client, Collection,
        bson::{Document, doc},
        options::{ClientOptions, ServerApi, ServerApiVersion},
    };

    pub trait _DataInfoDB {
        fn _get_uri(&self) -> String;
        fn _get_uri_len(&self) -> usize;
        fn _connect(&self) -> String;
        fn _is_connect(&self) -> bool;
    }

    pub struct _ClientInfo {
        pub uri: String,
        pub _len: u64,
        pub database_name: String,
        pub coll_name: String,
    }

    impl _DataInfoDB for _ClientInfo {
        fn _get_uri(&self) -> String {
            format!("{}", self._connect())
        }

        fn _get_uri_len(&self) -> usize {
            self.uri.len()
        }

        fn _connect(&self) -> String {
            self.uri.to_lowercase()
        }

        fn _is_connect(&self) -> bool {
            self.uri.len() > 1
        }
    }

    pub async fn connect_client(data: String) -> mongodb::error::Result<()> {
        let client: _ClientInfo = _ClientInfo {
            uri: String::from("mongodb://localhost:27017"),
            _len: 100,
            database_name: String::from("States"),
            coll_name: String::from("State"),
        };

        if client._get_uri().ends_with("localhost:27017") {
            let mut client_options = ClientOptions::parse(client.uri).await?;
            let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
            client_options.server_api = Some(server_api);

            let cl = Client::with_options(client_options)?;

            let db = cl.database(&client.database_name);

            let db_collection: Collection<Document> = db.collection(&client.coll_name);

            let _ = db_collection.insert_one(doc! {"state": data}).await;
        }

        Ok(())
    }
}

async fn get_setup() -> std::io::Result<()> {
    's: loop {
        HttpServer::new(|| {
            App::new()
                .service(get_nj)
                .route("/NJ/", web::get())
                .service(get_ma)
                .route("/MA/", web::get())
                .service(get_nh)
                .route("/NH/pop", web::get())
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
