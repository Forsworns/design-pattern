use crate::{auth_info, AuthAppendTrait, AuthInfo, AuthLinkTrait, AuthService, SystemTime};

#[derive(Clone)]
pub struct Level2AuthLink {
    level_user_id: String,
    level_user_name: String,
    next: Option<Box<dyn AuthLinkTrait>>,
}

impl Level2AuthLink {
    pub fn new(level_user_id: String, level_user_name: String) -> Self {
        Self {
            level_user_id,
            level_user_name,
            next: None,
        }
    }
}

impl AuthLinkTrait for Level2AuthLink {
    fn do_auth(&self, u_id: String, order_id: String, auth_date: SystemTime) -> AuthInfo {
        if let Some(date) = AuthService::query_auth_info(&self.level_user_id, &self.level_user_name)
        {
            if let Some(next) = self.next() {
                (*next).do_auth(u_id, order_id, auth_date)
            } else {
                // no more auth levels
                auth_info!(
                    "0000, ",
                    "order_id: ",
                    &order_id,
                    ", status: get the second auth from ",
                    &self.level_user_name,
                    " at: ",
                    &format!("{:?}", date)
                )
            }
        } else {
            auth_info!(
                "0001, ",
                "order_id: ",
                &order_id,
                ", status: wait for the second auth from ",
                &self.level_user_name
            )
        }
    }

    fn next(&self) -> Option<Box<dyn AuthLinkTrait>> {
        self.next.clone()
    }
}

impl AuthAppendTrait for Level2AuthLink {
    fn append_next(&mut self, next: Box<dyn AuthLinkTrait>) -> Box<dyn AuthLinkTrait> {
        self.next = Some(next.clone());
        Box::new(self.clone())
    }
}
