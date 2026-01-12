use resolver_api::HasResponse;

pub mod device;
pub mod filesystem;
pub mod node;

//
pub trait CicadaWriteRequest: HasResponse {}
