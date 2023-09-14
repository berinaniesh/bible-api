#!/bin/bash

ssh tncars.pp.ua sudo systemctl stop bible-api
scp target/release/bible-parser tncars.pp.ua:/srv/bible/
ssh tncars.pp.ua sudo systemctl start bible-api
