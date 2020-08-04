# smartdiff
A tool to be eventually used for getting a smarter diff for various file types.

## How to use it

Smartdiff can be used via the `git-difftool` command. You can follow these steps to use it,
although it is still very rough around the edges:

1. Clone the repository, and run `cargo build` inside it. If you do not have cargo, you need
to [install it](https://crates.io/).
2. The binary will be located within `target/debug`.
3. Add the path to that `target/debug` to your PATH variable.
4. Go to a Git repository that has an unstaged change to a JSON file, and run: `git smartdiff`.

That should do it, although it doesn't do much for now.
