#!/bin/bash

echo -n "Create day: "
read day

printf -v name 'day%02d' "$day"
mkdir $name

cp -r template/. $name/
sed -i "s/project-name/$name/g" $name/Cargo.toml
