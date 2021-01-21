pub trait Message: std::fmt::Debug {
    fn content(&self) {
        println!("This is a {:?} message.", self);
    }
}
#[derive(Debug)]
pub struct News {}
impl Message for News {}

#[derive(Debug)]
pub struct Mail {}
impl Message for Mail {}
#[derive(Debug)]
pub struct Post {}
impl Message for Post {}

pub struct MessageDispatcher {
    mail: Mail,
    news: News,
    post: Post,
}

impl MessageDispatcher {
    pub fn new() -> Self {
        Self {
            mail: Mail {},
            news: News {},
            post: Post {},
        }
    }
    pub fn report_mail(&self) {
        self.mail.content();
    }
    pub fn report_news(&self) {
        self.news.content();
    }
    pub fn report_post(&self) {
        self.post.content();
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    // cargo test -- --nocapture
    #[test]
    fn it_works() {
        let facade = MessageDispatcher::new();
        facade.report_news();
        facade.report_mail();
        facade.report_post();
    }
}
