export TRAVIS_TAG=`cargo pkgid | cut -d# -f2 | cut -d: -f2`
git tag "$TRAVIS_TAG"
git push --tags