#!/usr/bin/env bash
set -e

new_lines=()
mapfile -t lines < src/entities/prelude.rs2

for line in "${lines[@]}"; do
    new_lines+=("${line}")

    if echo "${line}" | grep -q '^pub'; then
        module="$(echo "${line}" | sed 's|^pub use super::||' | sed 's|::.*$||')"
        struct="$(echo "${line}" | sed 's|^.* ||' | sed 's|;$||')"
        new_lines+=("pub use super::${module}::Column as ${struct}Column;")
    fi
    new_lines+=('')
done

for line in "${new_lines[@]}"; do
    echo "${line}"
done
# vim: set sw=4 expandtab:
