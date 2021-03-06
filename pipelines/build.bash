#!/bin/bash

cargo build --release	
cp target/release/mc darwin	

docker build -t mc:dev -f docker/Dockerfile.linux .	
docker create --name=mc-linux mc:dev	
docker cp mc-linux:/opt/mc/target/release/mc debian	
docker rm mc-linux