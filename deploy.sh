#!/bin/bash

cargo build --release
scp target/release/bible-api berinaniesh.xyz:/home/berinaniesh/tmp/
ssh berinaniesh.xyz deploy-bible-api.sh
