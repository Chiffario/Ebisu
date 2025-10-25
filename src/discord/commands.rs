use poise::serenity_prelude as serenity;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// #[poise::command(prefix_command)]
// async fn run(ctx: Context<'_>, user: Option<serenity::User>) -> Result<(), Error> {
//     let u = user.as_ref.unwrap_or_else(|| ctx.author());

// }
