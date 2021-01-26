// persistence object
use crate::SystemTime;

pub struct User{
    id: u64,
    name: String,
    age: u8,
    create_time: SystemTime,
    update_time:SystemTime,
}

impl User{
    pub fn new(id: u64,
        name: String,
        age: u8,
        create_time: SystemTime,
        update_time:SystemTime)->Self{
        Self{
            id, name, age, create_time, update_time
        }
    }
}

pub struct School{
    id: u64,
    name: String,
    address: String,
    create_time: SystemTime,
    update_time:SystemTime,
}

impl School{
    pub fn new(id: u64,
        name: String,
        address: String,
        create_time: SystemTime,
        update_time:SystemTime)->Self{
        Self{
            id, name, address, create_time, update_time
        }
    }
}
