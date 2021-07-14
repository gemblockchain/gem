#!/usr/bin/env bash
# This script is meant to be run on Unix/Linux based systems
set -e

echo "*** Start Gem node ***"

cd $(dirname ${BASH_SOURCE[0]})/..

mkdir -p ./.local

docker-compose down --remove-orphans
docker-compose run --rm --service-ports dev $@
