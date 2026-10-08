# Agent Instructions

## Git Push Policy

**Do NOT run `git push` automatically.**

Only push to remote when the user explicitly requests it. After making commits, stop and wait for the user to ask for the push.

### Correct workflow:
1. Make changes and commit locally
2. Tell the user what was committed
3. Wait for user to say "push" or "push the code"
4. Then run `git push`

### Never:
- Push without explicit user request
- Assume push is wanted after commits
- Push as part of regular workflow

This gives the user control over when code is shared remotely.
