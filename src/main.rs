mod commands;
use std::{collections::HashSet, usize};

use serenity::async_trait;
use serenity::framework::standard::{
    help_commands,
    macros::{group, help},
    Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::framework::StandardFramework;
use serenity::model::{channel::Message, gateway::Ready, id::UserId};
use serenity::prelude::{Client, Context, EventHandler, GatewayIntents};

use serde::{Deserialize, Serialize};
use serde_json::Result;
use dotenv::*;
use std::env;

use commands::neko::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
    async fn message(&self, context: Context, message: Message) {
        println!("message send. {:?}", message);
    }
}

#[group]
#[description("汎用コマンド")]
#[summary("一般")]
#[commands(neko)]
struct General;

#[help] // Helpコマンド
#[individual_command_tip = "これはヘルプコマンド"] // Helpコマンドの説明
#[strikethrough_commands_tip_in_guild = ""] // 使用できないコマンドについの説明を削除
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    // _ は使用しない返り値を捨てることを明示している
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    // 空のタプルをreturn（仕様）
    // Rustでは`;`なしの行は値としてreturnすることを表す
    // return Ok(()); と同義
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

fn get_token() -> Result<String> {
    dotenv().ok();

    Ok(env::var("DISCORD_TOKEN").expect("token isn't set."))
}

#[tokio::main]
async fn main() {
    // Discord Bot Token を設定
    let token = get_token().expect("Err トークンが見つかりません");
    // コマンド系の設定
    let framework = StandardFramework::new()
        // |c| c はラムダ式
        .configure(|c| c.prefix("~")) // コマンドプレフィック
        .help(&MY_HELP) // ヘルプコマンドを追加
        .group(&GENERAL_GROUP); // general を追加するには,GENERAL_GROUP とグループ名をすべて大文字にする

    // Botのクライアントを作成
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler) // 取得するイベント
        .framework(framework) // コマンドを登録
        .await
        .expect("Err creating client"); // エラーハンドリング

    // メインループ．Botを起動
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}