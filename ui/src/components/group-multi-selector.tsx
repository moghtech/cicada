import { useRead } from "@/lib/hooks";
import { TagsInput, TagsInputProps } from "@mantine/core";

export interface GroupMultiSelectorProps
  extends Omit<TagsInputProps, "data"> {}

export default function GroupMultiSelector(props: GroupMultiSelectorProps) {
  const { data: groups } = useRead("ListGroups", {});
  return (
    <TagsInput
      placeholder="Select or create groups"
      data={groups?.map((g) => g.name) ?? []}
      styles={{ inputField: { width: 150 } }}
      miw="max-content"
      clearable
      {...props}
    />
  );
}
