# smartdiff
A tool to be eventually used for getting a smarter diff for various file types.

## How to use it

Smartdiff can be used via the `git-difftool` command. You can follow these steps to use it,
although it is still very rough on the edges:

1. Clone the repository, and run `cargo build` inside it. If you do not have cargo, you need
to [install it](https://crates.io/).
2. The binary will be located within `target/debug`, named `smartdiff`.
3. Run `export SMARTDIFF_PATH=<full_path_to_smartdiff_directory>`, substituting the placeholder with
the actual path to the project's root.
4. Go to a Git repository, and run: `git difftool --extcmd="$SMARTDIFF_PATH/target/debug/smartdiff"`.

That should do it, although it doesn't do much for now.
