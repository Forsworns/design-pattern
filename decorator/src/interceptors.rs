pub trait HandlerInterceptor {
    fn pre_handle(
        &self,
        request: &String,
        response: &String,
        handler: fn(String, String) -> bool,
    ) -> bool;
}

pub struct SsoInterceptor {}

impl HandlerInterceptor for SsoInterceptor {
    fn pre_handle(
        &self,
        request: &String,
        _response: &String,
        _handler: fn(String, String) -> bool,
    ) -> bool {
        let ticket = &request[1..8];
        ticket == "success"
    }
}
