#!/bin/bash 
VERSION=v0.2.1

if [[ "$OSTYPE" == "linux-gnu" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/webbrandon/mc/releases/download/${VERSION}/debian
  echo "Installing Master of Ceremony ${VERSION}"
  mv debian /usr/local/bin/mc
elif [[ "$OSTYPE" == "darwin"* ]]; then
  echo "Downloading darwin client."
  curl -LO https://github.com/webbrandon/mc/releases/download/${VERSION}/darwin
  echo "Installing Master of Ceremony ${VERSION}"
  mv darwin /usr/local/bin/mc
else
  printf "System not supported. Attempting to build from source. You must have rust installed."
  curl -LO https://github.com/webbrandon/mc/archive/${VERSION}.tar.gz
  tar -xvzf ${VERSION}.tar.gz
  cd ${VERSION}
  cargo build --release
  mv target/release/mc /usr/local/bin/mc
  cd ..
  rm -rf ${VERSION}
fi

exit 0