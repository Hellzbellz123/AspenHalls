#!/bin/bash

# Run avahi-browse command and store the output in a variable
avahi_output=$(avahi-browse --terminate --resolve _adb-tls-connect._tcp)

# Extract the port from the avahi-browse output
port=$(echo "$avahi_output" | grep -oP 'port = \[\K[0-9]+')

# Check if the port is not empty
if [ -n "$port" ]; then
    # Extract the IP address from the avahi-browse output
    ip_address=$(echo "$avahi_output" | grep -oP 'address = \[\K[0-9.]+')

    # Form the adb connect command with the extracted IP address and port
    adb_command="adb connect $ip_address:$port"

    # Print the adb connect command
    echo "Executing: $adb_command"

    # Execute the adb connect command
    eval "$adb_command"
else
    echo "Error: Port not found in avahi-browse output."
fi
