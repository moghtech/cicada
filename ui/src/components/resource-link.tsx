import { useRead } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { Group } from "@mantine/core";
import { Types } from "cicada_client";
import { hexColorByIntention } from "mogh_ui";
import { Link } from "react-router-dom";

export type ResourceType = "EncryptionKey" | "Secret" | "Filesystem" | "Device";

export interface ResourceLinkProps {
  type: ResourceType;
  id: string;
}

function link(type: ResourceType, id: string) {
  switch (type) {
    case "EncryptionKey":
      return "/encryption-keys/" + id;
    case "Secret":
      return "/secrets/" + id;
    case "Filesystem":
      return "/filesystems/" + id;
    case "Device":
      return "/device/" + id;
  }
}

export default function ResourceLink({ type, id }: ResourceLinkProps) {
  const resource = useRead(`List${type}s`, {}).data?.find((r) => r.id === id);
  const Icon = ICONS[type];
  const intention =
    type === "EncryptionKey"
      ? (resource as Types.EncryptionKeyEntity)?.initialized
        ? "Good"
        : "Critical"
      : type === "Secret"
        ? (resource as Types.SecretEntity)?.encryption_key &&
          (resource as Types.SecretEntity)?.data === null
          ? "Critical"
          : !(resource as Types.SecretEntity)?.data
            ? "Neutral"
            : "Good"
        : ["Filesystem", "User", "Device"].includes(type)
          ? "Good"
          : "None";
  return (
    <Group
      renderRoot={(props) => <Link to={link(type, id)} {...props} />}
      gap="xs"
      wrap="nowrap"
      w="fit-content"
      className="hover-underline"
      onClick={(e) => e.stopPropagation()}
    >
      <Icon size="1rem" color={hexColorByIntention(intention)} />
      {resource?.name ?? "Unknown"}
    </Group>
  );
}
