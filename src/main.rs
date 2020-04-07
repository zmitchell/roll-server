use warp::Filter;

#[tokio::main]
async fn main() {
    let roll = warp::path!("roll" / String)
        .map(|dice| format!("Roll {}", dice));

    warp::serve(roll)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
