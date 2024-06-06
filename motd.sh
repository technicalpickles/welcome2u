#!/usr/bin/env bash

BASE_DIR=$(dirname "$(readlink -f "$0")")
. "$BASE_DIR/bt.sh"
bt_init

bt_start "running ruby version"
/usr/bin/ruby "$BASE_DIR/motd.rb"
bt_end "running ruby version"

# bt_cleanup
