#!/usr/bin/env bash

. bt.sh                            # source bt.sh
bt_init                            # initialize

# Don't change! We want predictable outputs
export LANG="en_US.UTF-8"

# Dir of this scrip
BASE_DIR=$(dirname "$(readlink -f "$0")")
export BASE_DIR

# Set config path
if [[ -z ${1+x} ]]; then
    export CONFIG_PATH="${BASE_DIR}/config.sh"
else
    export CONFIG_PATH="$1"
fi

# Source the framework
source "${BASE_DIR}/framework.sh"

# find a grep with pcre support (-P)
if command -v ggrep >/dev/null; then
  grep="ggrep"
else
  grep="grep"
fi

# Run the modules and collect output
bt_start "determine modules"
# shellcheck disable=SC2010
modules="$(ls -1 "${BASE_DIR}/modules" | $grep -P '^(?<!\d)\d{2}(?!\d)-' | $grep -v $(printf " -e %s" "${exclude_modules[@]}"))"
bt_end "determine modules"

# keep track of pids in order
module_pids=()
# keep track of output files for each pid
declare -A module_outputs

bt_start "call modules"
while read -r module; do
    bt_start "call ${module}"
    module_path="${BASE_DIR}/modules/${module}"

    # create temp file to output to
    output_file=$(mktemp)

    # run in background, and save to output
    "${module_path}" > "$output_file" 2> /dev/null &
    pid=$!
    # remember the pid and output file
    module_pids+=($pid)
    module_outputs[$pid]=$output_file

    bt_end "call ${module}"
done <<< "${modules}"
bt_end "call modules"

# iterate over module outputs to build output
output=""
for pid in "${module_pids[@]}"; do
    bt_start "wait ${pid}"
    wait "$pid"
    bt_end "wait ${pid}"
    
    bt_start "read ${pid} output"
    module_output=$(cat "${module_outputs[$pid]}")
    output+="$module_output"
    bt_end "read ${pid} output"
    # remove the temp file
    rm -f "${module_outputs[$pid]}"
done

# Print the output in pretty columns
bt_start "columnize"
columnize "${output}" $'\t' $'\n'
bt_end "columnize"

if [[ -n "$BENCHMARK" ]]; then
  bt_cleanup                         # cleanup once when done
fi
