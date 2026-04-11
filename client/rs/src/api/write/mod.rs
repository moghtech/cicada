use mogh_resolver::HasResponse;

mod checkpoint;
mod device;
mod encryption_key;
mod filesystem;
mod node;
mod onboarding_key;
mod policy;
mod secret;
mod user;

pub use checkpoint::*;
pub use device::*;
pub use encryption_key::*;
pub use filesystem::*;
pub use node::*;
pub use onboarding_key::*;
pub use policy::*;
pub use secret::*;
pub use user::*;

//
pub trait CicadaWriteRequest: HasResponse {}
