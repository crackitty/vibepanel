# Contributing

Bug reports, feature requests and pull requests are all welcome. If you're planning something big, opening an issue first is a good way to check it fits before you put the time in.

## Setup

Build dependencies and steps are in the [Installation wiki](https://github.com/prankstr/vibepanel/wiki/Installation#from-source) (or use `nix develop`). Then:

```sh
cargo run    # starts a panel, stop your normal one first
```

## How the code fits together

`vibepanel-core` holds config parsing, validation and theming, with no GTK. `vibepanel` is the GTK app, with system integration in `services/` and bar widgets in `widgets/`.

- **Services** are main-thread singletons (`FooService::global()`) that notify widgets through `Callbacks`. `connect()` returns a `CallbackId`; keep it and `disconnect()` when the widget goes away.
- **Threads:** GTK only on the main thread. Background work (IPC, blocking I/O) hands results back with `glib::idle_add_once`.
- **Widgets:** start from a similar one in `widgets/`. Parse options in `WidgetConfig::from_entry` (call `warn_unknown_options`) and register the widget in `WidgetFactory::build`.
- **Compositors:** widgets use `CompositorManager` or the services built on it (`WorkspaceService`, `WindowTitleService`, ...), never compositor IPC directly. A new backend implements `CompositorBackend` (`services/compositor/types.rs`), gets detected in `factory.rs` and is added to `VALID_COMPOSITORS`.
- **CSS classes** live in `styles.rs`. Prefix names that collide with GTK or Adwaita with `vp-`.
- **User docs** live in the [wiki](https://github.com/prankstr/vibepanel/wiki), which can't take PRs. For new or changed options, put the wiki text in the PR description.

## Making changes

- Keep PRs focused to one fix or feature, and only the diff it needs.
- If you change behavior or the UI, test it on a real session. A screenshot in the PR helps a lot.
- Wrap new user-facing strings in `tr!()` (see nearby code), run `make -C crates/vibepanel/po` and commit the updated catalogs, including the `.mo` files. Needs GNU gettext and `xtr` (`cargo install xtr --version 0.1.11 --locked`).
- If you add or change an icon, run `scripts/subset-font.sh` to update the bundled font.
- Say what the change does and why in the PR description. Short and concise over long and generic please.

## Before opening a PR

The pre-commit hook (installed by `cargo test`) runs these checks for you and CI runs them again on every PR:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
scripts/run-ui-regression-tests.sh    # needs xvfb-run, see docs/ui-regression-tests.md
```

Use a [Conventional Commits](https://www.conventionalcommits.org/) PR title (`feat:`, `fix:`, `docs:`, ...). PRs are squash-merged, so the title becomes the commit that the changelog and releases are generated from.

## AI-assisted contributions

AI tools are okay to use, much of VibePanel itself is already built with AI tools. With that said:

- You're responsible for what you submit: review the diff, test it. Be prepared to answer questions about it.
- Write issues and PR descriptions in your own voice. Using AI to help is fine but at least make a small effort to give it a human touch :)

## Translations

See [Localization](https://github.com/prankstr/vibepanel/wiki/Localization) for how to add or update a translation.
