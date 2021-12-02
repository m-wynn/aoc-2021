#!/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Usage: ./newday.sh <daynum>"
    exit 1
fi

NUMBER="$1"
sed -i"" "s/^]/  \"day${NUMBER}\",\n]/" Cargo.toml
cargo new "day${NUMBER}"
for file in $(find ./day-template -type f); do
    path="./day${NUMBER}/${file##*day-template/}";
    NUMBER="$NUMBER" envsubst < "$file" > "$path"
    echo "putting $file in $path"
done
