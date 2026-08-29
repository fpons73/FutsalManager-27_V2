use sqlx::SqlitePool;

pub async fn record_match_result(pool:&SqlitePool,club_id:i64,date:&str,headline:String,body:String,important:bool)->Result<(),String>{sqlx::query("INSERT OR IGNORE INTO world_news(club_id,news_type,headline,body,date,importance) VALUES(?,?,?,?,?,?)").bind(club_id).bind("match_result").bind(headline).bind(body).bind(date).bind(important as i64).execute(pool).await.map_err(|e|e.to_string())?;Ok(())}

#[cfg(test)]
mod tests{use super::*;use crate::{db,world};#[tokio::test]async fn match_news_is_idempotent(){let p=db::init_memory_pool().await.unwrap();world::seed_world(&p).await.unwrap();record_match_result(&p,1,"2026-07-10","Resultado".into(),"Partido terminado".into(),true).await.unwrap();record_match_result(&p,1,"2026-07-10","Resultado".into(),"Partido terminado".into(),true).await.unwrap();let(n,):(i64,)=sqlx::query_as("SELECT COUNT(*) FROM world_news WHERE club_id=1").fetch_one(&p).await.unwrap();assert_eq!(n,1);}}
