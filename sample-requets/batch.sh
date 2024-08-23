#!/bin/bash

# Check if the correct number of arguments is provided
if [ $# -ne 2 ]; then
    echo "Usage: $0 <count> <command>"
    exit 1
fi

# Assign command line arguments to variables
count=$1
cmd=$2

# Check if count is a positive integer
if ! [[ "$count" =~ ^[0-9]+$ ]] || [ "$count" -eq 0 ]; then
    echo "Error: Count must be a positive integer"
    exit 1
fi

# Loop to execute the command the specified number of times
for ((i=1; i<=$count; i++))
do
    echo "Execution $i of $count:"
    eval $cmd
    echo "------------------------"
done

echo "Finished executing the command $count times."
