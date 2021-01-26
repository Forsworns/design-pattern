use crate::{ConfigFile, ConfigMemento};

pub struct ConfigOriginator {
    config_file: ConfigFile,
}

impl ConfigOriginator {
    pub fn new(config_file: ConfigFile) -> Self {
        Self { config_file }
    }

    pub fn save_memento(&self) -> ConfigMemento {
        ConfigMemento::new(self.config_file.clone())
    }

    pub fn load_memento(&mut self, mem: ConfigMemento) {
        self.config_file = mem.get_config();
    }

    pub fn set_config(&mut self, config_file: ConfigFile) {
        self.config_file = config_file;
    }

    pub fn get_config(&self) -> ConfigFile {
        self.config_file.clone()
    }
}
