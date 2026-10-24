#[cfg(test)]
mod tests {
    use chess_trainer::{app::AppResult, skin::Skins};

    #[test]
    fn load_bundled_skins() -> AppResult<()> {
        let skins: Skins = Skins::bundled()?;

        assert_ne!(skins.names().len(), 0);

        Ok(())
    }

    #[test]
    fn default_skin_exists() -> AppResult<()> {
        let skins: Skins = Skins::bundled()?;

        assert!(skins.get("chessdotcom").is_some());

        Ok(())
    }
}
