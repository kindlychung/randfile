#!/usr/bin/env bash

target/debug/randfile --help
echo -e "\n"
target/debug/randfile --startwith=tmp_ rs /tmp
