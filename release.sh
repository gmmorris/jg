export TRAVIS_TAG=`cargo pkgid | cut -d# -f2 | cut -d: -f2`
git tag "$TRAVIS_TAG.osx"
git tag "$TRAVIS_TAG.linux"
git push --tags