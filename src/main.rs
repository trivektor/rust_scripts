#[allow(unused)]

//mod dad_jokes;
mod spacex_api;

#[tokio::main]
async fn main() {
    let launches = spacex_api::get_launches().await.unwrap();
    let rockets = spacex_api::get_rockets().await.unwrap();

    for launch in launches.iter() {
        // https://stackoverflow.com/a/39272638/477697
        println!("{:#?}", launch.mission_name);
    }

    for rocket in rockets.iter() {
        println!("{:#?}", rocket.rocket_name);
    }
}
