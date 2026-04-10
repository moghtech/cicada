import { useRead } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { Group } from "@mantine/core";
import { Types } from "cicada_client";
import { hexColorByIntention } from "mogh_ui";
import { Link } from "react-router-dom";

export type ResourceType =
  | "EncryptionKey"
  | "Secret"
  | "Filesystem"
  | "Device"
  | "OnboardingKey";

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
      return "/devices/" + id;
    case "OnboardingKey":
      return "/onboarding-keys/" + id;
  }
}

export default function ResourceLink({ type, id }: ResourceLinkProps) {
  const resource = useRead(`List${type}s`, {}).data?.find((r) => r.id === id);
  const encryptionKeys = useRead("ListEncryptionKeys", {}).data;
  const Icon = ICONS[type];
  const intention =
    type === "EncryptionKey"
      ? (resource as Types.EncryptionKeyEntity)?.initialized
        ? "Good"
        : "Critical"
      : type === "Secret"
        ? (resource as Types.SecretListItem)?.encryption_key &&
          !encryptionKeys?.find(
            (e) => e.id === (resource as Types.SecretListItem)?.encryption_key,
          )?.initialized
          ? "Critical"
          : "Good"
        : ["Device", "OnboardingKey", "Policy"].includes(type)
          ? (resource as Types.DeviceRecord).enabled
            ? "Good"
            : "Critical"
          : type === "Filesystem"
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
