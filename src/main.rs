use std::{f64::consts::PI, sync::OnceLock};

use ::rhai::Engine;
use rosu_v2::{Osu, prelude::UserExtended};
use tokio::task::spawn_blocking;

use crate::rhai::{osu::register_osu, register_helpers};

mod discord;
mod rhai;

static OSU: OnceLock<Osu> = OnceLock::new();

pub fn osu<'a>() -> &'a Osu {
    OSU.get().unwrap()
}

pub async fn init_osu() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let client_id: u64 = std::env::var("CLIENT_ID")?.parse()?;
    let client_secret = std::env::var("CLIENT_SECRET")?;

    let osu = Osu::new(client_id, client_secret).await?;
    let _ = OSU.set(osu);
    Ok(())
}

enum Message<'a> {
    Player(&'a str),
}

enum RecvMessage {
    Player(UserExtended),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_osu().await?;

    let mut engine = Engine::new();

    register_osu(&mut engine);
    register_helpers(&mut engine);

    let test = r#"
    rankings().extract(1..=10).map(|| top(this.name, 10)).map(|v| v.reduce|sum| sum + this.pp, 0).for_each(|| print(`{this}`))
    "#;
    let mut res = Ok(());
    while res.is_ok() {
        let stdin = std::io::stdin();
        res = engine.run(&stdin.lines().next().unwrap().unwrap());
    }
    println!("{res:?}");
    Ok(())
}
