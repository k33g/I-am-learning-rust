#!/bin/bash

# https://gitmoji.dev/
# https://linuxize.com/post/bash-case-statement/

message=""

case $1 in

  art)
    message="Improve structure / format of the code. $2"
    ;;

  zap)
    message="Improve performance. $2"
    ;;

  memo)
    message=":memo: Add or update documentation. $2"
    ;;

  *)
    message="updated. $2"
    ;;
esac


git add .; git commit -m ":${1}: ${message}"; git push