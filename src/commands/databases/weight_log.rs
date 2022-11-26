use std::{env,process};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;

pub async fn insert_weight(weight: f64, date: String){
    dotenv().ok();    // .envの読み込み
    let database_url=match env::var("DATABASE_URL"){
        Ok(ok)=>ok,
        Err(err)=>{
            eprint!("Error: std::env said, {}\n",err);
            process::exit(1);
        }
    };

    let pool=match MySqlPool::connect(&database_url).await{
        Ok(ok)=>ok,
        Err(err)=>{
            eprint!("Error: sqlx said, {}\n",err);
            process::exit(1);
        }
    };

    if let Err(err)=sqlx::query!("
        REPLACE INTO weight_log(
            weight,
            date
        )
        VALUES(
            ?,
            ?
        )
    ",
    weight,
    date)
    .execute(&pool)
    .await{
        eprint!("Error: sqlx said {}\n",err);
        process::exit(1);
    }
}