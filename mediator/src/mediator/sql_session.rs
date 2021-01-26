use crate::{HashMap,Conn, BufReader};

pub struct XNode{
    namespace: String,
    id: String,
    param_type: String,
    res_type: String,
    sql: String,
    params: HashMap<u64, String>,
}

impl XNode{
    pub fn new(namespace: String,
        id: String,
        param_type: String,
        res_type: String,
        sql: String,
        params: HashMap<u64, String>,)->Self{
        Self{
            namespace,
            id,
            param_type,
            res_type,
            sql,
            params
        }
    }
}

pub trait SqlSessionTrait {
    fn select_one<T>(statement: String) -> Result<T,String>;
    fn select_one_with_params<T>(statement: String, parameter: String) -> Result<T,String>;
    fn select_list<T>(statement: String) -> Result<Vec<T>,String>;
    fn select_list_with_params<T>(statement: String, parameter: String) -> Result<Vec<T>,String>;
    fn close()->Result<(),String> ;
}

struct DefaultSqlSession {
    conn: Conn,
    mapper: HashMap<String, XNode>,
}


impl DefaultSqlSession {
    pub fn new(conn: Conn, mapper: HashMap<String, XNode>)->Self{
        Self{
            conn,
            mapper
        }
    }
}



impl SqlSessionTrait for DefaultSqlSession {
    fn select_one<T>(statement: String) -> Result<T, String> {}

    fn select_one_with_params<T>(statement: String, parameter: String) -> Result<T, String> {}

    fn select_list<T>(statement: String) -> Result<Vec<T>,String> {}
    fn select_list_with_params<T>(statement: String, parameter: String) -> Result<Vec<T>,String> {}
    fn close()->Result<(),String> {}
}


pub trait SqlSessionFactoryTrait{
    fn open_session<T:SqlSessionTrait>(&self)->T;
}


pub struct DefaultSqlSessionFactory{
    config:SqlConfiguration
}

impl DefaultSqlSessionFactory{
    pub fn new(config:SqlConfiguration)->Self{
        Self{
            config
        }
    }
}

impl SqlSessionFactoryTrait for DefaultSqlSessionFactory{
    fn open_session(&self)->DefaultSqlSession{
        DefaultSqlSession::new(self.config.conn,self.config.mapper_element)
    }
}

pub struct SqlSessionFactoryBuilder{}

impl SqlSessionFactoryBuilder{
    pub fn build(reader:BufReader)->DefaultSqlSessionFactory{}
}


pub struct SqlConfiguration{
    pub conn: Conn,
    pub data_source: HashMap<String,String>,
    pub mapper_element: HashMap<String, XNode>,
}

impl SqlConfiguration{
    pub fn new(conn: Conn,
        data_source: HashMap<String,String>,
        mapper_element: HashMap<String, XNode>)->Self{
        Self{
            conn,
            data_source,
            mapper_element,
        }
    }
}