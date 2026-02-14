#!/bin/bash

curl -fsSL https://claude.ai/install.sh | bash

rustup component add --toolchain nightly rustfmt
