import { useRead } from "@/lib/hooks";
import { MultiSelect, MultiSelectProps } from "@mantine/core";

export interface FilesystemMultiSelectorProps extends Omit<
  MultiSelectProps,
  "data"
> {}

export default function FilesystemMultiSelector(
  props: FilesystemMultiSelectorProps,
) {
  const { data: filesystems } = useRead("ListFilesystems", {});
  return (
    <MultiSelect
      placeholder="Select filesystems"
      data={filesystems?.map((f) => ({ value: f.id, label: f.name })) ?? []}
      styles={{ inputField: { width: 120 } }}
      miw="max-content"
      searchable
      clearable
      {...props}
    />
  );
}
