use std::{f64::consts::PI, sync::OnceLock};

use ::rhai::Engine;
use rosu_v2::{Osu, prelude::UserExtended};
use tokio::task::spawn_blocking;

mod rhai;

static OSU: OnceLock<Osu> = OnceLock::new();

pub fn osu<'a>() -> &'a Osu {
    OSU.get().unwrap()
}

pub async fn init_osu() -> Result<(), Box<dyn std::error::Error>> {
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

    engine.register_fn("player", crate::rhai::osu::get_user);

    engine
        .run(
            r#"
            let x = player("test");
            print(`Got ${x}`);
            print(`Name is ${x}`);
            "#,
        )
        .unwrap();
    Ok(())
}
