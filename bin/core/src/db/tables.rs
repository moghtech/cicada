pub const INIT_TABLES: &str = r#"
-- ============
--  Filesystem
-- ============

-- Init the Filesystem table if not exists
DEFINE TABLE IF NOT EXISTS Filesystem SCHEMAFULL;

-- Define fields
DEFINE FIELD OVERWRITE name ON TABLE Filesystem TYPE string;

-- Define unique index on name
DEFINE INDEX OVERWRITE Name ON TABLE Filesystem FIELDS name UNIQUE;

DEFINE FUNCTION OVERWRITE fn::first_filesystem() {
  RETURN (SELECT id FROM Filesystem LIMIT 1)[0].id
};

-- ======
--  Node
-- ======

-- Init the Node table if not exists
DEFINE TABLE IF NOT EXISTS Node SCHEMAFULL;

-- Create node creation function.
DEFINE FUNCTION OVERWRITE fn::create_node($data: object) {
  LET $max = (SELECT math::max(record::id(id)) AS max_ino FROM Node GROUP ALL)[0].max_ino;
  LET $ino = IF $max = NONE OR $max = NULL {
    2
  } ELSE {
    $max + 1
  };
  RETURN CREATE type::record("Node", $ino) CONTENT $data;
};

-- Define other fields
DEFINE FIELD OVERWRITE filesystem ON TABLE Node
  TYPE record<Filesystem>
  DEFAULT fn::first_filesystem();
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
