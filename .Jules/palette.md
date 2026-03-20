## 2026-03-20 - Added tooltip hints for settings keyboard shortcuts
**Learning:** This egui-based settings UI has keyboard shortcuts, but they aren't discoverable. Adding `.on_hover_text()` to the checkboxes adds small tooltips to teach users the shortcuts without cluttering the UI.
**Action:** When an app uses keyboard shortcuts, always ensure they are discoverable via tooltips or hints in the corresponding UI elements.
