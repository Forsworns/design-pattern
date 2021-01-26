use crate::{ConfigMemento, HashMap};

pub struct Admin {
    cursor_idx: i32,
    mementos: Vec<ConfigMemento>,
    mementos_map: HashMap<String, ConfigMemento>,
}

impl Admin {
    pub fn new() -> Self {
        Self {
            cursor_idx: 0,
            mementos: vec![],
            mementos_map: HashMap::new(),
        }
    }

    pub fn append(&mut self, m: ConfigMemento) {
        self.mementos.push(m.clone());
        self.mementos_map.insert(m.get_config().version_no, m);
        self.cursor_idx += 1;
    }

    pub fn undo(&mut self) -> ConfigMemento {
        self.cursor_idx -= 1;
        if self.cursor_idx < 0 {
            self.mementos.get(0).unwrap().clone()
        } else {
            self.mementos.get(self.cursor_idx as usize).unwrap().clone()
        }
    }

    pub fn redo(&mut self) -> ConfigMemento {
        self.cursor_idx += 1;
        if self.cursor_idx as usize > self.mementos.len() {
            self.mementos.get(self.mementos.len() - 1).unwrap().clone()
        } else {
            self.mementos.get(self.cursor_idx as usize).unwrap().clone()
        }
    }

    pub fn get(&self, s: &str) -> ConfigMemento {
        self.mementos_map.get(s).unwrap().clone()
    }
}
