use std::default::Default;

use sqlx::sqlite::SqlitePool;

#[derive(Debug)]
pub struct UserBuilder {
    uuid: String,
    id: String,
    pw: String,
    nickname: String,
    salt: String,
    create_date: String,
    updated_date: String,
}

#[derive(Debug)]
pub struct User {
    uuid: String,
    id: String,
    pw: String,
    nickname: String,
    salt: String,
    create_date: String,
    updated_date: String,
}

impl Default for UserBuilder {
    fn default() -> Self {
        UserBuilder {
            uuid: "".to_string(),
            id: "".to_string(),
            pw: "".to_string(),
            salt: "salt".to_string(),
            nickname: "".to_string(),
            create_date: "".to_string(),
            updated_date: "".to_string(),
        }
    }
}

impl UserBuilder {
    pub fn create(self, id: String, pw: String, nickname: String) -> Self {
        use chrono::Local;
        use uuid::Uuid;

        let now = Local::now();
        let uuid = Uuid::new_v4().to_string();
        UserBuilder {
            uuid,
            id,
            pw,
            salt: self.salt,
            nickname,
            create_date: now.to_string(),
            updated_date: now.to_string(),
        }
    }

    pub fn build(self) -> User {
        User {
            uuid: self.uuid,
            id: self.id,
            pw: self.pw,
            salt: self.salt,
            nickname: self.nickname,
            create_date: self.create_date,
            updated_date: self.updated_date,
        }
    }
}

impl User {
    pub async fn create_user(self, conn: &SqlitePool) -> bool {
        println!("{self:#?}");
        let result = sqlx::query!(
            r#"
INSERT INTO "USERS" (
    UUID,
    ID,
    PW,
    NICKNAME,
    SALT,
    CREATE_DATE,
    UPDATE_DATE
) VALUES (
    $1, $2, $3, $4, $5, $6, $7
) RETURNING *
            "#,
            self.uuid,
            self.id,
            self.pw,
            self.nickname,
            self.salt,
            self.create_date,
            self.updated_date
        )
        .fetch_one(conn)
        .await;

        match result {
            Ok(_) => true,
            Err(e) => {
                println!("{e:#?}");
                false
            }
        }
    }

    pub async fn get_user_by_id(uuid: String, conn: &SqlitePool) {
        let result = sqlx::query!(
            r#"
SELECT *
FROM USERS
WHERE ID = $1
            "#,
            uuid
        )
        .fetch_one(conn)
        .await;
    }

    pub async fn update_nickname(self, conn: &SqlitePool) {
        use chrono::Local;
        let now = Local::now().to_string();

        let result = sqlx::query!(
            r#"
UPDATE USERS
SET (
    NICKNAME,
    UPDATE_DATE
) = (
    $1, $2
)
WHERE UUID = $3
AND ID = $4
AND PW = $5
RETURNING *
            "#,
            self.nickname,
            now,
            self.uuid,
            self.id,
            self.pw
        )
        .fetch_one(conn)
        .await;
    }

    pub async fn delete_user(self, conn: &SqlitePool) -> bool {
        let result = sqlx::query!(
            r#"
DELETE FROM USERS 
WHERE UUID = $1
AND ID = $2
AND PW = $3
            "#,
            self.uuid,
            self.id,
            self.pw
        )
        .fetch_one(conn)
        .await;
        true
    }
}
