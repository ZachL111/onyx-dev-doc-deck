# Review Journal

The repository goal stays the same: build a Rust toolkit that studies doc behavior through append-only fixtures, with checkpoint recovery checks and no network dependency. This note explains the added review angle.

The local checks classify each case as `ship`, `watch`, or `hold`. That gives the project a small review vocabulary that matches its developer tools focus without claiming live deployment or external usage.

## Cases

- `baseline`: `change width`, score 185, lane `ship`
- `stress`: `diagnostic quality`, score 177, lane `ship`
- `edge`: `review cost`, score 152, lane `ship`
- `recovery`: `safe rewrite`, score 127, lane `watch`
- `stale`: `change width`, score 206, lane `ship`

## Note

The useful failure mode here is a wrong decision on a named case, not a vague style disagreement.
