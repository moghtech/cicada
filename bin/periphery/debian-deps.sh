#!/bin/bash

## Periphery deps installer

apt-get update
apt-get install -y ca-certificates fuse3
rm -rf /var/lib/apt/lists/*

