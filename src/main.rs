mod parse;
mod roll;

use parse::ParseError;
use warp::Filter;

#[tokio::main]
async fn main() {
    let roll = warp::path("roll");
    let dice_str = warp::path::param().and(warp::path::end());
    let crit = warp::path("crit");
    let normal_roll = roll
        .and(dice_str)
        .map(|dice: String| {
            format!("normal: {}", dice)
        });
    let critical_roll = roll
        .and(crit)
        .and(dice_str)
        .map(|dice: String| {
            format!("critical: {}", dice)
        });
    let routes = warp::get().and(normal_roll.or(critical_roll));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
