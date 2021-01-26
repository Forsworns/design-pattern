use crate::ConfigFile;

#[derive(Clone)]
pub struct ConfigMemento {
    config_file: ConfigFile,
}

impl ConfigMemento {
    pub fn new(config_file: ConfigFile) -> Self {
        Self { config_file }
    }

    pub fn get_config(&self) -> ConfigFile {
        self.config_file.clone()
    }
}
