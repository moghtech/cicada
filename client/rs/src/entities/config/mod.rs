pub mod core;
pub mod logger;
pub mod periphery;

fn default_config_keywords() -> Vec<String> {
  vec![String::from("*config.*")]
}

fn default_merge_nested_config() -> bool {
  true
}

fn default_extend_config_arrays() -> bool {
  true
}
