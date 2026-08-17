use alloc::string::String;

pub enum RouteTarget {
    Hacedle,
    Hacetral,
    Hacetime,
    Haceto,
}

pub struct BrainRouter;

impl BrainRouter {
    pub fn route(sio: &crate::RuntimeSio) -> RouteTarget {
        match sio.runtime.provider.as_str() {
            "hacetral" => RouteTarget::Hacetral,
            "hacetime" => RouteTarget::Hacetime,
            "haceto" => RouteTarget::Haceto,
            _ => RouteTarget::Hacedle,
        }
    }
}