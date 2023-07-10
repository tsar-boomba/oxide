export DOCKER_DEFAULT_PLATFORM=linux/arm64

# Fail on err
set -e

docker build -t oxide-arm-build -f tools/arm-build.dockerfile .

docker create --name oxide-arm-build oxide-arm-build

mkdir -p target
docker cp oxide-arm-build:/target/armv7-unknown-linux-gnueabihf/ target/

docker container rm oxide-arm-build
