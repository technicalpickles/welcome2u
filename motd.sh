#!/usr/bin/env bash

BASE_DIR=$(dirname "$(readlink -f "$0")")
cd $BASE_DIR
target/debug/fancy-motd
