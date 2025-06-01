## Code Changes

- Add `was_submitted: bool` guard in input state
- Only accept new entry if `was_submitted == false`
- Reset guard after key cycle
