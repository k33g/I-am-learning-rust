#!/bin/bash

#curl https://raw.githubusercontent.com/carloscuesta/gitmoji/master/src/data/gitmojis.json -o gitmojis.json
rm git.sh
node --experimental-fetch git.js 
chmod +x git.sh

