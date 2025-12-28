pub const INIT_FILESYSTEM_TABLE: &str = r#"
-- Init the Filesystem table if not exists
DEFINE TABLE IF NOT EXISTS Filesystem SCHEMAFULL;

-- Define fields
DEFINE FIELD OVERWRITE name ON TABLE Filesystem TYPE string;

-- Define unique index on name
DEFINE INDEX OVERWRITE Name ON TABLE Filesystem FIELDS name UNIQUE;
"#;

pub const INIT_NODE_TABLE: &str = r#"
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
DEFINE INDEX OVERWRITE InoUnique ON TABLE Node FIELDS ino UNIQUE;

-- Define other fields
DEFINE FIELD OVERWRITE filesystem ON TABLE Node TYPE string;
DEFINE FIELD OVERWRITE parent ON TABLE Node
  TYPE int
  DEFAULT 1;
DEFINE FIELD OVERWRITE name ON TABLE Node TYPE string;
DEFINE FIELD OVERWRITE kind ON TABLE Node
  TYPE "Folder" | "File"
  DEFAULT "Folder"
  READONLY;
DEFINE FIELD OVERWRITE data ON TABLE Node TYPE option<string>;

-- Define unique index on filesystem + parent + name
DEFINE INDEX OVERWRITE FilesystemParentNameUnique ON TABLE Node FIELDS filesystem, parent, name UNIQUE;
"#;
