use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://svc.metrotransit.org/NexTrip/{stopid}?format=json",
        stopid = "15280"
    );
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    let nexttrips: Vec<Nexttrips> = response.json().await?;
    println!("{:?}", nexttrips);
    Ok(())
}
