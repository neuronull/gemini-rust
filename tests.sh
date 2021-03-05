#!/bin/bash

#cargo test $1 -- --nocapture --test-threads=1
cargo test $1 -- --test-threads=1
