use slashy::{
    command, commands::{CommandResult},
    framework::{CommandContext},
    subcommand
};
use songbird::{SerenityInit, input::{restartable::Restartable, Input}, tracks::Track, Event, TrackEvent};
// use serenity::prelude::Context;


command!{
    skip,
    "type the name of the song",
    skip,
}


#[subcommand]
async fn skip(com_ctx: &CommandContext) -> CommandResult{
    //let name = com_ctx.get_str_arg("name").expect("name error").as_str();
    let manager = songbird::get(&com_ctx.ctx).await.expect("error: creating songbird manager failed").clone();
    let handler_key = match manager.get(com_ctx.guild_id().expect("guild does not exist")){
        Some(handler) => handler,
        None => {
            com_ctx.send_str("mb").await;
            return Ok(())
        }
    };
    let handler = handler_key.lock().await;
    handler.queue().skip()?;
    Ok(())
}