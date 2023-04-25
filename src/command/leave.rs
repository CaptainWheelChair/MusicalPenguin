use slashy::{
    command, commands::{CommandResult},
    framework::{CommandContext},
    subcommand
};

use songbird::{SerenityInit, input::{restartable::Restartable, Input}};

command!{
    leave,
    "buh-bye",
    leave
}

#[subcommand]
pub async fn leave(com_ctx: &CommandContext) -> CommandResult{
    let manager = songbird::get(&com_ctx.ctx).await.expect("could not retrieve songbird's voice client");
    com_ctx.send_str("buh_bye").await?;
    manager.leave(com_ctx.guild_id().unwrap()).await?;
    Ok(())
}