# Cicada Loader

Enable applications to load configurations remotely from Cicaca.

## Example

```rust
#[derive(Deserialize)]
struct Env {
  cicada_path: PathBuf,
}

let env = envy::from_env::<Env>()?;

// Load contents from cicada.
// Parses additional `CICADA_...` env vars for config.
let contents = cicada_loader::load(&env.cicada_path)?;

// Parse the file into app configuration
let config = serde_yaml_ng::from_str::<AppConfig>(&contents)?;
```

## Env

```sh
# The address of Cicada Core API.
CICADA_CORE_ADDRESS=https://cicada.example.com
# Mount /config/keys to app container for auth persistence.
CICADA_PRIVATE_KEY=file:/config/keys/cperiphery.key
# Only required on first auth onboard (and persisted keys)
CICADA_ONBOARDING_KEY=O_..._O
CICADA_DEVICE_NAME=My-App
```