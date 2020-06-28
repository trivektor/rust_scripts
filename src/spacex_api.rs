#[allow(unused)]

use serde::Deserialize;

const API_URL : &str = "https://api.spacexdata.com/v3";

#[derive(Deserialize, Debug)]
pub struct Rocket {
    rocket_id: String,
    rocket_name: String,
    rocket_type: String,
}

#[derive(Deserialize, Debug)]
pub struct Launch {
    mission_name: String,
    rocket: Rocket
}

pub async fn get_launches() -> Result<(Vec<Launch>), reqwest::Error> {
    let url = format!("{}/launches", API_URL);
    let response = reqwest::get(&url).await?;
    let launches = response.json().await?;

    Ok((launches))
}

pub async fn get_rockets() -> Result<(Vec<Rocket>), reqwest::Error> {
    let url = format!("{}/rockets", API_URL);
    let response = reqwest::get(&url).await?;
    let rockets = response.json().await?;

    Ok((rockets))
}
