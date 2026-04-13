import { useRead } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import {
  spotlight,
  SpotlightActionData,
  SpotlightActionGroupData,
} from "@mantine/spotlight";
import { Types } from "cicada_client";
import { LayoutDashboard } from "lucide-react";
import { useCallback, useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";

const ITEM_LIMIT = 7;
let count = 0;

export function useOmniSearch(): {
  search: string;
  setSearch: (value: string) => void;
  actions: SpotlightActionGroupData[];
} {
  const navigate = useNavigate();
  const nav = useCallback(
    (to: string) => {
      navigate(to);
      spotlight.close();
    },
    [navigate],
  );

  const [search, setSearch] = useState("");
  const searchTerms = useMemo(
    () =>
      search
        .toLowerCase()
        .split(" ")
        .filter((term) => term),
    [search],
  );

  const { data } = useRead(
    "Search",
    {
      keyword: search,
    },
    { enabled: !!search },
  );

  const filesystems = useRead("ListFilesystems", {}).data;

  const _actions = useMemo(() => {
    return [
      {
        group: "",
        actions: [
          {
            id: "Home",
            label: "Home",
            leftSection: <LayoutDashboard size="1.3rem" />,
            onClick: () => nav("/"),
          },
          {
            id: "Filesystems",
            label: "Filesystems",
            leftSection: <ICONS.Filesystem size="1.3rem" />,
            onClick: () => nav("/filesystem"),
          },
          {
            id: "Secrets",
            label: "Secrets",
            leftSection: <ICONS.Secret size="1.3rem" />,
            onClick: () => nav("/secrets"),
          },
          {
            id: "Encryption",
            label: "Encryption",
            leftSection: <ICONS.EncryptionKey size="1.3rem" />,
            onClick: () => nav("/encryption-keys"),
          },
        ].filter((item) => {
          if (!item) return;
          const label = item.label.toLowerCase();
          return (
            searchTerms.length === 0 ||
            searchTerms.every((term) => label.includes(term))
          );
        }) as SpotlightActionData[],
      },

      {
        group: "Filesystems",
        actions:
          data?.filesystems?.map((filesystem) => ({
            id: filesystem.id,
            label: filesystem.name,
            onClick: () => nav(`/filesystems/${filesystem.id}`),
            leftSection: <ICONS.Filesystem size="1.3rem" />,
          })) ?? [],
      },

      {
        group: "Files",
        actions:
          data?.nodes?.map((node) => ({
            id: node.id,
            label: node.name,
            onClick: () => nav(`/filesystems/${node.filesystem}/${node.inode}`),
            leftSection:
              node.kind === Types.NodeKind.Folder ? (
                <ICONS.Folder size="1.3rem" />
              ) : (
                <ICONS.File size="1.3rem" />
              ),
            description: `Filesystem: ${filesystems?.find((fs) => fs.id === node.filesystem)?.name}`,
          })) ?? [],
      },

      {
        group: "Secrets",
        actions:
          data?.secrets?.map((secret) => ({
            id: secret.id,
            label: secret.name,
            onClick: () => nav(`/secrets/${secret.id}`),
            leftSection: <ICONS.Secret size="1.3rem" />,
          })) ?? [],
      },

      {
        group: "Encryption",
        actions:
          data?.encryption_keys?.map((encryptionKey) => ({
            id: encryptionKey.id,
            label: encryptionKey.name,
            onClick: () => nav(`/encryption-keys/${encryptionKey.id}`),
            leftSection: <ICONS.EncryptionKey size="1.3rem" />,
          })) ?? [],
      },
    ];
  }, [data]);

  // LIMIT the action count for performance.
  // Reset count on render before creating actual actions.
  count = 0;
  const actions: SpotlightActionGroupData[] = [];
  for (const group of _actions) {
    const groupActions = [];
    for (const action of group.actions) {
      groupActions.push(action);
      count += 1;
      if (count > ITEM_LIMIT) {
        break;
      }
    }
    if (groupActions.length) {
      actions.push({ group: group.group, actions: groupActions });
    }
    if (count > ITEM_LIMIT) {
      break;
    }
  }

  return {
    search,
    setSearch,
    actions,
  };
}
