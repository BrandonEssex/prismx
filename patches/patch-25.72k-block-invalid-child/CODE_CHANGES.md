## Code Changes

- Check current node content before allowing Enter/Tab to create new node
- If label is empty or untouched, suppress new node creation
- Prevent invalid tree writes that cause circular ancestry (`node X is its own ancestor`)
- Always pre-reserve layout space, but only populate tree after interaction
