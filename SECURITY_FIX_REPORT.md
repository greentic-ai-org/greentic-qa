# Security Fix Report

Date (UTC): 2026-03-25
Branch: `ci/add-workflow-permissions`
Commit Reviewed: `ffbfa6a`

## Inputs Reviewed

- `security-alerts.json`
- `dependabot-alerts.json`
- `code-scanning-alerts.json`
- `pr-vulnerable-changes.json`

## Findings

- Dependabot alerts: `0`
- Code scanning alerts: `0`
- New PR dependency vulnerabilities: `0`

## PR Dependency Change Check

Dependency manifests/lockfiles present in the repo:

- `Cargo.toml`
- `Cargo.lock`
- `crates/component-qa/Cargo.toml`
- `crates/qa-cli/Cargo.toml`
- `crates/qa-lib/Cargo.toml`
- `crates/qa-spec/Cargo.toml`

Changed files in PR range (`origin/main...HEAD`):

- `.github/workflows/ci.yml`
- `.github/workflows/nightly-e2e.yml`

Result: no dependency manifest or lockfile changes were introduced by this PR.

## Remediation Actions

No code or dependency fixes were required. No vulnerabilities were identified in the provided security alert inputs, and no new dependency vulnerabilities were introduced by PR dependency changes.

## Files Modified

- `SECURITY_FIX_REPORT.md` (updated)
