use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};
use anyhow::Result;

pub(crate) const CHART_REPO: &str = "https://charts.vaughn.sh";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
    pub(crate) name: String,
    pub(crate) version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Charts {
    #[serde(rename="apiVersion")]
    api_version: String,
    entries: HashMap<String, Vec<ChartVersion>>,
    generated: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChartVersion {
    #[serde(rename="apiVersion")]
    api_version: String,
    #[serde(rename="appVersion")]
    app_version: String,
    created: String,
    description: String,
    digest: String,
    name: String,
    #[serde(rename="type")]
    chart_type: String,
    urls: Vec<String>,
    version: String
}

impl Template {
    pub(crate) async fn from_chart_repo(chart_name: &str) -> Result<Template> {
        let chart_repo_yaml = format!("{CHART_REPO}/index.yaml");

        let body = reqwest::get(chart_repo_yaml)
            .await?
            .text()
            .await?;

        let charts: Charts = serde_yaml::from_str(&body)?;
        match charts.entries.get(chart_name) {
            Some(charts) => Ok(Template {
                name: charts[0].name.clone(),
                version: charts[0].version.clone()
            }),
            None => panic!("No viable charts were found for {}", chart_name)
        }
    }
}
