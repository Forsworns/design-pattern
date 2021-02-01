use crate::{
    BufReader, EventReader, File, HashMap, OptsBuilder, Path, Pool, PooledConn, Regex, Row,
    XmlEvent,
};
// use std::any::Any;

#[derive(Clone, Debug, Default)]
pub struct XNode {
    namespace: String,
    id: String,
    param_type: String,
    res_type: String,
    pub sql: String,
    params: Vec<String>,
}

impl XNode {
    pub fn new(
        namespace: String,
        id: String,
        param_type: String,
        res_type: String,
        sql: String,
        params: Vec<String>,
    ) -> Self {
        Self {
            namespace,
            id,
            param_type,
            res_type,
            sql,
            params,
        }
    }
}

pub trait SqlSessionTrait {
    // fn select_one(&self, statement: String, params: Vec<&dyn Any>) -> Result<Row, String>;
    fn select_one(&self, statement: String, params: Vec<String>) -> Result<Row, String>;
}

#[derive(Debug)]
pub struct DefaultSqlSession {
    pub conn: PooledConn,
    pub mapper: HashMap<String, XNode>,
}

impl DefaultSqlSession {
    pub fn new(conn: PooledConn, mapper: HashMap<String, XNode>) -> Self {
        Self { conn, mapper }
    }
}

#[macro_export]
macro_rules! select_one {
    ($session:expr, $statement:expr, $($param:expr)*) => {{
        let mut sql = $session.mapper.get($statement).unwrap().sql.clone();
        $(
            // sql = rt_format!(sql {,param}*); // `runtime_fmt` on crate.io
            sql = sql.replacen("{}", &format!("{}", $param), 1);
        )*
        $session.conn.query::<Row,_>(sql)
    }};
}

impl SqlSessionTrait for DefaultSqlSession {
    fn select_one(&self, _statement: String, _params: Vec<String>) -> Result<Row, String> {
        Err("Not simple as the the macro above, but can be implemented inside the trait".into())
    }
}

pub trait SqlSessionFactoryTrait {
    type Session: SqlSessionTrait;
    fn open_session(&self) -> Self::Session;
}

#[derive(Debug)]
pub struct DefaultSqlSessionFactory {
    config: SqlConfiguration,
}

impl DefaultSqlSessionFactory {
    pub fn new(config: SqlConfiguration) -> Self {
        Self { config }
    }
}

impl SqlSessionFactoryTrait for DefaultSqlSessionFactory {
    type Session = DefaultSqlSession;
    fn open_session(&self) -> DefaultSqlSession {
        DefaultSqlSession::new(
            self.config.pool.get_conn().unwrap(),
            self.config.mapper_element.clone(),
        )
    }
}

#[derive(Debug)]
pub struct SqlSessionFactoryBuilder {}

impl SqlSessionFactoryBuilder {
    pub fn build() -> DefaultSqlSessionFactory {
        let config = Self::parse_configuration();
        DefaultSqlSessionFactory::new(config)
    }
    fn parse_configuration() -> SqlConfiguration {
        let file = File::open(Path::new("resources/Config.xml")).unwrap();
        let file = BufReader::new(file);
        let parser = EventReader::new(file);
        let mut configs = HashMap::<String, String>::new();
        let mut mappers = Vec::new();
        // start parsing
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.local_name == String::from("property") {
                        let key = attributes[0].value.clone();
                        let val = attributes[1].value.clone();
                        configs.insert(key, val);
                    } else if name.local_name == String::from("mapper") {
                        let mapper = attributes[0].value.clone();
                        mappers.push(mapper);
                    }
                }
                Err(e) => {
                    panic!("Error: {}", e);
                }
                _ => {}
            }
        }
        let data_source = OptsBuilder::new().from_hash_map(&configs).unwrap();
        // println!("{:?}", data_source);
        let pool = Pool::new(data_source).unwrap();
        SqlConfiguration::new(
            pool,
            OptsBuilder::new()
                .user(Some(configs.get("user").unwrap()))
                .db_name(Some(configs.get("db_name").unwrap())),
            Self::parse_mapper(mappers),
        )
    }
    fn parse_mapper(mappers: Vec<String>) -> HashMap<String, XNode> {
        let mut elements = HashMap::<String, XNode>::new();
        for path in mappers {
            let file = File::open(Path::new(&path)).unwrap();
            let file = BufReader::new(file);
            let parser = EventReader::new(file);
            let mut namespace = String::new();
            let mut id = String::new();
            let mut param_type = String::new();
            let mut res_type = String::new();
            // start parsing
            for e in parser {
                match e {
                    Ok(XmlEvent::StartElement {
                        name, attributes, ..
                    }) => {
                        if name.local_name == String::from("mapper") {
                            namespace = attributes[0].value.clone();
                        } else if name.local_name == String::from("select") {
                            id = attributes[0].value.clone();
                            param_type = attributes[1].value.clone();
                            res_type = attributes[2].value.clone();
                        }
                    }
                    Ok(XmlEvent::Characters(sql)) => {
                        let mut temp_sql = sql.clone();
                        let re = Regex::new(r"(#\{(.*?)\})").unwrap();
                        let mut params = Vec::new();
                        // the entire match is stored in the capture group at index 0
                        for cap in re.captures_iter(&sql) {
                            params.push(cap[2].into());
                            temp_sql = temp_sql.replace(&cap[1], "{}");
                        }
                        let xnode = XNode::new(
                            namespace.clone(),
                            id.clone(),
                            param_type.clone(),
                            res_type.clone(),
                            temp_sql,
                            params,
                        );
                        elements.insert(format!("{}.{}", namespace, id), xnode);
                    }
                    Err(e) => {
                        panic!("Error: {}", e);
                    }
                    _ => {}
                }
            }
        }
        elements
    }
}

#[derive(Debug)]
pub struct SqlConfiguration {
    pub pool: Pool,
    pub data_source: OptsBuilder,
    pub mapper_element: HashMap<String, XNode>,
}

impl SqlConfiguration {
    pub fn new(
        pool: Pool,
        data_source: OptsBuilder,
        mapper_element: HashMap<String, XNode>,
    ) -> Self {
        Self {
            pool,
            data_source,
            mapper_element,
        }
    }
}
