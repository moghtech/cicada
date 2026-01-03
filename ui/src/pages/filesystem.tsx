import { useRead } from "@/lib/hooks";
import { useParams } from "react-router-dom";

const FilesystemPage = () => {
  const { filesystem, parent } = useParams() as {
    filesystem: string;
    parent?: string;
  };
  const fs = useRead("ListFilesystems", {}).data?.find(
    (fs) => fs.id === filesystem
  );
  const { data: nodes } = useRead("ListNodes", {
    filesystem,
    parent: parent && Number(parent) ? Number(parent) : undefined,
  });
  return (
    <div>
      Filesystem {fs?.name ?? "Unknown"} |
      {nodes?.map((node) => (
        <div>{node.name}</div>
      ))}
    </div>
  );
};

export default FilesystemPage;
