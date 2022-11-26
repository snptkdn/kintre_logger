use super::super::poise_data::*;
use poise::serenity_prelude::{self as serenity, Mentionable};

#[derive(Debug)]
enum  WeekDay{
    MON,
    TUE,
    WED,
    THU,
    FRI,
    SAT,
    SUN,
    NONE,
}

impl WeekDay {
    pub fn get_string_weekday(weekday: Self) -> String {
        match weekday {
            WeekDay::MON => "Monday".to_string(),
            WeekDay::TUE => "Tuesday".to_string(),
            WeekDay::WED => "Wednesday".to_string(),
            WeekDay::THU => "Thursday".to_string(),
            WeekDay::FRI => "Friday".to_string(),
            WeekDay::SAT => "Sunday".to_string(),
            WeekDay::SUN => "Saturday".to_string(),
            WeekDay::NONE => "Sunday".to_string()
        }
    }
}
/// set schedule of training
#[poise::command(slash_command, prefix_command)]
pub async fn set_schedule(
    ctx: Context<'_>,
    #[description = "weekday ex) SUN MON"] weekday: String,
    #[description = "traning name"] training: String,
) -> Result<(), Error> {
    let weekday = match weekday.as_str() {
        "MON" => WeekDay::MON,
        "TUE" => WeekDay::TUE,
        "WED" => WeekDay::WED,
        "THU" => WeekDay::THU,
        "FRI" => WeekDay::FRI,
        "SAT" => WeekDay::SAT,
        "SUN" => WeekDay::SUN,
        _ => WeekDay::NONE
    };

    if let WeekDay::NONE = weekday {
        ctx.say("weekday shoud specify MON or TUE or WED or THU or FRI or SAT or SUN.").await?;
        return Ok(());
    }

    ctx.say(format!(
        "set {} in {}!",
        training,
        WeekDay::get_string_weekday(weekday))
    ).await?;

    Ok(())
}
