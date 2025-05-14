#!/bin/bash
# Author: Brandon Essex
# PrismX/GemX QA Bot - Project Folder Scaffold Script
# Run from: ./src

# Navigate to root
cd "$(dirname "$0")/../"

# Create all required folders
mkdir -p \
  src/action \
  src/input \
  src/node_tree \
  src/state \
  src/ui \
  src/config \
  src/plugin \
  assets/icons \
  docs \
  target \
  tmp \
  logs

# Create placeholder source files
touch \
  src/action/{action.rs,mod.rs} \
  src/input/{input.rs,mod.rs} \
  src/node_tree/{node.rs,mod.rs} \
  src/state/{app_state.rs,sidebar_view.rs,view.rs,mod.rs} \
  src/ui/{draw.rs,mod.rs} \
  src/config/{config.rs,plugin.json,mod.rs} \
  src/plugin/{plugin.rs,mod.rs} \
  src/main.rs \
  docs/README.md

echo "✅ Project structure and stub files created."

