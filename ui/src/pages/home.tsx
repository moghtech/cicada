import { useRead } from "@/lib/hooks";
import { ICONS } from "@/lib/icons";
import { FancyCard, Page } from "mogh_ui";
import { Group, SimpleGrid, Text } from "@mantine/core";
import { Link } from "react-router-dom";
import { type LucideIcon } from "lucide-react";

export default function HomePage() {
  const { data: filesystems } = useRead("ListFilesystems", {});
  const { data: secrets } = useRead("ListSecrets", {});
  const { data: encryptionKeys } = useRead("ListEncryptionKeys", {});

  const sections: {
    title: string;
    icon: LucideIcon;
    path: string;
    count: number | undefined;
    description: string;
  }[] = [
    {
      title: "Filesystems",
      icon: ICONS.Filesystem,
      path: "/filesystems",
      count: filesystems?.length,
      description: "Manage filesystems and browse files",
    },
    {
      title: "Secrets",
      icon: ICONS.Secret,
      path: "/secrets",
      count: secrets?.length,
      description: "Manage encrypted secrets",
    },
    {
      title: "Encryption Keys",
      icon: ICONS.EncryptionKey,
      path: "/encryption-keys",
      count: encryptionKeys?.length,
      description: "Manage encryption keys",
    },
    {
      title: "Access",
      icon: ICONS.Access,
      path: "/access",
      count: undefined,
      description: "Users, devices, onboarding keys, and policies",
    },
  ];

  return (
    <Page title="Dashboard" icon={ICONS.Info}>
      <SimpleGrid cols={{ base: 1, sm: 2 }}>
        {sections.map((section) => (
          <FancyCard
            key={section.path}
            renderRoot={(props) => <Link to={section.path} {...props} />}
            p="md"
            bdrs="md"
          >
            <Group mb="xs">
              <section.icon size="1.4rem" />
              <Text fw={600} size="lg">
                {section.title}
              </Text>
              {section.count !== undefined && (
                <Text c="dimmed" size="sm" ml="auto">
                  {section.count}
                </Text>
              )}
            </Group>
            <Text c="dimmed" size="sm">
              {section.description}
            </Text>
          </FancyCard>
        ))}
      </SimpleGrid>
    </Page>
  );
}
