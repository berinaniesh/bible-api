#!/bin/bash

cargo build --release
scp target/release/bible-api tncars.pp.ua:/srv/bible-api/bible-api.new
ssh tncars.pp.ua cp /srv/bible-api/bible-api /srv/bible-api/bible-api.old
ssh tncars.pp.ua sudo systemctl stop bible-api
ssh tncars.pp.ua cp /srv/bible-api/bible-api.new /srv/bible-api/bible-api
ssh tncars.pp.ua sudo systemctl start bible-api
