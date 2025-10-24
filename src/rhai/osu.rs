use futures::executor;
use rosu_v2::prelude::UserExtended;

use crate::osu;

pub(crate) fn get_user(name: &str) -> UserExtended {
    let mut data = Err(rosu_v2::error::OsuError::NotFound);
    let data_ref = &mut data;
    executor::block_on(async move {
        *data_ref = osu().user(name).await;
        println!("I am doing something");
        println!("User: {data_ref:?}");
    });
    data.unwrap()
}
