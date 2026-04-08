use futures_util::{TryStreamExt, stream::FuturesUnordered};
use mogh_error::anyhow;

use crate::{
  db::query::secret::list_secrets_matching,
  encryption::decrypt_secret,
};

pub async fn interpolate_secrets(
  contents: String,
) -> mogh_error::Result<String> {
  let var_regex = regex::Regex::new(r"\$(\$)?\{([^}]+)\}")
    // This is guaranteed valid regex
    .unwrap();

  let secret_names = var_regex
    .captures_iter(&contents)
    .filter_map(|caps| {
      // Skip escaped $${...} matches
      if caps.get(1).is_some() {
        return None;
      }
      let m = &caps[2];
      Some(
        m.split_once(':')
          .map(|(name, _)| name)
          .unwrap_or(m)
          .to_string(),
      )
    })
    .collect::<Vec<_>>();

  let has_escapes = var_regex
    .captures_iter(&contents)
    .any(|caps| caps.get(1).is_some());

  if secret_names.is_empty() && !has_escapes {
    // No captures to interpolate, early return
    return Ok(contents);
  }

  trace!("Captured secret names: {}", secret_names.join(", "));

  // This will already early return when secret_names is empty
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
      // Escaped $${...} -> literal ${...}
      if caps.get(1).is_some() {
        trace!("Unescaping variable {}", &caps[2]);
        return format!("${{{}}}", &caps[2]);
      }

      let (secret_name, missing_behavior) =
        caps[2].split_once(':').unwrap_or((&caps[2], ""));

      trace!(
        name = secret_name,
        behavior = missing_behavior,
        "Got secret name and missing behavior"
      );

      if let Some(secret) =
        secrets.iter().find(|s| s.name == secret_name)
      {
        trace!(
          id = secret.id.0,
          "Found secret"
        );
        return secret.data.clone().unwrap_or_default();
      }

      trace!(
        name = secret_name,
        "Did not find matching secret"
      );

      // Handle missing behavior
      if missing_behavior.starts_with('-') {
        trace!(
          name = secret_name,
          "Using 'default' missing behavior"
        );
        // Provide the rest of this value as default
        return missing_behavior[1..].to_string();
      }

      if missing_behavior.starts_with('?') {
        trace!(
          name = secret_name,
          "Using 'error message' missing behavior"
        );
        errors.push(format!(
          "Missing secret '{secret_name}' with message: {}",
          &missing_behavior[1..]
        ));
        return Default::default();
      }

      if missing_behavior.is_empty() {
        trace!(
          name = secret_name,
          "Using 'none' missing behavior"
        );
        errors.push(format!(
          "Missing secret '{secret_name}'"
        ));
        return Default::default();
      }

      trace!(
        name = secret_name,
        behavior = missing_behavior,
        "Got invalid missing behavior"
      );

      errors.push(format!("Missing secret '{secret_name}' with invalid missing behavior: {missing_behavior}"));
      Default::default()
    });

  if errors.is_empty() {
    Ok(res.to_string())
  } else {
    Err(
      anyhow::Error::msg(errors.join(" | "))
        .context("Interpolation failed")
        .into(),
    )
  }
}
