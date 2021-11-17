use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Nexttripdeparture {
    Actual: bool,
    BlockNumber: u32,
    DepartureText: String,
    DepartureTime: String,
    Description: String,
    Gate: Option<String>,
    Route: String,
    RouteDirection: String,
    Terminal: String,
    VehicleHeading: u32,
    VehicleLatitude: u32,
    VehicleLongitude: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://svc.metrotransit.org/NexTrip/{stopid}?format=json",
        stopid = "15280" //15412 eastbound
    );
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;
    
    println!("{:#?}", response);

    let departures: Vec<Nexttripdeparture> = response.json().await?;
    println!("{:?}", departures);
    Ok(())
}
