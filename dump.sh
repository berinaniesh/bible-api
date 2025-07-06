#!/bin/bash
# Usage: ./dump_pg.sh your_db_name

set -e

DB_NAME="$1"
OUT_FILE="sql/latest_full.sql.zst"

if [ -z "$DB_NAME" ]; then
  echo "Usage: $0 your_db_name"
  exit 1
fi

pg_dump --no-owner "$DB_NAME" | zstd -o "$OUT_FILE"
echo "Database '$DB_NAME' dumped and compressed as $OUT_FILE"
