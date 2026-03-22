# component-qa E2E Fixture

This directory contains a small replay fixture for exercising `component-qa`
through flow wizard tooling.

Files:

- `answers.json`: seeded wizard answers for the `component-qa` setup flow.
- `questions.form.json`: minimal QA form consumed by `component-qa`.

Assumption:

- The wizard/add-step flow is using the setup contract exposed by
  `component-qa`.
- The setup bootstrap question is `qa_form_asset_path`, pointing at a generated
  FormSpec JSON file.

Example:

```bash
gtc wizard --answers-file e2e/component-qa/answers.json
```
