use mogh_resolver::HasResponse;

pub mod device;
pub mod encryption_key;
pub mod filesystem;
pub mod node;
pub mod onboarding_key;
pub mod secret;

//
pub trait CicadaWriteRequest: HasResponse {}
