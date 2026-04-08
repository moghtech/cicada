use futures_util::{TryStreamExt, stream::FuturesUnordered};
use mogh_error::anyhow::anyhow;

use crate::{
  db::query::secret::list_secrets_matching,
  encryption::decrypt_secret,
};

pub async fn interpolate_secrets(
  contents: String,
) -> mogh_error::Result<String> {
  let var_regex = regex::Regex::new(r"\$\{([A-Za-z0-9_\-\:]+)\}")
    // This is guaranteed valid regex
    .unwrap();

  let secret_names = var_regex
    .captures_iter(&contents)
    .map(|caps| {
      caps[1]
        .split_once(':')
        .map(|(name, _)| name)
        .unwrap_or(&caps[1])
        .to_string()
    })
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

  let mut errors = Vec::new();

  let res =
    var_regex.replace_all(&contents, |caps: &regex::Captures| {
      let (secret_name, missing_behavior) =
        caps[1].split_once(':').unwrap_or((&caps[1], ""));

      if let Some(secret) =
        secrets.iter().find(|s| s.name == secret_name)
      {
        return secret.data.clone().unwrap_or_default();
      }

      // Handle missing behavior
      if missing_behavior.starts_with('-') {
        // Provide the rest of this value as default
        return missing_behavior[1..].to_string();
      }

      if missing_behavior.starts_with('?') {
        errors.push(format!(
          "Missing secret {secret_name}: {}",
          &missing_behavior[1..]
        ));
        return Default::default();
      }

      if missing_behavior.is_empty() {
        errors.push(format!(
          "Missing secret {secret_name}"
        ));
        return Default::default();
      }

      errors.push(format!("Missing secret {secret_name} with invalid missing behavior: {missing_behavior}"));
      Default::default()
    });

  if errors.is_empty() {
    Ok(res.to_string())
  } else {
    Err(
      anyhow!("Interpolation failed | {}", errors.join(" | ")).into(),
    )
  }
}
