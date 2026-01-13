import { Flex, Group, Text } from "@mantine/core";
import { FC, ReactNode } from "react";

export const Page = ({
  title,
  icon: Icon,
  rightTitle,
  actions,
  children,
}: {
  title: string;
  icon: FC<{ size?: string | number }>;
  rightTitle?: ReactNode;
  actions?: ReactNode;
  children?: ReactNode;
}) => {
  return (
    <Flex direction="column" gap="lg">
      <Group>
        <Icon size={20} />
        <Text fz="h2" opacity={0.6}>
          {title}
        </Text>
        {rightTitle}
      </Group>

      {actions && <Group>{actions}</Group>}

      {children}
    </Flex>
  );
};
