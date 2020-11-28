use serde::{Deserialize, Serialize};
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct DomainProduct {
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DomainResponse {
    products: Vec<DomainProduct>,
}

#[derive(Serialize, Deserialize, Debug)]
struct DomainOwner {
    given: String,
    family: String,
    email: String,
    phone: String,

    streetaddr: String,
    city: String,
    state: String,
    country: String,
    zip: String,

    r#type: i8,
}

#[derive(Serialize, Deserialize, Debug)]
struct DomainPurchase {
    duration: i8,
    fqdn: String,
    owner: DomainOwner,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    apikey: String,
    domain: String,
    owner: DomainOwner,
}

async fn purchase(
    apikey: &str,
    domain: &str,
    owner: DomainOwner,
) -> Result<(), Box<dyn std::error::Error>> {
    let request_body = DomainPurchase {
        duration: 1,
        fqdn: domain.to_string(),
        owner: owner,
    };
    let client = reqwest::Client::new();
    let url = "https://api.gandi.net/v5/domain/domains";
    let resp = client
        .post(url)
        .json(&request_body)
        .header("Authorization", format!("Apikey {}", apikey))
        .header("content-type", "application/json")
        .header("dry-run", "1")
        .send()
        .await?
        .text()
        .await?;
    println!("{:?}", resp);
    Ok(())
}

fn get_config(config_file: String) -> Config {
    let data = fs::read_to_string(config_file).expect("Unable to read file");
    let config: Config = serde_json::from_str(&data).expect("Unable to parse");
    return config;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let help = format!(
        "Usage: {} PATH_TO_CONFIG_FILE",
        std::env::args().nth(0).unwrap()
    );
    let config_file = std::env::args().nth(1).expect(&help);
    let config = get_config(config_file);
    println!("{:?}", config);
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.gandi.net/v5/domain/check?name={}",
        config.domain
    );
    let resp = client
        .get(&url)
        .header("Authorization", format!("Apikey {}", config.apikey))
        .send()
        .await?
        .text()
        .await?;
    let result: DomainResponse = serde_json::from_str(&resp)?;
    let avail = &result.products[0].status;
    if avail == "available" || true {
        println!(
            "Domain {} is available, attempting to purchase.",
            config.domain
        );
        purchase(&config.apikey, &config.domain, config.owner).await?;
    }
    Ok(())
}
