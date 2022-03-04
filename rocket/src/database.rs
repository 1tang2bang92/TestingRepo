use sqlx::SqlitePool;

pub async fn init_db(conn: &SqlitePool) {
    sqlx::query!(
        r#"
            CREATE TABLE IF NOT EXISTS "USER" (
                ID	            TEXT NOT NULL,
                PW	            TEXT NOT NULL,
                NICKNAME	    TEXT NOT NULL,
                SALT	        TEXT NOT NULL,
                CREATE_DATE     TEXT NOT NULL,
                UPDATE_DATE 	TEXT NOT NULL,
                PRIMARY KEY(ID)  
            )
        "#,
    )
    .execute(conn)
    .await
    .unwrap();
}
