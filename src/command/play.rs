use slashy::{
    command, commands::{CommandResult},
    framework::{CommandContext},
    subcommand,
    serenity::builder::CreateMessage
};
use songbird::{SerenityInit, input::{restartable::Restartable, Input}, tracks::Track, Event, TrackEvent};
// use serenity::prelude::Context;


command!{
    play,
    "type the name of the song",
    play,
    [
        required String name | "name"
    ]
}


#[subcommand]
async fn play(com_ctx: &CommandContext) -> CommandResult{
    //let name = com_ctx.get_str_arg("name").expect("name error").as_str();
    let manager = songbird::get(&com_ctx.ctx).await.expect("error: creating songbird manager failed").clone();
    let handler_key = match manager.get(com_ctx.guild_id().expect("guild does not exist")){
        Some(handler) => handler,
        None => {
            com_ctx.send_str("mb").await;
            return Ok(())
        }
    };
    let mut handler = handler_key.lock().await;
    let name = com_ctx.get_str_arg("name").expect("error: search has no results");
    let source:Input = Restartable::ytdl_search(name, true).await?.into();
    let _queue = handler.enqueue_source(source);
    //com_ctx.channel().await.expect("").id().send_message(com_ctx.ctx.http, "help");
    //com_ctx.send_str(&("Added".to_string() + &handler.queue().current().expect("could not find track in source's metadata").metadata().track.as_deref().expect("not found").to_owned() + "to queue.")).await?;
//    println!("{}", handler.queue().current().expect("o").metadata().source_url.as_ref().expect("ooh"));
    Ok(())
}