use chrono::prelude::*;
use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow};
use std::num::NonZeroU32;

use super::sqlx_error;

#[derive(Debug, Clone, Deserialize, Serialize, Default, FromRow)]
#[serde(default)]
pub struct User {
    pub id: String,
    pub pw: String,
    pub nickname: String,
    pub salt: String,
    pub create_date: String,
    pub update_date: String,
}

pub async fn user_info(conn: &SqlitePool, id: String) -> User {
    let result = sqlx::query!(
        r#"
            SELECT *
            FROM USER
            WHERE id = ?
        "#,
        id
    )
    .fetch_one(conn)
    .await
    .expect("Database:: User Not Found Error");

    User {
        id: result.ID,
        nickname: result.NICKNAME,
        create_date: result.CREATE_DATE,
        update_date: result.UPDATE_DATE,
        ..Default::default()
    }
}

pub async fn create_user(conn: &SqlitePool, data: User) -> Result<bool, &str> {
    let User {
        id, pw, nickname, ..
    } = data;

    let (hashed_pw, salt) = {
        const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
        let n_iter = NonZeroU32::new(100_000).unwrap();
        let rng = rand::SystemRandom::new();

        let mut salt = [0u8; CREDENTIAL_LEN];
        let _tmp = rng.fill(&mut salt).is_ok();

        let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];

        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            pw.as_bytes(),
            &mut pbkdf2_hash,
        );

        (
            HEXUPPER.encode(&pbkdf2_hash),
            HEXUPPER.encode(&salt).to_string(),
        )
    };

    let create_date = Local::now().to_string();
    let update_date = &create_date;

    let result = sqlx::query!(
        r#"
            INSERT INTO USER
            VALUES( ?, ?, ?, ?, ?, ? )
        "#,
        id,
        hashed_pw,
        nickname,
        salt,
        create_date,
        update_date,
    )
    .fetch_one(conn)
    .await;

    match result {
        Ok(_) => dbg!(Ok(true)),
        Err(err) => Err(sqlx_error(err)),
    }
}

pub async fn delete_user(conn: &SqlitePool, data: User) {
    let User { id, pw, .. } = data;

    sqlx::query!(
        r#"
            DELETE FROM USER
            WHERE ID = ?
            AND PW = ?
        "#,
        id,
        pw
    )
    .execute(conn)
    .await
    .expect("Database:: Delete User Error");
}
