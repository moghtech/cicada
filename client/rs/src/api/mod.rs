//! # Cicada Core API
//!
//! Cicada Core exposes an HTTP api using standard JSON serialization.
//!
//! All calls share some common HTTP params:
//! - Method: `POST`
//! - Path: `/auth`, `/read`, `/write`
//! - Headers:
//!   - Content-Type: `application/json`
//!   - Authorization: `your_jwt`
//!   - X-Api-Key: `your_api_key`
//!   - X-Api-Secret: `your_api_secret`
//!   - Use either Authorization *or* X-Api-Key and X-Api-Secret to authenticate requests.
//! - Body: JSON specifying the request type (`type`) and the parameters (`params`).
//!
//! You can create API keys on the user profile page.
//!
//! To call the api, construct JSON bodies following
//! the schemas in [read] and [mod@write].
//!
//! For example, this is an example body for [read::GetFilesystem]:
//! ```json
//! {
//!   "id": "my-filesystem"
//! }
//! ```
//!
//! The request's parent module (eg. [read], [mod@write]) and name determines the http path which
//! must be used for the requests. For example, requests under [read] are made using http path `/read/{REQUEST_NAME}`.
//!
//! ## Curl Example
//!
//! Putting it all together, here is an example `curl` for [write::CreateFilesystem]:
//!
//! ```text
//! curl --header "Content-Type: application/json" \
//!     --header "X-Api-Key: your_api_key" \
//!     --header "X-Api-Secret: your_api_secret" \
//!     --data '{ "name": "new-filesystem" }' \
//!     https://cicada.example.com/write/CreateFilesystem
//! ```
//!
//! ## Modules
//!
//! - [auth]: Requests relating to logging in / obtaining authentication tokens.
//! - [read]: Read only requests which retrieve data from Cicada.
//! - [mod@write]: Requests which alter data, like create / update / delete resources.
//!
//! ## Errors
//!
//! Request errors will be returned with a JSON body containing information about the error.
//! They will have the following common format:
//! ```json
//! {
//!   "error": "top level error message",
//!   "trace": [
//!     "first traceback message",
//!     "second traceback message"
//!   ]
//! }
//! ```

pub mod read;
pub mod write;

pub mod auth {
  pub use mogh_auth_client::api::*;
}
