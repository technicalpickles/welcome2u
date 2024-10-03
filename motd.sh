#!/usr/bin/env bash

set -euo pipefail

# TODO: remove this once there aren't any shell scripts using it
BASE_DIR=$(dirname "$(readlink -f "$0")")

MOTD_PROFILE=${MOTD_PROFILE:-}
if [ -z "$MOTD_PROFILE" ]; then
    if [ "$PWD" = "$BASE_DIR" ]; then
        # If running in BASE_DIR, set to debug
        MOTD_PROFILE="debug"
    else
        # Otherwise, set to release
        MOTD_PROFILE="release"
    fi
fi
export MOTD_PROFILE

cd $BASE_DIR
MOTD_BINARY="target/$MOTD_PROFILE/welcome2u"
if [ "$MOTD_PROFILE" = "debug" ]; then
    cargo build --quiet
fi

"$MOTD_BINARY"