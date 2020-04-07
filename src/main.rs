use warp::Filter;

#[tokio::main]
async fn main() {
    let roll = warp::path("roll");
    let dice_str = warp::path::param().and(warp::path::end());
    let routes = roll.and(dice_str).map(|dice: String| {
        format!("dice: {}", dice)
    });
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
