#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod parse;
mod roll;

use parse::parse_dice_str;
use rocket::response::status::BadRequest;
use roll::{roll_crit, roll_normal};

fn main() {
    rocket::ignite()
        .mount("/roll", routes![normal, critical])
        .launch();
}

#[get("/<dice>")]
fn normal(dice: String) -> Result<String, BadRequest<String>> {
    let cmd = parse_dice_str(dice.as_ref())?;
    let rolls = roll_normal(&cmd);
    let roll_str = rolls
        .0
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join(" + ");
    let resp = [roll_str, rolls.0.iter().sum::<usize>().to_string()].join(" = ");
    Ok(resp)
}

#[get("/crit/<dice>")]
fn critical(dice: String) -> Result<String, BadRequest<String>> {
    let cmd = parse_dice_str(dice.as_ref())?;
    let rolls = roll_crit(&cmd);
    let roll_str: String = rolls
        .0
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join(" + ");
    let resp = [roll_str, rolls.0.iter().sum::<usize>().to_string()].join(" = ");
    Ok(resp)
}
