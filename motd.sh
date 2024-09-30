#!/usr/bin/env bash

set -e

BASE_DIR=$(dirname "$(readlink -f "$0")")
cd $BASE_DIR
# Set MOTD_ENV to 'debug' by default, but allow it to be overridden
MOTD_ENV=${MOTD_ENV:-debug}

# Determine the correct binary based on MOTD_PROFILE
if [ "$MOTD_PROFILE" = "release" ]; then
    MOTD_BINARY="target/release/fancy-motd"
else
    MOTD_BINARY="target/debug/fancy-motd"
    cargo build --quiet
fi

$MOTD_BINARY