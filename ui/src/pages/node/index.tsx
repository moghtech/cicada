import { useRead } from "@/lib/hooks";
import { useParams } from "react-router-dom";
import { lazy } from "react";
import { Center, Text } from "@mantine/core";

const Folder = lazy(() => import("@/pages/node/folder"));
const File = lazy(() => import("@/pages/node/file"));

const NodePage = () => {
  const { filesystem, inode: _inode } = useParams() as {
    filesystem: string;
    inode?: string;
  };
  const n_inode = _inode ? Number(_inode) : undefined;
  const inode = n_inode ?? 1;
  const { data: node } = useRead(
    "FindNode",
    { filesystem, inode },
    { enabled: inode > 1 }
  );
  if (inode === 1 || node?.kind === "Folder") {
    return (
      <Folder
        filesystem={filesystem}
        node={node?.inode === inode ? node : undefined}
      />
    );
  } else if (node?.kind === "File") {
    return <File node={node} />;
  } else {
    return (
      <Center>
        <Text size="lg">404: No node found</Text>
      </Center>
    );
  }
};

export default NodePage;
