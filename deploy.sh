#!/bin/bash

ssh tncars.pp.ua sudo systemctl stop bible-api
scp target/release/bible-api tncars.pp.ua:/srv/bible-api/
ssh tncars.pp.ua sudo systemctl start bible-api
