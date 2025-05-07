# PrismX Dashboard Module Documentation

## Overview

The PrismX Dashboard Module provides a customizable and interactive workspace within the PrismX TUI application. It supports user-defined widgets, persistent layouts, intuitive keyboard and mouse interactions, and seamless extension integration.

---

## Features

- **Customizable Layout:** Arrange, resize, and toggle visibility of widgets dynamically.
- **Persistent Configurations:** Save and load your personalized dashboard layouts using JSON files.
- **Advanced Widgets:** Pre-built widgets for Mindmaps, Pinned Items, and Extensions management.
- **Extension Support:** Manage extensions directly from the dashboard interface.
- **Enhanced Interaction:** Mouse drag-and-drop, keyboard navigation, and shortcuts.
- **Robustness:** Undo/Redo functionality, Grid Snapping, and Theming support.

---

## Configuration

Dashboard layouts and settings are persisted in JSON files under `data/dashboard_config.json`.

### Example configuration (`dashboard_config.json`):

```json
{
  "widgets": [
    {
      "widget_type": "mindmap",
      "position": {"x": 0, "y": 0},
      "size": {"width": 50, "height": 20},
      "enabled": true
    },
    {
      "widget_type": "pinned",
      "position": {"x": 50, "y": 0},
      "size": {"width": 30, "height": 10},
      "enabled": true
    },
    {
      "widget_type": "extensions",
      "position": {"x": 50, "y": 10},
      "size": {"width": 30, "height": 10},
      "enabled": true
    }
  ],
  "layout_version": "1.0",
  "grid_snap": 5
}
Built-in Widgets

Mindmap Widget
Interactive widget for visualizing and interacting with your mindmaps.

Pinned Items Widget
Quick access to frequently used notes, tasks, and project files.

Extensions Widget
Manage PrismX extensions. Activate, deactivate, and check for updates directly within your dashboard.

Keyboard Shortcuts

Shortcut	Action
Tab	Cycle through widgets
Enter	Activate or select a widget
Ctrl+W	Toggle visibility of selected widget
Ctrl+Z	Undo last action
Ctrl+Y or Ctrl+Shift+Z	Redo last undone action
Arrow Keys	Move selected widget
Widget Interaction (Mouse)

Drag Widgets: Click-and-hold left mouse button on widget borders and drag to reposition.
Resize Widgets: (Future Enhancement) Drag widget corner handles to resize widgets directly.
Extension Management

Extensions are managed in the data/extensions_state.json file.

Example Extension State:
{
  "extensions": [
    {
      "id": "mindmap_engine",
      "name": "Mindmap Engine",
      "version": "2.1.0",
      "enabled": true
    },
    {
      "id": "focus_timer",
      "name": "Focus Timer",
      "version": "1.3.2",
      "enabled": false
    }
  ]
}
Widget Theming

Widget colors and styles can be optionally customized via the data/widget_themes.json file.

Example Theme Configuration:
{
  "default": {
    "border_color": "white",
    "background_color": "black"
  },
  "mindmap": {
    "border_color": "blue",
    "background_color": "black"
  },
  "pinned": {
    "border_color": "green",
    "background_color": "black"
  },
  "extensions": {
    "border_color": "magenta",
    "background_color": "black"
  }
}
Guidance for Extension and Plugin Developers

PrismX supports custom widgets and plugins through a clearly defined API:

Required Widget Trait Methods
All widgets must implement the following trait methods:

new(config: &WidgetConfig)
init()
render(frame: &mut Frame, area: Rect)
handle_event(event: &Event)
get_area()
update_position(position: WidgetPosition)
activate()
Use the provided WidgetFactory pattern for instantiating new widgets based on user configuration.

Integration & Compatibility Notes

Ensure JSON configuration paths align with PrismX's global data storage convention.
Dashboard module integrates tightly with PrismX's logging (logging.rs) and error-handling (error_handling.rs) frameworks. Maintain compatibility if modifications are necessary.
Test integrations comprehensively using provided tests (tests/dashboard_tests.rs).
Performance & Optimization

This module is optimized for performance and has been rigorously tested under various conditions. For further enhancements:

Enable verbose logging and debugging via global PrismX settings.
Monitor performance metrics if extensive customization or widget counts are utilized.
End of Dashboard Module Documentation