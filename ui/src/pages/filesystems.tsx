import { useRead } from "@/lib/hooks";

const FilesystemsPage = () => {
  const { data } = useRead("ListFilesystems", {});
  return <>Filesystems: {data?.map((fs) => fs.name).join(", ")}</>;
};

export default FilesystemsPage;
