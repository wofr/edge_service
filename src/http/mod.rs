use rocket::config::{Config, Environment,ConfigError};
use std::thread;
use rocket::error::LaunchError;
use rocket_contrib::json::{Json};
use rocket::data::Data;

#[derive(Deserialize)]
pub struct DataJson {
    data: String
}

#[post("/upload", format = "plain", data = "<data>")]
fn upload(data: Data) -> Option<String> {
    data.stream_to_file("/tmp/upload.txt").map(|n| n.to_string());
    None
}


#[post("/data", format = "application/json", data = "<jsond>")]
pub fn put_data(jsond: Json<DataJson>) -> Option<String> {
    println!("Put data called {:?}",jsond.data);
    Some(String::from("Data"))
}

pub fn start_http_endpoint(port_http: &Option<u16>)->std::thread::JoinHandle<LaunchError> {
    let http_config = get_rocket_custom_config(*port_http).expect("Custom configuration for Rocket failed");

    // Create an thread which spwans the Http endpoint
    let rocket_err = thread::spawn(move || {
        rocket::custom(http_config)
            .mount("/",routes![put_data,upload]).launch()
    });

    rocket_err
}

pub fn get_rocket_custom_config(port: Option<u16>) ->Result<Config,ConfigError> {
    // using default Production setting, just the port is set individually
    match port {
        Some(p)=> Config::build(Environment::Production).port(p).finalize(),
        None =>  Config::build(Environment::Production).finalize(),
    }
}



#[cfg(test)]
mod test {
    use super::*;
}