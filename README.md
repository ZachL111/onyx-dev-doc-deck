# onyx-dev-doc-deck

`onyx-dev-doc-deck` explores developer tools with a small Rust codebase and local fixtures. The technical goal is to build a Rust toolkit that studies doc behavior through append-only fixtures, with checkpoint recovery checks and no network dependency.

## Use Case

The project exists to keep a narrow engineering decision visible and testable. For this repo, that decision is how change width and review cost should influence a review result.

## Onyx Dev Doc Deck Review Notes

For a quick review, compare `change width` with `safe rewrite` before reading the middle cases.

## Highlights

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/onyx-dev-doc-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `change width` and `safe rewrite`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Code Layout

The fixture data drives the tests. The code stays thin, while `metadata/domain-review.json` and `config/review-profile.json` explain what each case is meant to protect.

The Rust implementation avoids hidden state so fixture changes are easy to reason about.

## Run The Check

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Regression Path

The same command runs the local verification path. The highest-scoring domain case is `stale` at 206, which lands in `ship`. The most cautious case is `recovery` at 127, which lands in `watch`.

## Future Work

This remains a local project with deterministic fixtures. It does not depend on credentials, hosted services, or live data. Future work should add richer malformed inputs before widening the public API.
