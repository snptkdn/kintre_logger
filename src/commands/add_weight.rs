use super::super::poise_data::*;
use super::databases::weight_log::insert_weight;
use chrono::{Utc, NaiveDate};

/// add weight of day
#[poise::command(slash_command, prefix_command)]
pub async fn add_weight(
    ctx: Context<'_>,
    #[description = "body weight(kg)"] weight: f64,
    #[description = "date(if you don't specify, set today.)"] date: Option<String>,
) -> Result<(), Error> {
    let date = 
        if let Some(date) = date {
            NaiveDate::parse_from_str(&date, "%Y/%m/%d")
        } else {
            Ok(Utc::now().date_naive())
        };
    
    if let Ok(date) = date {
        insert_weight(weight, date.to_string()).await;
        ctx.say(format!(
            "set {} in {}!",
            weight,
            date
        )).await?;
    } else {
        ctx.say("date format invalid. should specify ex. 2022/11/29").await?;
    }


    Ok(())
}
