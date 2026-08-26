export type InstallCommand = {
  id: "cargo" | "github";
  label: string;
  command: string;
};

export const installCommands: InstallCommand[] = [
  {
    id: "cargo",
    label: "Cargo",
    command: "cargo install siftforge",
  },
  {
    id: "github",
    label: "GitHub binary",
    command:
      "curl --proto '=https' --tlsv1.2 -LsSf https://github.com/DovudxonN/siftforge/releases/latest/download/siftforge-installer.sh | sh",
  },
];
