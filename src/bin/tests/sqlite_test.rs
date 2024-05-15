#[cfg(test)]
mod tests {
    
    use std::result::Result;
    use sqlx::{sqlite::SqliteQueryResult,  SqlitePool, };

    async fn create_schema(db_url:&str) -> Result<SqliteQueryResult, sqlx::Error> {
        let pool = SqlitePool::connect(&db_url).await?;
        let qry = 
        "PRAGMA foreign_keys = ON ;
        CREATE TABLE IF NOT EXISTS settings
            (
                settings_id             INTEGER PRIMARY KEY NOT NULL,
                description             TEXT                NOT NULL,
                created_on              DATETIME DEFAULT (datetime('now','localtime')),
                updated_on              DATETIME DEFAULT (datetime('now','localtime')),
                done                    BOOLEAN             NOT NULL DEFAULT 0
            );    
        CREATE TABLE IF NOT EXISTS project
            (
                project_id                   INTEGER PRIMARY KEY AUTOINCREMENT,
                product_name                 TEXT ,
                created_on                   DATETIME DEFAULT (datetime('now','localtime')),
                updated_on                   DATETIME DEFAULT (datetime('now','localtime')),
                img_directory                TEXT NOT NULL,
                out_directory                TEXT NOT NULL,
                status                       TEXT NOT NULL,
                settings_id                  INTEGER  NOT NULL DEFAULT 1,
                FOREIGN KEY (settings_id)    REFERENCES settings (settings_id) ON UPDATE SET NULL ON DELETE SET NULL
            );";
        let result = sqlx::query(&qry).execute(&pool).await;   
        pool.close().await; 
        return result;
    }

    #[async_std::test]
    async fn test_create_schema() {
        let db_url = "sqlite://sqlite.db";
        let result = create_schema(db_url).await;
        assert!(result.is_ok());
    }

    #[async_std::test]
    async fn test_insert_into_settings() {
        let db_url = "sqlite://sqlite.db";
        let pool = SqlitePool::connect(db_url).await.unwrap();
        let result = create_schema(db_url).await;
        assert!(result.is_ok());

        let qry = "INSERT INTO settings (description) VALUES($1)";
        let result = sqlx::query(&qry).bind("testing").execute(&pool).await;
        match result {
            Ok(_) => println!("Inserted into settings"),
            Err(e) => panic!("{}",e),
        }
        assert!(result.is_ok());

        let row: (String,) = sqlx::query_as("SELECT description FROM settings")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(row.0, "testing");
    }
}
