export const run = async (action: string) => {
  const branch = await new Deno.Command("bash", {
    args: ["-c", "git rev-parse --abbrev-ref HEAD"],
  })
    .output()
    .then((r) => new TextDecoder("utf-8").decode(r.stdout).trim());

  // Cargo check first here to make sure lock file is updated before commit.
  const cmd = `
cargo check

km run -y action ${action} "CICADA_BRANCH=${branch}"
`
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0 && !line.startsWith("//"))
    .join(" && ");

  new Deno.Command("bash", {
    args: ["-c", cmd],
  }).spawn();
};
