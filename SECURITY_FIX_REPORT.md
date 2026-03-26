# Security Fix Report

Date (UTC): 2026-03-26
Branch: `chore/shared-ci-template`
Commit Reviewed: `8ced6c7`

## Inputs Reviewed

- Security alerts JSON:
  - Dependabot alerts: `0`
  - Code scanning alerts: `0`
- New PR dependency vulnerabilities list: `0`

## PR Dependency Change Check

Dependency manifests/lockfiles present in repository:

- `Cargo.toml`
- `Cargo.lock`
- `crates/component-qa/Cargo.toml`
- `crates/qa-cli/Cargo.toml`
- `crates/qa-lib/Cargo.toml`
- `crates/qa-spec/Cargo.toml`

Files changed in reviewed PR commit range (`HEAD~1..HEAD`):

- `.github/workflows/ci.yml`

Result: no dependency manifest or lockfile changes were introduced by this PR.

## Remediation Actions

No code or dependency remediation was required.

Reason:

- No Dependabot alerts were provided.
- No code scanning alerts were provided.
- No new PR dependency vulnerabilities were provided.
- PR changes do not modify dependency manifests or lockfiles.

## Validation Notes

Attempted defense-in-depth advisory checks:

- `cargo audit -q`
- `cargo deny check advisories`

Both commands failed in this CI sandbox because Rustup could not write temporary files under `/home/runner/.rustup/tmp` (read-only filesystem).

## Files Modified

- `SECURITY_FIX_REPORT.md` (updated for this run)
