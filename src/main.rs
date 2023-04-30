mod command;

use command::*;

use std::env;
use slashy::{
    command, commands::{CommandResult},
    framework::{CommandContext, Framework},
    serenity::{prelude::GatewayIntents, Client, futures::future::BoxFuture},
    settings::Settings,
    subcommand
};
use songbird::{SerenityInit, input::restartable::Restartable};
//struct Handler;
//
//#[serenity::async_trait]
//impl EventHandler for Handler {
//    async fn message(&self, ctx: Context, msg: Message) {
//        if msg.content == "!online"{
//            ctx.online().await;
//        }
//    }
//    async fn ready(&self, ctx: Context, ready: Ready){
//        println!("Connection Established by {}", ready.user.name);
//        
//        tokio::time::sleep(Duration::from_secs(5)).await;
//        let guilds = ctx.cache.guilds().len();
//        println!("{} guilds", guilds);
//        
//    }
//}


#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    tracing_subscriber::fmt::init();
    
    let token = &env::var("DISCORD_TOKEN").expect("A token");
    let intents =  GatewayIntents::non_privileged()
                  | GatewayIntents::GUILD_MESSAGES
                  | GatewayIntents::DIRECT_MESSAGES
                  | GatewayIntents::GUILD_VOICE_STATES
                  | GatewayIntents::MESSAGE_CONTENT;
    let app_id = env::var("APP_ID").expect("expected App ID").parse().expect("app id parse failed");
    
    let settings = Settings {
        prefixes: vec!["?"],
        auto_register: true,
        auto_delete: true,
        slash_command_guilds: vec![],
    };
    
    let framework = Framework::new(settings, app_id, token.clone())
      .await
      .command::<JOIN_COMMAND>()
      .command::<PLAY_COMMAND>()
      .command::<LEAVE_COMMAND>()
      .command::<SKIP_COMMAND>()
      .command::<CLEAR_COMMAND>();
    
    let mut client = Client::builder(&token, intents)
        .event_handler(framework)
        .register_songbird()
        .await
        .expect("error: client not built");
    
    tokio::spawn(async move {
        let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
    });
    tokio::signal::ctrl_c().await;
    println!("Received Ctrl-C, shutting down.");
}

//
//async fn BotCommands(ctxCom: &CommandContext) -> CommandArgumentsTree{
//    let musical_commands = CommandArgumentsTree{
//        children: Some(vec![
//            CommandArguments::SubCommand {
//                name: "join",
//                description: "Bot Joins The Users Voice Channel",
//                required: false,
//                options: None,
//                func: Some(joined)
//        }
//        ]),
//        func: None
//    }
//    return musical_commands
//}


//command!{
//    joine,
//    "",
//    ase
//}
//#[subcommand]
//async fn joined(ctxCom: &CommandContext) -> CommandResult{
//    let guild_id =  ctxCom.guild_id().unwrap();
//    let guild = guild_id.to_guild_cached(&ctxCom.ctx.cache).unwrap();
//    
//    let channel_id = guild.voice_states.get(&ctxCom.author().unwrap().id).and_then(|authin_chan| authin_chan.channel_id);
//    let connect_to = match channel_id {
//        Some(channel) => channel,
//        None => {
//            ctxCom.send_str("Please enter a voice channel").await;
//            return Ok(());
//        }
//    };
//    let manager = songbird::get(&ctxCom.ctx).await
//        .expect("Songbird Voice client placed in at initialisation.").clone();
//    
//    let _handler = manager.join(guild_id, connect_to).await;
//    ctxCom.send_str("*clever join message*").await;
//    Ok(())
//}
//#[command]
//async fn play(ctx: &Context, msg: &Message, mut args: Args){
//    Ok(())
//}
//
