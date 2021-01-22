#[derive(Default, Debug)]
pub struct AuthInfo {
    code: String,
    info: String,
}

impl AuthInfo {
    pub fn new(code: String, info: String) -> Self {
        Self { code, info }
    }
}

#[macro_export]
macro_rules! auth_info {
    () => {
        $crate::auth_info::AuthInfo {
            ..Default::default()
        }
    };
    ($code:expr, $($info:expr),+ ) => {
        {
            let mut infos = String::new();
            $(
                infos.push_str($info);
            )+
            $crate::auth_info::AuthInfo::new($code.into(), infos)
        }
    };
}
