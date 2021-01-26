use crate::HashMap;

#[derive(Clone, Debug)]
pub struct Employee {
    pub u_id: String,
    name: String,
    desc: String,
}

impl Employee {
    pub fn new(u_id: &'static str, name: &'static str, desc: &'static str) -> Self {
        Self {
            u_id: String::from(u_id),
            name: String::from(name),
            desc: String::from(desc),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Link {
    pub from_id: String,
    pub to_id: String,
}

impl Link {
    pub fn new(from_id: &'static str, to_id: &'static str) -> Self {
        Self {
            from_id: String::from(from_id),
            to_id: String::from(to_id),
        }
    }
}

pub struct Group {
    id: String,
    name: String,
    from_id: String,
    to_id: String,
    employee_map: HashMap<String, Employee>,
    link_map: HashMap<String, Vec<Link>>,
    inverted_map: HashMap<String, String>,
    key_map: HashMap<String, usize>,
}

impl Group {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id: id.clone(),
            name,
            from_id: id.clone(),
            to_id: id,
            employee_map: HashMap::new(),
            link_map: HashMap::new(),
            inverted_map: HashMap::new(),
            key_map: HashMap::new(),
        }
    }

    pub fn add(&mut self, employee: Employee) -> bool {
        self.employee_map
            .insert(employee.u_id.clone(), employee)
            .is_none()
    }

    pub fn add_link(&mut self, key: &'static str, link: Link) {
        self.inverted_map
            .insert(link.to_id.clone(), link.from_id.clone());
        if self.link_map.contains_key(key) {
            let links = self.link_map.get_mut(key).unwrap();
            links.push(link);
        } else {
            let mut links = Vec::<Link>::new();
            links.push(link);
            self.link_map.insert(String::from(key), links);
        }
    }

    pub fn get_curr_idx(&mut self, key: &String) -> usize {
        let mut idx: usize = 0;
        if self.key_map.contains_key(key) {
            idx = *self.key_map.get(key).unwrap();
            idx += 1;
            self.key_map.insert(key.clone(), idx);
        } else {
            self.key_map.insert(key.clone(), idx);
        }
        idx
    }
}

impl Iterator for Group {
    type Item = Employee;

    fn next(&mut self) -> Option<Self::Item> {
        let link_map = self.link_map.clone();
        let mut links = link_map.get(&self.to_id);
        let mut id = self.to_id.clone();
        let mut curr_idx = self.get_curr_idx(&id);
        if links.is_none() {
            // select next sibling
            id = self.from_id.clone();
            curr_idx = self.get_curr_idx(&id);
            links = link_map.get(&id);
        }

        let mut links = links.unwrap();
        while curr_idx > links.len() - 1 {
            // have reached the last children of current parent, have to roll back to the upper level ancestors
            let parent = self.inverted_map.get(&self.from_id);
            if parent.is_none() {
                // root doestn't owns a parent
                return None;
            }
            self.from_id = parent.unwrap().clone();
            id = self.from_id.clone();
            curr_idx = self.get_curr_idx(&id);
            links = self.link_map.get_mut(&id).unwrap();
        }

        let link = &links[curr_idx];
        self.to_id = link.to_id.clone();
        self.from_id = link.from_id.clone();

        Some(self.employee_map.get(&self.to_id).unwrap().clone())
    }
}
