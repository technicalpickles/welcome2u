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
output=""

bt_start "determine modules"
# shellcheck disable=SC2010
modules="$(ls -1 "${BASE_DIR}/modules" | $grep -P '^(?<!\d)\d{2}(?!\d)-' | $grep -v $(printf " -e %s" "${exclude_modules[@]}"))"
bt_end "determine modules"

bt_start "call modules"
while read -r module; do
    bt_start "call ${module}"
    module_output=$("${BASE_DIR}/modules/${module}" 2> /dev/null)
    bt_end "call ${module}"
    if [ "$?" != 0 ]; then continue; fi

    bt_start "append $module output"
    output+="${module_output}"
    [[ -n "${module_output}" ]] && output+=$'\n'
    bt_end "append $module output"
done <<< "${modules}"
bt_end "call modules"

# Print the output in pretty columns
bt_start "columnize"
columnize "${output}" $'\t' $'\n'
bt_end "columnize"

if [[ -n "$BENCHMARK" ]]; then
  bt_cleanup                         # cleanup once when done
fi
