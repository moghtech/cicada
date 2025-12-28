use std::sync::LazyLock;

use anyhow::Context as _;
use surrealdb::{Surreal, engine::any::Any, opt::auth};

pub static DB: LazyLock<Surreal<Any>> = LazyLock::new(Surreal::init);

pub async fn init() -> anyhow::Result<()> {
  DB.connect("wss://surreal.van")
    .await
    .context("Failed to connect to database")?;

  DB.signin(auth::Root {
    username: String::from("root"),
    password: String::from("root"),
  })
  .await?;

  DB.use_ns("cicada").use_db("cicada").await?;

  init_node_table().await?;

  Ok(())
}

async fn init_node_table() -> anyhow::Result<()> {
  DB.query(
    r#"
-- Init the Node table if not exists
DEFINE TABLE IF NOT EXISTS Node SCHEMAFULL;

-- Create inode allocator function.
DEFINE FUNCTION OVERWRITE fn::node_next_ino() {
  LET $max = (SELECT math::max(ino) AS max_ino FROM Node GROUP ALL)[0].max_ino;
  RETURN IF $max = NONE OR $max = NULL {
    2
  } ELSE {
    $max + 1
  };
};
-- Define the ino field and unique index
DEFINE FIELD OVERWRITE ino ON TABLE Node
  TYPE int
  DEFAULT fn::node_next_ino()
  READONLY;
DEFINE INDEX OVERWRITE NodeInoUnique ON TABLE Node FIELDS ino UNIQUE;

-- Define other fields
DEFINE FIELD OVERWRITE parent ON TABLE Node
  TYPE int
  DEFAULT 1;
DEFINE FIELD OVERWRITE name ON TABLE Node TYPE string;
DEFINE FIELD OVERWRITE kind ON TABLE Node
  TYPE "Folder" | "File"
  DEFAULT "Folder"
  READONLY;
DEFINE FIELD OVERWRITE data ON TABLE Node TYPE option<string>;

-- Define unique index on parent + name
DEFINE INDEX OVERWRITE NodeParentNameUnique ON TABLE Node FIELDS parent, name UNIQUE;
"#,
)
.await
.context("Failed to initialize node table")?;

  Ok(())
}
