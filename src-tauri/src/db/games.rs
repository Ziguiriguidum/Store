use super::get_database;

pub struct Games{
    pub idx: i32,
    pub id: String,
    pub platform: String,
    pub name: String,
    pub version: String,
    pub sceneGroup: String,
    pub magnet: String,
    pub size: f32,
    pub installer: String,
    pub page: String
}

impl Games{
    pub fn new(idx: i32, id: String, platform: String, name: String, version: String, sceneGroup: String, magnet: String, size: f32, installer: String, page: String) -> Games{
        Games {
            idx,
            id,
            platform,
            name,
            version,
            sceneGroup,
            magnet,
            size,
            installer,
            page
        }
    }

   pub async fn add_db(&self) -> Result<bool, String> {
        let db = get_database().await;
        let result = sqlx::query("INSERT INTO GAMES (id, platform, name, version, sceneGroup, magnet, size, installer, page) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(&self.id)
            .bind(&self.platform)
            .bind(&self.name)
            .bind(&self.version)
            .bind(&self.sceneGroup)
            .bind(&self.magnet)
            .bind(&self.size)
            .bind(&self.installer)
            .bind(&self.page)
            .execute(&db)
            .await;

        match result {
            Ok(_) => return Ok(true),
            Err(E) => return Err(E.to_string())
        }
        
    }
}