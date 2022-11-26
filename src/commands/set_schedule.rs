use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "set training schedule. ex) '/set_schedule WED [Trainig Name]'"]
async fn set_schedule(ctx: &Context, msg: &Message) -> CommandResult {
    println!("{}", msg.content);
    msg.channel_id
        .say(&ctx.http, format!("{} にゃーん", msg.author.mention()))
        .await?;
    // CommandResultはResultを継承している
    // `Result?` は正常な値の場合，Resultの中身を返し，エラーの場合は即座にreturnする演算子
    Ok(())
}
