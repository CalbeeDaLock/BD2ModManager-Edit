#!/usr/bin/env python3
import re
from pathlib import Path

# Directory containing your files
directory = Path(".")  # Current directory, change if needed

# List all PNG files in the directory
files = list(directory.glob("icon_char*.png"))

for f in files:
    p = f
    # Try short name: keep only the first group
    new = p.with_name(re.sub(r"icon_char(\w+)_\w+\.png", r"\1.png", p.name))
    
    # If file already exists, use fallback with both groups
    if new.exists():
        new = p.with_name(re.sub(r"icon_char(\w+)_(\w+)\.png", r"\1_\2.png", p.name))
    
    # Rename the file
    p.rename(new)

print("Renaming completed!")
