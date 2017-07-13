#!/bin/bash

# Basic script to update the image with the new executable
set -ex

project=$(cat ../Cargo.toml | grep "name" | cut -d "\"" -f 2)
exe="ci_build"
# Make version align with Cargo.toml version
version=$(cat ../Cargo.toml | grep "version" | cut -d "\"" -f 2)
arch="amd64"
os="linux"

aci_name="${exe}-${version}-${os}-${arch}.aci"

if [[ -f ${aci_name} ]]; then
    echo "Overwriting"
    rm ${aci_name}
fi

acbuild begin
acbuild set-name maccoda.io/${project}
acbuild copy ../target/debug/${exe} /bin/${exe}
acbuild set-exec /bin/${exe}
acbuild label add version ${version}
acbuild label add arch ${arch}
acbuild label add os ${os}
acbuild annotation add authors "Dylan Maccora <maccora17@gmail.com>"
acbuild write ${aci_name}
acbuild end
