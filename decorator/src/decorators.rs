use crate::{HandlerInterceptor, SsoInterceptor, AUTH_MAP};
struct SsoDecorator<T = SsoInterceptor>
where
    T: HandlerInterceptor,
{
    handler_interceptor: T,
}

impl<T: HandlerInterceptor> SsoDecorator<T> {
    pub fn new(handler_interceptor: T) -> Self {
        Self {
            handler_interceptor,
        }
    }

    pub fn pre_handle(
        // have to call the handler_interceptor.pre_handle to overwrite it
        &self,
        request: &String,
        response: &String,
        handler: fn(String, String) -> bool,
    ) -> bool {
        self.handler_interceptor
            .pre_handle(request, response, handler)
    }
}

pub struct LoginSsoDecorator<T = SsoInterceptor>
where
    T: HandlerInterceptor,
{
    sso_decorator: SsoDecorator<T>, // inherient by combination...
}

impl<T: HandlerInterceptor> LoginSsoDecorator<T> {
    pub fn new(handler_interceptor: T) -> Self {
        let sso_decorator = SsoDecorator::new(handler_interceptor);
        Self { sso_decorator }
    }

    pub fn pre_handle(
        &self,
        request: &String,
        response: &String,
        handler: fn(String, String) -> bool,
    ) -> bool {
        let success = self.sso_decorator.pre_handle(request, response, handler);
        if !success {
            return success;
        }
        let user = &request[8..];
        let methods = AUTH_MAP.lock().unwrap();
        let method = methods.get(user).unwrap();
        "queryUserInfo".to_owned() == *method
    }
}
