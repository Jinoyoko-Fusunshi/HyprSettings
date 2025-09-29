#!/bin/bash

gpg --import signing/gpg-key.asc
gpg --verify signing/SHA256SUMS.asc signing/SHA256SUMS
sha256sum -c signing/SHA256SUMS