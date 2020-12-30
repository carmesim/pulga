#!/usr/bin/env bash
echo "Testing Pulga under Valgrind"

# Making sure we're in the directory where this script is contained
PRG="$0"
while [ -h "$PRG" ] ; do
   PRG=$(readlink "$PRG")
done

scriptdir=$(dirname "$PRG")
cd $scriptdir
#

if [[ ! -f "./target/release/pulga" ]]; then
    echo "you need to run \"cargo build --release\" first"
    exit 1
fi

# Run it under Valgrind, fail if Valgrind reports any error
valgrind --leak-check=full --error-exitcode=123 ./target/release/pulga

if [[ "$?" != '123' ]]; then
    echo ''
    echo 'Valgrind did not find any errors'
else
    exit 127
fi
