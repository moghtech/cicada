use futures_util::{TryStreamExt, stream::FuturesUnordered};

use crate::{
  db::query::secret::list_secrets_matching,
  encryption::decrypt_secret,
};

pub async fn interpolate_secrets(
  contents: String,
) -> mogh_error::Result<String> {
  let var_regex = regex::Regex::new(r"\$\{([A-Za-z0-9_]+)\}")
    // This is guaranteed valid regex
    .unwrap();

  let secret_names = var_regex
    .captures_iter(&contents)
    .map(|caps| caps[1].to_string())
    .collect::<Vec<_>>();

  if secret_names.is_empty() {
    // No captures to interpolate, early return
    return Ok(contents);
  }

  trace!("Captured secret names: {}", secret_names.join(", "));

  let secrets = list_secrets_matching(secret_names)
    .await?
    .into_iter()
    .map(|s| decrypt_secret(s))
    .collect::<FuturesUnordered<_>>()
    .try_collect::<Vec<_>>()
    .await?;

  let res =
    var_regex.replace_all(&contents, |caps: &regex::Captures| {
      secrets
        .iter()
        .find(|s| s.name == &caps[1])
        .and_then(|s| s.data.clone())
        .unwrap_or_default()
    });

  Ok(res.to_string())
}
