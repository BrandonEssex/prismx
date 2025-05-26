## Code Changes

- `zen/journal.rs`
  - Add new enum variant `Attachment` for images
  - Insert entries when image files dropped or pasted
- `zen/render.rs`
  - Render image attachments as inline graphics
  - Limit width to fit terminal
- `ui/components/feed.rs`
  - Update layout logic for image padding and caption input
- `state/media.rs` (new)
  - Parse dropped files or clipboard data
  - Sanitize filenames and verify MIME type
- `config/settings.toml`
  - Add toggle for `media_mode` to enable/disable image parsing
