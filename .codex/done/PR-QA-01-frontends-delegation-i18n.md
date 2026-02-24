# PR-QA-01 — Frontend Abstraction + Deterministic Plan/Execute + Spec Composition + i18n

**Repo:** `greentic-qa`  
**Theme:** Deterministic wizard planning, reusable frontends, additive i18n, and spec-level delegation.

## Confirmed findings
- `submit_patch` currently validates and applies side effects in one call; this violates the plan-first execution boundary.
- Renderer/frontend behavior already exists (renderers + CLI presenter); this PR should unify/wrap, not rebuild.
- `next(form_id, ctx_json, answers_json)` has naming/meaning mismatch (`ctx_json` treated as config); fix now to avoid delegation confusion.
- i18n is currently string-only; any i18n support must be backward compatible.
- `docs/` additions are additive and must be explicitly scoped.

## Outcomes
- Add a deterministic **plan + execute** model:
  - planning endpoints/functions are side-effect free
  - execution is explicit and separate
- Introduce a `QaFrontend` abstraction that wraps existing text/json/card renderers.
- Add additive i18n fields and resolution flow without breaking existing specs.
- Implement delegation via **spec composition** (includes), with deterministic expansion and cycle detection.
- Keep CLI UX and existing payload behavior stable by default.

## Non-goals
- No breaking CLI behavior unless explicitly documented.
- No host-callback delegation in this PR.
- No mandatory WIT surface expansion in this PR (stage cross-boundary changes later).

## Architecture decisions
1. `QaRunner` location:
   - Put pure runner logic in `qa-spec` (or `qa-spec::runner` module).
   - Keep `component-qa` as transport/adapters and compatibility wrappers.
2. Frontend strategy:
   - Add `QaFrontend` trait and wrap existing implementations with minimal churn.
   - Preserve current renderer outputs unless an additive i18n behavior requires otherwise.
3. Deterministic plan schema:
   - Introduce internal `QaPlanV1` with:
     - `plan_version: 1`
     - `form_id`
     - step/mode/state token fields already needed by current flow
     - `validated_patch` (canonical patch)
     - `effects: []` (explicit deferred side effects)
     - optional warnings/errors
   - Versioning:
     - bump `plan_version` on semantic meaning change only
     - additive optional fields do not require bump
4. `submit_patch` API:
   - Keep public for compatibility.
   - Re-implement as wrapper: `plan -> execute -> combined result`.
   - Document as legacy convenience; prefer explicit plan+execute path.
5. Delegation model:
   - Use **spec composition** (include/subform by reference), not callback orchestration.
   - Expansion must be deterministic and stable in ordering.
6. i18n model:
   - Keep existing fields (`title`, `description`) unchanged.
   - Add additive fields:
     - `title_i18n: Option<I18nText>`
     - `description_i18n: Option<I18nText>`
   - Rendering precedence:
     - i18n resolved text if present
     - else raw string fields
7. Resolver contract:
   - Use synchronous pre-resolved injection in this PR (map or sync resolver trait).
   - Any async resolution belongs in adapters before calling pure runner.
8. Locale fallback:
   - requested locale
   - form default locale (if present)
   - raw text fallback
9. Adaptive Card i18n output:
   - output resolved strings by default
   - optional debug metadata only when debug i18n mode is enabled
10. `next` compatibility:
   - Keep current signature for compatibility.
   - Clarify/document current config semantics and align internal naming.
   - Use a single endpoint version; evolve behavior additively in-place.
11. WIT change policy:
   - Keep WIT stable in PR-QA-01 unless strictly required.
   - Stage broader WIT i18n/delegation transport inputs in PR-QA-02.

## Implementation scope (tight)
1. Add audit doc:
   - `docs/audit-frontends.md`
2. Add pure planning API in runner module:
   - e.g. `plan_submit_patch()`, `validate_patch()`, `plan_next()` (naming can follow repo style)
3. Refactor execution boundary:
   - make planning side-effect free
   - execute effects in explicit step
4. Refactor compatibility wrapper:
   - `submit_patch()` calls plan then execute
5. Add `QaFrontend` trait wrapping existing text/json/card renderers.
6. Add additive i18n structs/fields and resolver injection path in runner/render pipeline.
7. Add spec composition includes with:
   - deterministic expansion
   - cycle detection (`IncludeCycleDetected { chain }`)
   - missing include target error
8. Align `next` naming/docs for clarity while preserving compatibility.
9. Add docs:
   - `docs/frontends.md`
   - `docs/i18n.md`

## Safety contract
- Planning endpoints/functions must be guaranteed side-effect free (contract + tests).
- Execution endpoints/functions apply effects explicitly.
- Existing file-output safety constraints remain unchanged (env-gated behavior in CLI/generator paths).

## Tests
- Golden: sample form -> adaptive card JSON stability.
- Round-trip: adaptive-card submit payload -> validated patch -> apply plan effects.
- Regression: existing CLI wizard behavior unchanged in default mode (no locale/i18n fields).
- New i18n tests:
  - i18n fields + locale resolve correctly
  - fallback order is deterministic
- Include/composition tests:
  - deterministic expansion order
  - cycle detection error
  - missing include target error
- Contract tests:
  - planning path does not mutate store/apply effects
  - execution path applies effects from plan

## Acceptance criteria
- Deterministic plan/execute split implemented and tested.
- `submit_patch` remains compatible but internally uses plan+execute.
- Frontends unified under trait wrappers with no default UX regression.
- Additive i18n fields function without breaking old JSON/spec fixtures.
- Spec composition works with deterministic expansion and robust errors.
- New docs exist in `docs/`.
