import { useInvalidate, useWrite } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { ActionIcon, Text, TextInput } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { Types } from "cicada_client";
import { Save, X } from "lucide-react";
import { useState } from "react";

export const NodePageTitle = ({
  node,
}: {
  node: Types.NodeEntity | undefined;
}) => {
  const kind = node?.kind ?? "Folder";
  const Icon = ICONS[kind];
  const [name, setName] = useState<string | null>(null);
  const inv = useInvalidate();
  const { mutate: updateNode } = useWrite("UpdateNode", {
    onSuccess: () => {
      inv(["FindNode"]);
      notifications.show({ message: `${kind} name updated` });
      setName(null);
    },
  });
  return (
    <>
      <Icon size={22} opacity={0.6} />
      <Text fz="h2" opacity={0.6}>
        {kind}:
      </Text>
      {name === null ? (
        <Text
          fz="h2"
          style={{ cursor: node && "pointer" }}
          onClick={() => node && setName(node.name)}
        >
          {node?.name ?? "Root"}
        </Text>
      ) : (
        <>
          <TextInput
            size="lg"
            value={name}
            onChange={(e) => setName(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Escape") {
                setName(null);
              } else if (e.key === "Enter") {
                e.preventDefault();
                name && node && updateNode({ id: node.id, name });
              }
            }}
            autoFocus
          />
          <ActionIcon
            onClick={() => name && node && updateNode({ id: node.id, name })}
            disabled={!name || name === node?.name}
          >
            <Save size="1rem" />
          </ActionIcon>
          <ActionIcon color="red" onClick={() => setName(null)}>
            <X size="1rem" />
          </ActionIcon>
        </>
      )}
    </>
  );
};

export const NodePageDescription = ({
  filesystem,
}: {
  filesystem: Types.FilesystemRecord | undefined;
}) => {
  return (
    <>
      <ICONS.Filesystem size="1.1rem" opacity={0.6} />
      <Text opacity={0.6} size="lg">
        Filesystem:
      </Text>
      <Text size="lg">{filesystem?.name}</Text>
    </>
  );
};
