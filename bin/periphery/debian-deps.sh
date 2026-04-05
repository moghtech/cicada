#!/bin/bash

## Periphery deps installer

apt-get update
apt-get install -y fuse3
rm -rf /var/lib/apt/lists/*

