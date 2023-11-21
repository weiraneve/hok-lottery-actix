use crate::dao::Table;
use crate::model::hero::Hero;

impl Table<Hero> {
    pub async fn get_heroes_not_is_pick(&self) -> Result<Hero, sqlx::Error> {
        sqlx::query_as::<_, Hero>("SELECT * FROM `hero` h WHERE h.is_pick = FALSE ORDER BY RAND() LIMIT 1")
            .fetch_one(&*self.pool)
            .await
    }

    pub async fn save_hero(&self, hero: Hero) -> Result<(), sqlx::Error> {
        sqlx::query(r#"INSERT INTO hero (id, name, line, is_pick)
        VALUES (?, ?, ?, ?)
        ON DUPLICATE KEY UPDATE
        name = VALUES(name),
        line = VALUES(line),
        is_pick = VALUES(is_pick)
    "#)
            .bind(hero.id)
            .bind(&hero.name)
            .bind(&hero.line)
            .bind(hero.is_pick)
            .execute(&*self.pool)
            .await?;

        Ok(())
    }

    pub async fn reset_all_heroes(&self) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE `hero` SET `is_pick`=false WHERE `is_pick`=true ")
            .execute(&*self.pool)
            .await?;

        Ok(())
    }
}
