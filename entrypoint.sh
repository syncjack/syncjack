#!/bin/bash

exec "$@" &
PID=`jobs -p`

trap "kill -SIGQUIT $PID" INT
wait