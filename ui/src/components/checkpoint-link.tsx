import { useRead } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { Group } from "@mantine/core";
import { Types } from "cicada_client";
import { hexColorByIntention } from "mogh_ui";
import { Link } from "react-router-dom";

export interface CheckpointLinkProps {
  id: string;
  target: Types.CheckpointTarget;
}

export default function CheckpointLink({ id, target }: CheckpointLinkProps) {
  const checkpoint = useRead("ListCheckpoints", { target }).data?.find(
    (r) => r.id === id,
  );
  const encryptionKeys = useRead("ListEncryptionKeys", {}).data;
  const Icon = ICONS.Checkpoint;
  const intention =
    checkpoint?.encryption_key &&
    !encryptionKeys?.find((e) => e.id === checkpoint?.encryption_key)
      ?.initialized
      ? "Critical"
      : "Good";
  return (
    <Group
      renderRoot={(props) => <Link to={"/checkpoints/" + id} {...props} />}
      gap="xs"
      wrap="nowrap"
      w="fit-content"
      className="hover-underline"
      onClick={(e) => e.stopPropagation()}
    >
      <Icon size="1rem" color={hexColorByIntention(intention)} />
      {checkpoint?.name || "Checkpoint"}
    </Group>
  );
}
