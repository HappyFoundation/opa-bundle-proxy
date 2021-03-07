#![feature(proc_macro_hygiene, decl_macro)]

use std::{convert::TryInto, env, error::Error, time::{Duration, SystemTime, UNIX_EPOCH}};

use flate2::{Compression, write::GzEncoder};
use rocket::{Request, Response, fairing::{Fairing, Info, Kind}, http::RawStr};
use rocket_prometheus::PrometheusMetrics;
use tar::{Builder, Header};
use serde::{Deserialize, Serialize};

#[macro_use] extern crate rocket;

// Source: https://stackoverflow.com/a/64904947
struct CORS;
impl Fairing for CORS {
    fn info(&self) -> rocket::fairing::Info {
        Info {
            name: "CORS Headers",
            kind: Kind::Response
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Methods", "GET, OPTIONS"));
        response.set_header(rocket::http::Header::new("Access-Control-Allow-Headers", "*"));
    }
}

#[get("/healthz")]
fn healthz() -> &'static str {
    "ok"
}

#[derive(Serialize, Deserialize)]
struct BundleManifest {
    revision: String,
    roots: Vec<String>,
}

#[derive(Responder)]
enum BundleResponse {
    #[response(status = 200, content_type = "application/gzip")]
    Found(Vec<u8>),
    #[response(status = 404, content_type = "text/plain")]
    NotFound(String),
    #[response(status = 500, content_type = "text/plain")]
    InternalError(String),
}

#[get("/bundles/<bundle>")]
fn bundle<'r>(bundle: &RawStr) -> BundleResponse {
    if !bundle.ends_with(".tar.gz") {
        return BundleResponse::NotFound("Not found".to_string());
    }

    match fetch(&bundle) {
        Ok(result) => BundleResponse::Found(result),
        Err(error) => BundleResponse::InternalError(format!("Internal Error: {}", error)),
    }
}

fn fetch(_bundle: &RawStr) -> Result<Vec<u8>, Box<dyn Error>>{
    let resource_url = &env::var("OBP_RESOURCE_URL")?;
    let resource_path = &env::var("OBP_RESOURCE_PATH")?;
    let resource_type = &env::var("OBP_RESOURCE_TYPE")?;

    let path = format!("{}/data.{}", resource_path, resource_type);
    
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;
    let result = client.get(resource_url)
        .header(reqwest::header::ACCEPT, "application/json")
        .send()?
        .text()?;

    let time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
    
    let manifest = BundleManifest {
        revision: time.to_string(),
        roots: vec![resource_path.clone()]
    };
    let maniser = serde_json::to_vec(&manifest)?;
    let mut header_mani = Header::new_gnu();
    header_mani.set_size(maniser.len().try_into().unwrap());
    header_mani.set_cksum();

    let mut header_res = Header::new_gnu();
    header_res.set_size(result.as_bytes().len().try_into().unwrap());
    header_res.set_cksum();

    let encoder = GzEncoder::new(Vec::new(), Compression::default());
    let mut archive = Builder::new(encoder);
    archive.append_data(&mut header_res, path, result.as_bytes())?;
    archive.append_data(&mut header_mani, ".manifest", maniser.as_slice())?;
    
    Ok(archive.into_inner()?.finish()?)
}


fn main() {
    // Load the configuration
    dotenv::dotenv().ok();

    // Create the fairing for the metrics endpoint
    let prometheus = PrometheusMetrics::new();

    rocket::ignite()
        .attach(CORS)
        .attach(prometheus.clone())
        // Monitoring Endpoints
        .mount("/metrics", prometheus)
        .mount("/", routes![healthz])
        // Endpoints v1
        .mount("/v1", routes![bundle])
        .launch();
}
