#!/bin/sh

systemctl is-active --quiet app && curl localhost/health
