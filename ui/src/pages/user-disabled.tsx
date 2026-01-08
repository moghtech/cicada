import { useUser } from "@/lib/hooks";
import { Button, Center, Paper } from "@mantine/core";
import { MoghAuth } from "cicada_client";
import { UserX } from "lucide-react";

export default function UserDisabled() {
  const user_id = useUser().data?.id;
  return (
    <Center>
      <Paper>
        <UserX size="2rem" />
        User Not Enabled
        <Button
          variant="filled"
          color="red"
          onClick={() => {
            user_id && MoghAuth.LOGIN_TOKENS.remove(user_id);
            location.reload();
          }}
        >
          Log Out
        </Button>
      </Paper>
    </Center>
  );
}
