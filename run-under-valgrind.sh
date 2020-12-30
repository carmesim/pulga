#!/bin/bash
echo "Testing Pulga under Valgrind"

# Making sure we're in the directory where this script is contained
PRG="$0"
while [ -h "$PRG" ] ; do
   PRG=$(readlink "$PRG")
done
scriptdir=$(dirname "$PRG")
cd $scriptdir

# Run it under Valgrind, fail if Valgrind reports any error
if valgrind --leak-check=full --error-exitcode=23 ./target/release/pulga; then
    echo "Valgrind did not find any errors!"
else
    exit ${127}
fi

