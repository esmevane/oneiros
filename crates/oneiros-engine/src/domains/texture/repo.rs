use rusqlite::params;

use crate::*;

pub struct TextureRepo<'a> {
    context: &'a ProjectContext,
}

impl<'a> TextureRepo<'a> {
    pub fn new(context: &'a ProjectContext) -> Self {
        Self { context }
    }

    pub async fn get(&self, name: &TextureName) -> Result<Option<Texture>, EventError> {
        let db = self.context.db()?;
        let mut stmt =
            db.prepare("SELECT name, description, prompt FROM textures WHERE name = ?1")?;

        let result = stmt.query_row(params![name.to_string()], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        });

        match result {
            Ok((name, description, prompt)) => Ok(Some(
                Texture::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build(),
            )),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn list(&self) -> Result<Vec<Texture>, EventError> {
        let db = self.context.db()?;
        let mut stmt =
            db.prepare("SELECT name, description, prompt FROM textures ORDER BY name")?;

        let textures = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })?
            .collect::<Result<Vec<(String, String, String)>, _>>()?
            .into_iter()
            .map(|(name, description, prompt)| {
                Texture::builder()
                    .name(name)
                    .description(description)
                    .prompt(prompt)
                    .build()
            })
            .collect();

        Ok(textures)
    }
}
