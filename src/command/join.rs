use slashy::{
    command, commands::{CommandResult},
    framework::{CommandContext},
    subcommand
};
use songbird::SerenityInit;

command!{
    join,
    "join the call",
    join,
}

#[subcommand]
async fn join(ctxCom: &CommandContext) -> CommandResult{
    let guild_id =  ctxCom.guild_id().unwrap();
    let guild = guild_id.to_guild_cached(&ctxCom.ctx.cache).unwrap();

    let channel_id = guild.voice_states.get(&ctxCom.author().unwrap().id).and_then(|authin_chan| authin_chan.channel_id);
    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            ctxCom.send_str("Please enter a voice channel").await?;
            return Ok(());
        }
    };
    let manager = songbird::get(&ctxCom.ctx).await
    .expect("Songbird Voice client placed in at initialisation.").clone();

    manager.join(guild_id, connect_to).await;
    ctxCom.send_str("*clever join message*").await?;
    Ok(())
}
