export DOCKER_DEFAULT_PLATFORM=linux/arm32v7

docker build -t oxide-weston -f ./tools/weston.dockerfile .
# if it fails, it already exists probably
docker create --name oxide-weston oxide-weston || true

docker cp -L oxide-weston:/tmp/wayland/ ./build/lib/

mkdir -p ./build/lib
mkdir -p ./build/bin

docker container rm oxide-weston
