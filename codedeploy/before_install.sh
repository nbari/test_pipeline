#!/bin/sh
NOW=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

test -e /opt/app && mv /opt/app "/opt/app-${NOW}"

exit 0
