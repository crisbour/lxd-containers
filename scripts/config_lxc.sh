#!/usr/bin/sh

lxc init images:ubuntu/focal container1

lxc config set container1 security.nesting true

lxc start container1


