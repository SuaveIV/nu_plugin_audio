# Contributing to nu_plugin_audio

Thanks for your interest in contributing! This document covers everything you need to get started.

---

## Getting started

### Prerequisites

- A stable Rust toolchain (`rustup` is recommended — see [rust-toolchain.toml](rust-toolchain.toml))
- On Linux, the ALSA development package:

  | Distro | Command |
  | --- | --- |
  | Debian / Ubuntu | `sudo apt install libasound2-dev pkg-config` |
  | Fedora / RHEL | `sudo dnf install alsa-lib-devel pkgconf-pkg-config` |
  | Arch | `sudo pacman -S alsa-lib pkgconf` |
  | openSUSE | `sudo zypper install alsa-lib-devel pkg-config` |

### Building

```bash
git clone https://github.com/SuaveIV/nu_plugin_audio.git
cd nu_plugin_audio
cargo build
```

To build with every decoder enabled:

```bash
cargo build --features=all-decoders
```

### Running locally

```nushell
cargo build
nu -c "plugin add target/debug/nu_plugin_audio"
```

> You'll need to restart Nushell (or run `plugin use nu_plugin_audio`) after registering for the first time.

---

## Commit messages

This project uses **[Conventional Commits](https://www.conventionalcommits.org/)**.
Commit messages are parsed automatically to generate the changelog on each release, so please follow the format.

### Format

```markdown
<type>(<optional scope>): <short description>

<optional body>

<optional footer>
```

### Types

| Type | When to use |
| --- | --- |
| `feat` | A new feature or command |
| `fix` | A bug fix |
| `docs` | Documentation changes only |
| `refactor` | Code changes that neither fix a bug nor add a feature |
| `perf` | Performance improvements |
| `test` | Adding or fixing tests |
| `chore` | Maintenance — dependency bumps, CI, build scripts |
| `style` | Formatting, whitespace — no logic changes |

### Examples

```sh
feat(meta): add bit_depth field to sound meta output
fix(player): clamp position to total duration to prevent overflow
docs: add CONTRIBUTING.md
chore: bump nu-plugin to 0.111.0
refactor(utils): extract format_duration into shared utils module
```

### Breaking changes

If your change breaks the public command interface, add `!` after the type and include a `BREAKING CHANGE:` footer:

```sh
feat(play)!: rename --amplify to --volume

BREAKING CHANGE: the -a / --amplify flag has been renamed to -v / --volume
to better match Nushell conventions.
```

Breaking changes will be called out explicitly in the generated changelog.

---

## Pull requests

- **One thing per PR** — focused changes are much easier to review and bisect if something goes wrong.
- **Target `main`** — all PRs should be opened against the main branch.
- **Update the README** if you add or change command flags, behaviour, or supported formats.
- There are no formal tests right now. If you add one, great — but it's not a blocker.

---

## Releasing (maintainers only)

Releases are managed with [cargo-smart-release](https://github.com/Byron/gitoxide/tree/main/cargo-smart-release).

### Prerequisites

- Your working tree must be clean and you must be on `main`.
- You need a branch protection bypass rule active for your account before pushing — enable it in **Settings → Rules → Rulesets**, run the release, then disable it again immediately after the tag push.

### Steps

```bash
just release
```

Which runs:

```bash
cargo smart-release --update-crates-index --execute --changelog-without commit-statistics --no-tag
git tag v$(cargo pkgid | cut -d# -f2)
git push origin v$(cargo pkgid | cut -d# -f2)
```

This will:

1. Parse conventional commits since the last tag and write `CHANGELOG.md`
2. Bump the version in `Cargo.toml`
3. Publish to [crates.io](https://crates.io/crates/nu_plugin_audio)
4. Push the version bump commit to `main`
5. Create and push the version tag as a separate step

The tag push triggers the Release CI, which builds platform binaries via cargo-dist and creates the GitHub release. Once the Release workflow completes, the ARM64 build fires automatically.

> The tag is pushed manually (via `--no-tag`) to avoid a race condition with cargo-dist's `pull_request` trigger, which would otherwise cause the Release workflow to run twice and corrupt the release.

### After releasing

- Disable your branch protection bypass.
- Verify the GitHub release has all platform binaries attached.
- The ARM64 build will attach its artifact automatically — check the Actions tab if it doesn't appear within a few minutes.

---

### Troubleshooting a failed release

#### The Release workflow ran twice and the release is empty or missing artifacts

This can happen if the tag was pushed before the version bump commit settled. The release may exist but have no binaries attached.

1. Check whether the build artifacts were produced:

```bash
gh run list --repo SuaveIV/nu_plugin_audio --workflow=release.yml
gh run download <run-id> --repo SuaveIV/nu_plugin_audio
```

1. If the artifacts exist, upload them manually:

```nushell
ls artifacts-build-local-*/ | where type == file | each { |f| cp $f.name ~/Downloads/release-artifacts/ }
ls artifacts-build-global/ | where type == file | each { |f| cp $f.name ~/Downloads/release-artifacts/ }
cd ~/Downloads/release-artifacts
ls | where name != "global-dist-manifest.json" | each { |f| gh release upload <tag> $f.name --repo SuaveIV/nu_plugin_audio --clobber }
```

1. If the release itself is missing, recreate it from the dist manifest before uploading:

```nushell
open global-dist-manifest.json | get announcement_github_body | save /tmp/notes.txt
gh release create <tag> --repo SuaveIV/nu_plugin_audio --title "<tag>" --notes-file /tmp/notes.txt --target <commit-sha>
```

#### The release needs to be fully rolled back

You will need your branch protection bypass active for the tag delete and force push.

```bash
gh release delete <tag> --repo SuaveIV/nu_plugin_audio --yes
git push origin :refs/tags/<tag>
git tag -d <tag>
git reset --hard HEAD~1
git push origin main --force
```

> Note: if the version was already published to crates.io, you cannot republish the same version number. Yank it and bump to the next patch version instead.

#### The ARM64 build did not trigger automatically

Kick it off manually:

```bash
gh workflow run release-arm64.yml --repo SuaveIV/nu_plugin_audio --field tag=<tag>
```

---

## Code style

- Run `cargo fmt` before committing.
- Run `cargo clippy` and address any warnings.
- Public functions and types should have doc comments (`///`).

---

## Just

This project uses a [`Justfile`](Justfile) for common tasks. If you have [just](https://github.com/casey/just) installed:
```just --list``` to see all available recipes.

The most useful ones for contributors are `just check` before committing and `just install-nx` / `just install-win` to install a local build in Nushell.

## Contributors

See [CONTRIBUTORS.md](CONTRIBUTORS.md) for the full list of people who have contributed to this project.
