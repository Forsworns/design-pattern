mod admin;
mod config_file;
mod config_memento;
mod config_originator;

pub use admin::*;
pub use config_file::*;
pub use config_memento::*;
pub use config_originator::*;

use std::collections::HashMap;
use std::time::SystemTime;

#[cfg(test)]
mod tests {
    use crate::*;
    // cargo test -- --nocapture

    #[test]
    fn it_works() {
        let mut admin = Admin::new();
        let mut config_originator = ConfigOriginator::new(ConfigFile::default());
        config_originator.set_config(ConfigFile::new(
            "1000001",
            "configA=v1",
            SystemTime::now(),
            "kyrie",
        ));
        admin.append(config_originator.save_memento());
        config_originator.set_config(ConfigFile::new(
            "1000002",
            "configA=v2",
            SystemTime::now(),
            "kyrie",
        ));
        admin.append(config_originator.save_memento());
        config_originator.set_config(ConfigFile::new(
            "1000003",
            "configA=v3",
            SystemTime::now(),
            "kyrie",
        ));
        admin.append(config_originator.save_memento());
        config_originator.set_config(ConfigFile::new(
            "1000004",
            "configA=v4",
            SystemTime::now(),
            "kyrie",
        ));
        admin.append(config_originator.save_memento());

        config_originator.load_memento(admin.undo());
        println!("memento undo: {:?}", config_originator.get_config());

        config_originator.load_memento(admin.undo());
        println!("memento undo: {:?}", config_originator.get_config());

        config_originator.load_memento(admin.redo());
        println!("memento redo: {:?}", config_originator.get_config());

        config_originator.load_memento(admin.get("1000002"));
        println!("memento redo: {:?}", config_originator.get_config());
    }
}
