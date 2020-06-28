#[allow(unused)]

//mod dad_jokes;
mod spacex_api;

#[tokio::main]
async fn main() {
    let launches = spacex_api::get_launches().await;
    let rockets = spacex_api::get_rockets().await;

    println!("{:?}", launches);
    println!("{:?}", rockets);
}
