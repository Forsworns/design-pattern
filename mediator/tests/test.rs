use mediator::*;

// cargo test -- --nocapture
#[cfg(test)]
#[test]
fn it_works() {
    let builder = SqlSessionFactoryBuilder::build();
    let mut session = builder.open_session();
    let res = select_one!(session, "UserDao.queryUserInfoById", 1u64).unwrap();
    println!("{:?}", res);
}
