pub mod argo;

use std::collections::HashMap;
use std::fs::rename;
use k8s_openapi::api::core::v1::Pod;
use dotenv::dotenv;
use anyhow::{bail, Context, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use derivative::Derivative;
use futures::{StreamExt, TryStreamExt};
use chrono::DateTime;
use http::HeaderMap;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, ApiResponse, OpenApi, OpenApiService};
use poem_openapi::payload::Json;

const CHART_REPO: &str = "https://charts.vaughn.sh";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
    name: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SynkronizedProject {
    name: String,
    template: String,
    config: serde_yaml::Value
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Charts {
    apiVersion: String,
    entries: HashMap<String, Vec<ChartVersion>>,
    generated: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChartVersion {
    apiVersion: String,
    appVersion: String,
    created: String,
    description: String,
    digest: String,
    name: String,
    #[serde(rename="type")]
    chart_type: String,
    urls: Vec<String>,
    version: String
}

async fn get_chart_template(chart_name: &str) -> Result<Template> {
    let chart_repo_yaml = format!("{CHART_REPO}/index.yaml");

    let body = reqwest::get(chart_repo_yaml)
    .await?
    .text()
    .await?;

    let charts: Charts = serde_yaml::from_str(&body).expect("Unable to parse charts.");
    match charts.entries.get(chart_name) {
        Some(charts) => Ok(Template{
            name: charts[0].name.clone(),
            version: charts[0].version.clone()
        }),
        None => panic!("No viable charts were found for {}", chart_name)
    }
}

async fn read_synkronized_file(fp: &str) -> Result<SynkronizedProject> {
    let f = std::fs::File::open(fp)?;
    let synkronized_yaml: SynkronizedProject = serde_yaml::from_reader(f)?;
    Ok(synkronized_yaml)
}

// #[tokio::main]
// async fn main() -> Result<()>{
//     dotenv().ok();
//
//     let mut kube_config = Config::from_kubeconfig(&KubeConfigOptions::default()).await?;
//     kube_config.accept_invalid_certs = true;
//     let client = Client::try_from(kube_config).expect("Could not configure the client.");
//
//     let synkronized_yaml = read_synkronized_file("synkronized.yaml").await?;
//     let chart_template = get_chart_template(&synkronized_yaml.template).await?;
//
//     // println!("{:?}", chart);
//     // println!("{:?}", synkronized_yaml);
//
//     let application = argo::Application::create(synkronized_yaml, chart_template);
//     application.apply(client).await
// }

struct Api;

#[derive(ApiResponse)]
enum PackageUpdateResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 500)]
    Error,
}

#[derive(ApiResponse)]
enum HealthCheckResponse {
    #[oai(status = 200)]
    Ok,
}

#[OpenApi]
impl Api {
    #[oai(path = "/synkronized", method = "post")]
    async fn synkronized(&self, package: Json<serde_json::Value>, headers: &HeaderMap) -> PackageUpdateResponse {
        println!("{:?}", package);
        println!("{:?}", serde_json::to_string(&package.0).unwrap());
        if headers.get("X-GitHub-Event").unwrap() == "ping" {
            return PackageUpdateResponse::Ok
        }
        PackageUpdateResponse::Ok
    }
    #[oai(path = "/ping", method = "post")]
    async fn health_check(&self) -> HealthCheckResponse {
        HealthCheckResponse::Ok
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:8080");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await
}
