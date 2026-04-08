import { useRead } from "@/lib/hooks";
import { useParams } from "react-router-dom";
import { lazy } from "react";
import { Center, Text } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { EnableSwitch } from "mogh_ui";

const Folder = lazy(() => import("@/pages/node/folder"));
const File = lazy(() => import("@/pages/node/file"));

const NodePage = () => {
  const { filesystem: _filesystem, inode: _inode } = useParams() as {
    filesystem: string;
    inode?: string;
  };
  const n_inode = _inode ? Number(_inode) : undefined;
  const inode = n_inode ?? 1;

  const [interpolated, { toggle }] = useDisclosure();

  const { data: node, error: nodeError } = useRead(
    "FindNode",
    { filesystem: _filesystem, inode, interpolated },
    { enabled: inode > 1 },
  );
  const filesystem = useRead("ListFilesystems", {}).data?.find(
    (fs) => fs.id === _filesystem,
  );

  if (inode === 1 || node?.kind === "Folder") {
    return (
      <Folder
        filesystem={filesystem}
        node={node?.inode === inode ? node : undefined}
      />
    );
  } else if (node?.kind === "File" || nodeError) {
    return (
      <File
        filesystem={filesystem}
        node={node}
        nodeError={nodeError as { result?: unknown }}
        toggleInterpolation={
          <EnableSwitch
            label="Interpolation"
            checked={interpolated}
            onCheckedChange={toggle}
          />
        }
      />
    );
  } else {
    return (
      <Center>
        <Text size="lg">404: No node found</Text>
      </Center>
    );
  }
};

export default NodePage;
