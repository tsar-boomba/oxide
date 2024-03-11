export DOCKER_DEFAULT_PLATFORM=linux/arm32v7

docker build -t oxide-weston -f ./tools/weston.dockerfile .
# if it fails, it already exists probably
docker create --name oxide-weston oxide-weston || true

mkdir -p ./build/lib
mkdir -p ./build/bin

docker cp -L oxide-weston:/tmp/wayland ./build/lib/

docker container rm oxide-weston
