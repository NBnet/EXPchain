#!/usr/bin/env bash

######################################################

docker_file=$1
dir="/tmp/NB_DOCKER_RUNTIME_BUILD_DIR_${RANDOM}_$(date +%s)"

mkdir -p ${HOME}/__NB_DATA__/usr_local_bin || exit 1
chmod -R 1777 ${HOME}/__NB_DATA__ # allow some unknown files, `|| exit 1`

mkdir $dir || exit 1
cd $dir || exit 1
cp ~/.ssh/authorized_keys ./ || exit 1
cp $docker_file ./Dockerfile || exit 1

######################################################

docker images --format=json | grep '"Tag":"\\u003cnone\\u003e"' | jq '.ID' | xargs docker image rm

if [ 0 -eq $(docker images --format json | jq '.Tag' | grep -c 'nbnet_24.04') ]; then
    docker pull ubuntu:24.04 || exit 1
    docker tag ubuntu:24.04 ubuntu:nbnet_24.04 || exit 1
fi

which docker
docker build -t ubuntu:nbnet_runtime_v0 . || exit 1

docker rm -f nbnet_runtime

docker run --restart always --rm -d --network=host \
    -v ${HOME}/__NB_DATA__:/tmp \
    --name nbnet_runtime \
    ubuntu:nbnet_runtime_v0 || exit 1

docker ps

######################################################

rm -rf $dir