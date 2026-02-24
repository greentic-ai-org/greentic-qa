# PR-QA-01-greentic-qa-lib-i18n.md

## Decision Lock (approved)

1. API shape
- Implement both:
  - `WizardDriver` as the primary stepwise API.
  - `QaRunner::run_wizard(...)` as a minimal convenience wrapper.
- `QaRunner::run_wizard(...)` behavior:
  - If an answer-provider callback is supplied, use it to drive completion.
  - If callback is absent and the flow needs interaction, return `QaLibError::NeedsInteraction`.

2. Driver payload contract
- `WizardDriver::next_payload_json()` returns JSON string for the selected `WizardFrontend`:
  - `JsonUi`: JSON UI payload.
  - `Card`: adaptive card JSON payload.
  - `Text`: JSON wrapper payload (machine-consumable), e.g. includes text and progress/next-question metadata.

3. Public API data format
- Keep public driver/runner surface JSON-string based for this PR.
- Typed payload helpers (e.g. `next_payload_value()`) are deferred and can be added later non-breaking.

4. Naming clarity vs endpoint signatures
- Use clear internal variable names consistently:
  - `ctx_json` = runtime context (`locale`, `i18n_resolved`, debug flags, etc.)
  - `config_json` = form config payload (`form_spec_json`, etc.)
- Do not change `component-qa` endpoint signatures in this PR.

5. `--i18n-resolved` file format
- Accept flat string map only: `{"key":"value"}`.
- Reject nested/non-string maps with clear error:
  - `i18n-resolved must be a flat object map of string keys to string values.`

6. `--i18n-debug`
- Include now in PR-QA-01.
- Set both runtime keys for compatibility when enabled:
  - `i18n_debug: true`
  - `debug_i18n: true`

7. Test scope
- Required:
  - `qa-lib` i18n test proving localized string appears with locale + resolved map.
- Optional:
  - Minimal CLI plumbing test (flag parse + map load), no brittle interactive E2E requirement.

8. Scope exclusions (moved)
- Move machine-output additions (`--output-answers`, `--output-format`) to PR-QA-02.
- Move formal exit-code mapping table to PR-QA-02.

9. Re-exports policy
- Keep `qa-lib` re-exports narrow:
  - `qa_spec::AnswerSet`
  - `qa_spec::i18n::ResolvedI18nMap`
- Do not broadly re-export internal spec/domain types in this PR.

## Title
Introduce `greentic-qa-lib` (orchestration facade) + wire i18n end-to-end (library + CLI flags)

## Background
The greentic-qa workspace currently has:

- `crates/qa-spec` (core types/render/validate/runner + i18n resolver functions)
- `crates/component-qa` (adapter exposing JSON-string endpoints; applies i18n when `ctx.locale` + `ctx.i18n_resolved` are present)
- `crates/qa-cli` (binary `greentic-qa`; contains orchestration in `main.rs` and `wizard.rs` and currently passes `"{}"` as ctx_json, so locale/i18n cannot be driven from CLI)

Audits confirm i18n exists in model/render pipeline but is **not usable from CLI** and there is **no catalog loader**—i18n depends on caller-supplied `ctx.i18n_resolved`.

This PR:
1) Adds `greentic-qa-lib` to provide a stable, typed orchestration API (no subprocess integration required for embedders).
2) Makes i18n “work” by ensuring locale + resolved map are passed through the orchestration layer into component-qa calls.
3) Adds non-breaking CLI flags to `greentic-qa wizard` to supply `--locale` and `--i18n-resolved <file>` (JSON map).

> Note: This PR does not introduce a new translation system. It exposes and wires the existing “resolved map” model.

---

## Goals
### A. `greentic-qa-lib`
- Provide a reusable library API that runs the wizard orchestration currently embedded in `qa-cli`.
- Keep `qa-spec` as the stable core; do not move domain logic out of it.
- Keep `component-qa` as the adapter layer; lib orchestrates by calling adapter endpoints.

### B. i18n end-to-end
- Enable i18n by passing `ctx_json` containing `locale` and `i18n_resolved` on all relevant calls:
  - `component_qa::next`
  - `component_qa::render_*`
  - `component_qa::submit_patch` / `submit_all`
  - `component_qa::validate` (if used)
- Add CLI flags for wizard:
  - `--locale <LOCALE>`
  - `--i18n-resolved <FILE>` (JSON object map: `{ "key": "value", ... }`)
- Maintain full backwards compatibility: if flags are omitted, behavior unchanged.

---

## Non-goals
- No new i18n framework (fluent/icu/gettext).
- No opinionated catalog file layout or merge strategy beyond “load a JSON map”.
- No major redesign of `qa-cli` prompt UX (stdin/stdout) beyond calling into lib.

