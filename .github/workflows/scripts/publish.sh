#!/bin/sh

IMAGE=$1

# Source: https://github.com/rust-lang/cargo/issues/4082#issuecomment-422507510
SEMVER=`cargo pkgid | cut -d# -f2 | cut -d: -f2`
MAJOR="$(echo $SEMVER | cut -d'.' -f1)"
MINOR="$(echo $SEMVER | cut -d'.' -f2)"
PATCH="$(echo $SEMVER | cut -d'.' -f3)"

echo "Image: $IMAGE"
echo "Version: $SEMVER"

publish() {
    TAG=$IMAGE:$1

    echo "Publishing $TAG..."
    docker tag temp $TAG 
    docker push $TAG
}

publish $MAJOR
publish $MAJOR.$MINOR
publish $MAJOR.$MINOR.$PATCH
publish latest
