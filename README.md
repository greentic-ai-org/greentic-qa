# Greentic QA

This repository hosts the Greentic QA toolkit: `qa-spec` for defining QA forms/flows, `component-qa` as a wasm component, `greentic-qa` for authoring, and `qa-wizard-pack` for deployable wizard automation.

## Repository structure

```
greentic-qa/
  .codex/
  global_rules.md
  SECURITY.md
  LICENSE.md
  README.md
  ci/
  crates/
    qa-spec/
    component-qa/
    qa-cli/ (greentic-qa CLI)
  packs/
    qa-wizard-pack/
```

## Getting started

1. Run `ci/local_check.sh` to verify formatting, linting, and tests.
2. Each crate is a workspace member; building the workspace checks all executable components.
3. Follow the PR bundle in `.codex/QA-PR-*.md` for the incremental implementation plan.

## Governance

- Review `.codex/global_rules.md` before adding features (CLI scaffolding, secrets, visibility, and persistence policies are enforced there).
- Secrets are default-deny; use `secrets-policy` helpers once implemented.
- Wizard outputs default to event emission; enable dev-mode explicitly via `QA_WIZARD_OUTPUT_DIR` under an allowed root.

## greentic-qa CLI

- `greentic-qa wizard --spec <form.json>` runs the text-based component wizard against a FormSpec.
  - Optional i18n flags:
    - `--locale <LOCALE>`
    - `--i18n-resolved <file.json>` (flat JSON object map of string keys to string values)
    - `--i18n-debug` (adds debug metadata for compatible frontends)
- `greentic-qa new [--out <dir>] [--force]` walks through metadata and question prompts, then emits the bundle of forms/flows/examples/schemas (stored under `<dir>/<dir_name>`). If `--out` isn’t provided the command uses `QA_WIZARD_OUTPUT_DIR` (or falls back to the current working directory). The CLI refuses to overwrite an existing bundle unless you pass `--force`.
- `greentic-qa generate --input <answers.json> [--out <dir>] [--force]` consumes a JSON payload (see `ci/fixtures/sample_form_generation.json`) and regenerates the bundle non-interactively. It respects `QA_WIZARD_OUTPUT_DIR`/`QA_WIZARD_ALLOWED_ROOTS` so you can run it as the dev-mode writer while ensuring file writes stay under the allowed roots.
- `greentic-qa validate --spec <form.json> --answers <answers.json>` validates stored answers and prints the error summary.

Smoke tests rely on `ci/scripts/smoke.sh`, which reads the fixture above and runs `greentic-qa generate` to build a sample bundle. The generated bundle includes the derived README plus the JSON artifacts that you can reuse in other repositories or packs.

## component-qa compatibility notes

- The interface remains single-version and backward-compatible.
- Config input accepts:
  - raw `FormSpec` JSON (legacy/direct)
  - config envelope with `form_spec_json`
  - optional `include_registry` (`form_ref -> form spec JSON`) for include expansion.
- Runtime context accepts:
  - direct context payload (legacy)
  - additive envelope style with `ctx` object.
- i18n rendering can consume:
  - `ctx.locale`
  - `ctx.i18n_resolved` map
  - optional debug flag `ctx.i18n_debug` (or `ctx.debug_i18n`) for card metadata.