---

## Public API (greentic-qa-lib)

### New crate
`crates/qa-lib` (package name `greentic-qa-lib`, lib crate)

#### Core types (minimal, typed facade)
- `QaRunner`
- `WizardRunConfig`
- `WizardRunResult`
- `WizardFrontend` (Text | JsonUi | Card)
- `I18nConfig` (locale + resolved map)

#### Proposed API
```rust
// crates/qa-lib/src/lib.rs

pub use qa_spec::{FormSpec, AnswerSet};
pub use qa_spec::i18n::ResolvedI18nMap;

#[derive(Clone, Debug)]
pub enum WizardFrontend {
    Text,
    JsonUi,
    Card,
}

/// i18n is caller-supplied: locale + resolved map
#[derive(Clone, Debug, Default)]
pub struct I18nConfig {
    pub locale: Option<String>,
    pub resolved: Option<ResolvedI18nMap>,
    pub debug: bool, // optional: attach debug metadata if supported
}

#[derive(Clone, Debug)]
pub struct WizardRunConfig {
    pub spec_json: String,                 // raw FormSpec JSON
    pub initial_answers_json: Option<String>, // optional answers JSON
    pub frontend: WizardFrontend,
    pub i18n: I18nConfig,
    pub verbose: bool,
}

#[derive(Clone, Debug)]
pub struct WizardRunResult {
    pub answer_set: AnswerSet,
    pub answer_set_cbor_hex: String,
}

pub struct QaRunner;

impl QaRunner {
    pub fn run_wizard(config: WizardRunConfig) -> Result<WizardRunResult, QaLibError>;
}
Error model

QaLibError wraps:

IO errors

JSON parse errors

component-qa endpoint errors

validation errors

user cancel (if detectable in CLI; library can expose a variant but CLI decides exit code mapping)

Keep CLI exit-code mapping in qa-cli only.

i18n key convention

Keep current resolver behavior:

looks for "{locale}:{key}", "{locale}/{key}", then bare key

Recommended convention for callers:

Provide locale-specific files containing bare keys only:

en-GB.json contains "wizard.menu.title": "…"

nl-NL.json contains "wizard.menu.title": "…"
This is simplest and works with the resolver.

Implementation Plan
1) Workspace wiring

Files

Root Cargo.toml (workspace members)

Add crates/qa-lib

Acceptance

cargo test --workspace still passes after scaffolding (even before refactor).

2) Create crates/qa-lib

Files

crates/qa-lib/Cargo.toml

crates/qa-lib/src/lib.rs

(optional) crates/qa-lib/src/wizard.rs

(optional) crates/qa-lib/src/util.rs

Dependencies

qa-spec

component-qa

serde, serde_json, thiserror

Key tasks

Define the public API described above.

Implement the wizard orchestration by lifting the non-IO orchestration from qa-cli:

spec loading/parsing stays caller-provided (raw JSON) for now

loop:

call component_qa::next(ctx_json, config_json)

call component_qa::render_json_ui / render_text / render_card based on frontend

interpret payload for next question id and schema

accept answers injection via initial_answers_json (for non-interactive / prefilled flows)

call component_qa::submit_patch

completion:

call component_qa::submit_all or final validate if needed

return AnswerSet + CBOR hex

Note: Interactive prompting remains CLI-only; qa-lib focuses on orchestration and producing render payloads + accepting “patch submissions”. To keep this PR small, qa-lib can expose a “wizard driver” method used by CLI’s existing prompt loop, OR it can embed the loop if it accepts a callback interface. Pick the minimal approach below.

Minimal approach (least churn)

Implement a WizardDriver in lib that:

returns the next render payload (JSON UI / text / card)

accepts a patch (JSON object) to submit

tells you when done and returns AnswerSet

This maps perfectly to your current CLI loop and also supports embedders.

API

pub struct WizardDriver { ... }

impl WizardDriver {
  pub fn new(config: WizardRunConfig) -> Result<Self, QaLibError>;
  pub fn next_payload_json(&mut self) -> Result<String, QaLibError>; // JSON UI (or text/card depending)
  pub fn submit_patch_json(&mut self, patch_json: &str) -> Result<ValidationOrProgress, QaLibError>;
  pub fn is_complete(&self) -> bool;
  pub fn finish(self) -> Result<WizardRunResult, QaLibError>;
}

Why

Keeps prompt IO in CLI

Library is reusable in operator, web UIs, etc.

Avoids redesigning question prompting logic here

3) Wire i18n through context JSON (the actual “make it work” change)

Where

greentic-qa-lib must construct a non-empty ctx JSON and pass it to every component-qa call.

Implementation detail
Add helper in qa-lib:

fn build_ctx_json(i18n: &I18nConfig) -> String {
  let mut obj = serde_json::Map::new();
  if let Some(locale) = &i18n.locale { obj.insert("locale".into(), locale.clone().into()); }
  if let Some(map) = &i18n.resolved { obj.insert("i18n_resolved".into(), serde_json::to_value(map).unwrap()); }
  if i18n.debug { obj.insert("i18n_debug".into(), true.into()); }
  serde_json::Value::Object(obj).to_string()
}

Then ensure every call uses:

ctx_json = build_ctx_json(&config.i18n)

(not "{}")

Acceptance

Existing tests still pass.

New tests show localized strings appear when spec uses *_i18n.

4) Refactor qa-cli to use greentic-qa-lib (thin wrapper)

Files

crates/qa-cli/src/main.rs

crates/qa-cli/src/wizard.rs

Tasks

Replace internal orchestration / component-qa call graph with calls to greentic-qa-lib::WizardDriver

Keep:

clap parsing

stdin/stdout prompt UI (prompt_question, parse_answer)

printing of final CBOR hex / JSON answers

Acceptance

Command behavior unchanged by default.

qa-cli no longer directly calls component-qa endpoints except through lib.

5) Add wizard i18n flags to CLI (non-breaking)

Files

crates/qa-cli/src/main.rs (clap definitions)

crates/qa-cli/src/wizard.rs (plumbing)

Add flags

greentic-qa wizard --locale <LOCALE>

greentic-qa wizard --i18n-resolved <FILE>

optionally --i18n-debug (bool) to toggle debug metadata if supported

Behavior

Load the resolved map from file if provided:

JSON object map: { "wizard.menu.title": "..." }

Pass to lib via I18nConfig

Acceptance

No flags → current behavior unchanged.

With flags → localized output when spec uses i18n keys.

6) Tests (must be added)
A) Library i18n test (qa-lib)

New file

crates/qa-lib/tests/i18n_wizard.rs

Test idea (grounded in existing fixtures)

Use crates/qa-spec/tests/fixtures/simple_form.json if it already includes title_i18n/description_i18n.

If it doesn’t, add a minimal new fixture in crates/qa-lib/tests/fixtures/i18n_form.json using the existing FormSpec schema:

include one question with title_i18n.key = "q.name"

set literal title empty or present; ensure i18n wins

Provide:

locale = "nl-NL"

i18n_resolved = { "q.name": "Naam" } (bare key form)

Assert render payload contains "Naam".

B) CLI i18n smoke test (qa-cli)

Update or add

crates/qa-cli/src/main.rs tests module or a new integration test if you already have harness.

Run wizard in a non-interactive mode as much as possible:

Use --answers <file> prefill to complete quickly

Use --format json to inspect payload text or ensure the first prompt label is localized (depends on existing output)
If full CLI testing is hard due to interactive IO, keep CLI test minimal and focus on lib test; CLI plumbing is thin.

Machine-output improvements (optional but recommended for operator integration)

This PR can include a small additive improvement (still non-breaking):

Add wizard --output-answers <FILE>

Write stable JSON AnswerSet to a file.

Flags

--output-answers <FILE>

--output-format json|cbor|both (default: json)

Behavior

Always keep current stdout behavior

Additionally write to file if specified

This enables greentic-operator to call greentic-qa binary as subprocess and consume answers reliably.

If you prefer to keep this separate, move it to PR-QA-02. (But it pairs well with making CLI a stable integration target.)

Acceptance Criteria

cargo test --workspace passes.

New crate greentic-qa-lib exists and builds on all supported targets.

greentic-qa CLI continues to work the same with no flags.

greentic-qa wizard --locale nl-NL --i18n-resolved ./nl.json --spec ... results in localized question titles/descriptions when *_i18n fields exist.

No new Q&A state engine exists outside qa-spec/component-qa; qa-cli delegates orchestration to greentic-qa-lib.

Documentation updated:

brief README section on i18n input model (resolved map) and CLI flags.

File-by-file Change List (concrete)
New

crates/qa-lib/Cargo.toml

crates/qa-lib/src/lib.rs

crates/qa-lib/src/wizard.rs (if split)

crates/qa-lib/tests/i18n_wizard.rs

(optional) crates/qa-lib/tests/fixtures/i18n_form.json

(optional) crates/qa-cli/tests/fixtures/i18n_resolved_nl.json

Modified

Root Cargo.toml (workspace members)

crates/qa-cli/Cargo.toml (add dependency on greentic-qa-lib)

crates/qa-cli/src/main.rs (use lib; add CLI flags)

crates/qa-cli/src/wizard.rs (delegate to lib; pass i18n config)
