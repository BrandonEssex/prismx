## Code Changes

- Add new enum `NodeType` in `node.rs`:
  ```rust
  pub enum NodeType {
      Text,
      Task,
      Tag(String),
      Link(String),
      Command(String),
      DBRecord { id: String, source: String },
  }
Add a field to Node:
pub node_type: NodeType
Update Node::new() to default to NodeType::Text
In render_gemx() or relevant draw pass:
Add visual cues based on type (e.g., "📎" for Link, "☑" for Task)
Prepare for Spotlight filtering and command matching by type
