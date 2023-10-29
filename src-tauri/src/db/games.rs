pub struct Games{
    pub idx: i32,
    pub id: String,
    pub platform: String,
    pub name: String,
    pub version: String,
    pub scene_group: String,
    pub magnet: String,
    pub size: f32,
    pub installer: String,
    pub page: String
}

impl Games{
    pub fn new(idx: i32, id: String, platform: String, name: String, version: String, scene_group: String, magnet: String, size: f32, installer: String, page: String) -> Games{
        Games {
            idx,
            id,
            platform,
            name,
            version,
            scene_group,
            magnet,
            size,
            installer,
            page
        }
    }

   pub async fn add_db(&mut self, db: sqlx::Pool<sqlx::Sqlite>) -> Result<bool, String> {
        let result = sqlx::query("INSERT INTO GAMES (id, platform, name, version, sceneGroup, magnet, size, installer, page) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&self.id)
            .bind(&self.platform)
            .bind(&self.name)
            .bind(&self.version)
            .bind(&self.scene_group)
            .bind(&self.magnet)
            .bind(&self.size)
            .bind(&self.installer)
            .bind(&self.page)
            .execute(&db)
            .await;

        match result {
            Ok(_) => return Ok(true),
            Err(e) => return Err(e.to_string())
        }
        
    }
}