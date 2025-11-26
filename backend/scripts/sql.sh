#!/usr/bin/env bash
set -euo pipefail

if [ -f .env ]; then
    export $(grep -v '^#' .env | grep '=' | xargs)
fi

if [ -z "${DATABASE_URL:-}" ]; then
    echo "DATABASE_URL not set in environment or .env"
    exit 1
fi

# Expected: postgres://user:pass@host:port/db
proto_removed="${DATABASE_URL#*://}"

creds="${proto_removed%@*}"
hostdb="${proto_removed#*@}"

user="${creds%%:*}"
pass="${creds#*:}"
pass="${pass%%@*}"

hostport="${hostdb%%/*}"
dbname="${hostdb#*/}"
dbname="${dbname%%\?*}"

host="${hostport%%:*}"
port="${hostport#*:}"
[ "$host" = "$port" ] && port=5432

username="$(whoami)"

exec psql -U "$user" -d "$dbname" -h "$host" -p "$port"
