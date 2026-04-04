import {
  HardDrive,
  User,
  File,
  FolderOpen,
  Server,
  KeyRound,
  CircleQuestionMark,
  KeySquare,
  SquareAsterisk,
} from "lucide-react";

export const ICONS = {
  User,
  Device: Server,
  OnboardingKey: KeyRound,
  Filesystem: HardDrive,
  Folder: FolderOpen,
  File,
  EncryptionKey: KeySquare,
  Secret: SquareAsterisk,
  Unknown: CircleQuestionMark,
};
