use resolver_api::HasResponse;

pub mod device;
pub mod filesystem;
pub mod node;
pub mod onboarding_key;

//
pub trait CicadaWriteRequest: HasResponse {}
