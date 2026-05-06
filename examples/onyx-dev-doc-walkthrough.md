# Onyx Dev Doc Deck Walkthrough

The fixture is intentionally compact, so the review starts with the cases that pull farthest apart.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 185 | ship |
| stress | diagnostic quality | 177 | ship |
| edge | review cost | 152 | ship |
| recovery | safe rewrite | 127 | watch |
| stale | change width | 206 | ship |

Start with `stale` and `recovery`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

If `recovery` becomes less cautious without a clear reason, I would inspect the drag input first.
