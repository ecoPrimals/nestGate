#!/bin/bash

# NestGate UI Mock Disk Data Updater
# This script updates the mock disk data to show more realistic sizes for the demo

echo "Updating mock disk data with realistic sizes for demo..."

# Create temporary file
TMP_FILE=$(mktemp)

# Update Dashboard.tsx to use realistic disk sizes for mock data
cat > $TMP_FILE << 'EOF'
    {
      id: 'sda',
      status: 'PASSED',
      model: 'WDC WD141KFGX-68FH',
      serial: 'WD-13KY9T4K',
      size: '14.0 TB',
      temperature: 42
    },
    {
      id: 'sdb',
      status: 'PASSED',
      model: 'WDC WD141KFGX-68FH',
      serial: 'WD-13KY9T4L',
      size: '14.0 TB',
      temperature: 41
    },
    {
      id: 'sdc',
      status: 'PASSED',
      model: 'WDC WD141KFGX-68FH',
      serial: 'WD-13KY9T4M',
      size: '14.0 TB',
      temperature: 43
    }
EOF

# Replace the mock disk data in Dashboard.tsx
sed -i "/id: 'sda',/,/temperature: 43/{
  /id: 'sda',/,/temperature: 43/d
}" "src/routes/Dashboard.tsx"

# Find position to insert the new data
LINE_NUM=$(grep -n "const usingMockData = useMockData('websocket');" "src/routes/Dashboard.tsx" | cut -d ':' -f 1)
LINE_NUM=$((LINE_NUM + 5))  # Skip 5 lines to get to the beginning of the array

# Insert the new mock data
sed -i "${LINE_NUM}r $TMP_FILE" "src/routes/Dashboard.tsx"

echo "Mock disk data updated successfully."
echo "When running in mock mode, disks will now show as 14TB WDC WD141KFGX models."
echo "To run with the updated mock data: ./start-mock-mode.sh"

# Clean up temp file
rm $TMP_FILE 