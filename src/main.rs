mod parse;
mod roll;

use parse::{ParseError, parse_dice_str};
use roll::{Roll, roll_normal, roll_crit};
use warp::Filter;

#[tokio::main]
async fn main() {
    let roll = warp::path("roll");
    let dice_str = warp::path::param().and(warp::path::end());
    let crit = warp::path("crit");
    let normal_roll = roll
        .and(dice_str)
        .map(|dice: String| {
            let cmd = parse_dice_str(dice.as_ref()).unwrap();
            let rolls = roll_normal(&cmd);
            let roll_str: String = rolls.dice.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(" + ");
            let resp = [roll_str, rolls.dice.iter().sum::<usize>().to_string()].join(" = ");
            resp
        });
    let critical_roll = roll
        .and(crit)
        .and(dice_str)
        .map(|dice: String| {
            let cmd = parse_dice_str(dice.as_ref()).unwrap();
            let rolls = roll_crit(&cmd);
            let roll_str: String = rolls.dice.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(" + ");
            let resp = [roll_str, rolls.dice.iter().sum::<usize>().to_string()].join(" = ");
            resp
        });
    let routes = warp::get().and(normal_roll.or(critical_roll));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
