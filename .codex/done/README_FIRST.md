# Greentic QA — PR Bundle

Date: 2026-01-23  
Repo: `greenticai/greentic-qa`  
Scope: New repo containing `component-qa` + `qa-cli` + `qa-spec` + `qa-wizard` deployable gtpack.

This bundle contains *detailed* PR plans intended for Codex-driven implementation.
Each PR is designed to be mergeable independently, with tests/CI gates.

---

## Repo structure (target)

```
greentic-qa/
  .codex/
    repo_overview.md
  global_rules.md
  SECURITY.md
  LICENSE.md
  README.md
  ci/
    local_check.sh
  .github/
    workflows/
      ci.yml
      publish.yml
      nightly-e2e.yml
  crates/
    qa-spec/
    component-qa/
    qa-cli/
  packs/
    qa-wizard-pack/
```

---

## Conventions used in these PRs

- **“card”** means Adaptive Card v1.3 interactive transport (channel-agnostic).
- **“event/headless”** means event-driven, no UI rendering required.
- **Secrets** are **default deny**; any read/write requires explicit policy + allowlist.

---


