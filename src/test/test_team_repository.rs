#[cfg(test)]
mod test {
    use std::env;
    use std::sync::Arc;

    use chrono::{FixedOffset, Utc};
    use dotenvy::dotenv;
    use sqlx::{Executor, MySql, Pool};

    use crate::model::team::Team;
    use crate::repository::team::{TeamRepository, TeamRepositoryImpl};

    async fn setup_database() -> Result<Pool<MySql>, sqlx::Error> {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL_TEST").expect("DATABASE_URL must be set");

        let pool = Pool::<MySql>::connect(&database_url).await?;
        let create_table_query = r#"
            CREATE TABLE IF NOT EXISTS `team` (
                `id` int NOT NULL AUTO_INCREMENT,
                `encrypt_code` varchar(20) CHARACTER SET utf8mb4 DEFAULT NULL COMMENT '队伍秘钥',
                `pick_content` varchar(100) CHARACTER SET utf8mb4 DEFAULT NULL COMMENT '抽取结果',
                `is_picked` tinyint(1) NOT NULL DEFAULT '0' COMMENT '是否抽取过',
                `update_time` datetime DEFAULT NULL COMMENT '更新时间',
                PRIMARY KEY (`id`) USING BTREE
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
        "#;
        pool.execute(create_table_query).await?;
        Ok(pool)
    }

    async fn cleanup_database(pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
        pool.execute("DELETE FROM `team`").await?;
        Ok(())
    }

    #[sqlx::test]
    async fn test_save_team() {
        let pool = setup_database().await.expect("Failed to setup database");
        let team_repository = Arc::new(TeamRepositoryImpl::new(Arc::new(pool.clone())));
        let team = team_repository.save(
            Team {
                id: 9,
                encrypt_code: String::from("asd"),
                pick_content: String::from("[鲁班大师,白起]or[杨戬,雅典娜]"),
                is_picked: true,
                update_time: Utc::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap()).naive_local(),
            }
        )
            .await
            .unwrap_or_else(|err| panic!("Failed to save team: {}", err));

        let encrypt_code = String::from("asd");
        let team = team_repository
            .get_by_encrypt_code(encrypt_code)
            .await
            .unwrap_or_else(|err| panic!("Failed to get team by encrypt_code: {}", err));

        assert_eq!(team.id, 9);

        cleanup_database(&pool).await.expect("Failed to cleanup database");
    }
}
