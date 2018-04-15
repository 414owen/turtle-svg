#!/usr/bin/env bash

msg=$1
col=$2
echo "$(tput setaf $col)$msg$(tput init)"
