use futures::executor;
use rhai::{Array, Dynamic, Engine};
use rosu_v2::prelude::{Score, User, UserExtended};

use crate::osu;

macro_rules! get_base {
    ($getter:expr) => {{
        let mut inner = None;
        let inner_mut_ref = &mut inner;
        executor::block_on(async move {
            *inner_mut_ref = $getter;
        });
        inner.unwrap()
    }};
}
pub(crate) fn get_user(name: &str) -> UserExtended {
    get_base!(osu().user(name).await.ok())
}

pub(crate) fn get_rankings() -> Array {
    get_base!(
        osu()
            .performance_rankings(rosu_v2::model::GameMode::Osu)
            .page(0)
            .await
            .ok()
    )
    .ranking
    .into_iter()
    .map(Dynamic::from)
    .collect()
}

pub(crate) fn get_top(name: &str) -> Array {
    get_base!(osu().user_scores(name).best().limit(100).await.ok())
        .into_iter()
        .map(Dynamic::from)
        .collect()
}

pub(crate) fn get_top_limit(name: &str, limit: i64) -> Array {
    get_base!(
        osu()
            .user_scores(name)
            .best()
            .limit(limit as usize)
            .await
            .ok()
    )
    .into_iter()
    .map(Dynamic::from)
    .collect()
}

macro_rules! register_type {
    ($type:ty as $name:expr, $field:ident, $($getter:expr => $getter_expr:expr),*) => {
        pastey::paste!{
            fn [< register_ $type:lower >](engine: &mut rhai::Engine) -> &mut rhai::Engine {
                engine.register_type_with_name::<$type>($name)
                    .register_fn("==", |item1: &mut $type, item2: $type| item1 == &item2)
                $(
                    .register_get($getter, |$field: &mut $type| $getter_expr)
                )*
            }
        }
    };
}

register_type!(UserExtended as "user", user,
    "id" => user.user_id,
    "name" => user.username.to_string(),
    "playcount" => user.statistics.as_ref().map(|stat| stat.playcount).unwrap_or_default(),
    "country" => user.country.clone(),
    "rank" => user.statistics.as_ref().and_then(|stat| stat.global_rank).unwrap_or_default(),
    "peak" => user.highest_rank.as_ref().map(|rank| rank.rank).unwrap_or_default(),
    "acc" => user.statistics.as_ref().map(|stat| stat.accuracy).unwrap_or_default(),
    "pp" => user.statistics.as_ref().map(|stat| stat.pp).unwrap_or_default()
);

register_type!(User as "user", user,
    "id" => user.user_id,
    "name" => user.username.to_string(),
    "playcount" => user.statistics.as_ref().map(|stat| stat.playcount).unwrap_or_default(),
    "country" => user.country.clone().unwrap_or_default(),
    "rank" => user.statistics.as_ref().and_then(|stat| stat.global_rank).unwrap_or_default(),
    "peak" => user.highest_rank.as_ref().map(|rank| rank.rank).unwrap_or_default(),
    "acc" => user.statistics.as_ref().map(|stat| stat.accuracy).unwrap_or_default(),
    "pp" => user.statistics.as_ref().map(|stat| stat.pp).unwrap_or_default()
);

register_type!(Score as "score", score,
    "pp" => score.pp.unwrap_or_default()
);

macro_rules! register_osu_func {
    {$($func:expr => $name:expr),*} => {
        fn register_osu_functions(engine: &mut rhai::Engine) -> &mut rhai::Engine {
            engine
                $(
                  .register_fn($name, $func)
                )*
        }
    };
}

register_osu_func! {
    get_user => "player",
    get_rankings => "rankings",
    get_top => "top",
    get_top_limit => "top"
}
pub fn register_osu(engine: &mut Engine) -> &mut Engine {
    register_osu_functions(engine);
    register_user(engine);
    register_score(engine);
    register_userextended(engine)
}
