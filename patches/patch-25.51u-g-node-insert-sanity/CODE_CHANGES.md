## Code Changes

### 1. Fix `Enter` Insertion Logic (sibling)

- Ensure selected node exists:
```rust
if let Some(current_id) = state.selected {
    let parent = state.nodes.get(&current_id).and_then(|n| n.parent);
    // insert new node with parent = parent
}
Prevent insertion if no selection or invalid tree context:
if state.selected.is_none() { return; }
2. Fix Tab Insertion Logic (child)
Only allow if selected node exists and is still valid
Ensure parent-child link is formed:
state.nodes.get_mut(&selected_id).unwrap().children.push(new_node_id);
3. Improve Fallback Loop Protection
Maintain a set fallback_promoted_this_session: HashSet<NodeID>
Before promoting:
if already_in_roots || no_children || fallback_promoted_this_session.contains(id) {
    continue;
}
On promotion:
fallback_promoted_this_session.insert(*id);
Clear fallback_promoted_this_session on:
Reset
Successful drill/pop
Root manually changed
4. Fix Grid Position Collision
After insertion (when !auto_arrange), call:
state.ensure_grid_positions();
Prevents overlapping nodes at (0,0)
5. Validate and Rebuild Orphan Chains
In recalculate_roles():
Reassign LayoutRole::Free to orphaned nodes
Promote them only in dev mode or fallback
