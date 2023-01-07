#!/bin/bash

echo "test start"

section=$1
target=$2
echo $target 


input_texts=$(echo $(find ./src/$section/$target/input -name "*.txt" -type f))

for input_text in $input_texts; do
    echo "--------------------"
    echo "target: $input_text"
    answer=$(echo $input_text | rev | cut -d/ -f1 | rev)
    data=$( cargo run --bin $target < $input_text )
    echo "===================="
    echo "output = $data"
    echo "answer = $(cat ./src/$section/$target/ans/$answer)"
done