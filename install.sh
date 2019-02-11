#!/bin/bash 
VERSION=v0.5.4

if [[ "$OSTYPE" == "linux-gnu" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/webbrandon/mc/releases/download/${VERSION}/debian
  echo "Installing Master of Ceremony ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/mc
elif [[ "$OSTYPE" == "cygwin" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/webbrandon/mc/releases/download/${VERSION}/debian
  echo "Installing Master of Ceremony ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/mc
elif [[ "$OSTYPE" == "debian"* ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/webbrandon/mc/releases/download/${VERSION}/debian
  echo "Installing Master of Ceremony ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/mc
elif [[ "$OSTYPE" == "msys" ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/webbrandon/mc/releases/download/${VERSION}/debian
  echo "Installing Master of Ceremony ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/mc
elif [[ "$OSTYPE" == "freebsd"* ]]; then
  echo "Downloading debian client."
  curl -LO https://github.com/webbrandon/mc/releases/download/${VERSION}/debian
  echo "Installing Master of Ceremony ${VERSION}"
  chmod +x debian
  mv debian /usr/local/bin/mc
elif [[ "$OSTYPE" == "darwin"* ]]; then
  echo "Downloading darwin client."
  curl -LO https://github.com/webbrandon/mc/releases/download/${VERSION}/darwin
  echo "Installing Master of Ceremony ${VERSION}"
  chmod +x darwin
  mv darwin /usr/local/bin/mc
else
  printf "System not supported. Attempting to build from source. You must have rust installed."
  curl -LO https://github.com/webbrandon/mc/archive/${VERSION}.tar.gz
  tar -xvzf ${VERSION}.tar.gz
  cd ${VERSION}
  cargo build --release
  chmod +x target/release/mc
  mv target/release/mc /usr/local/bin/mc
  cd ..
  rm -rf mc-${VERSION}
fi

exit 0