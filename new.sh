#!/bin/bash

year=${1:-$(date +"%Y")}

echo "Working on year $year"

echo -n "Create day: "
read day

printf -v name 'day%02d' "$day"
mkdir -p $year/$name

cp -r template/. $year/$name/
sed -i "s/project-name/$name/g" $year/$name/Cargo.toml
