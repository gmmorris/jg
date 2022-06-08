export RELEASE_TAG=`cargo pkgid | cut -d# -f2 | cut -d: -f2`
git tag "v$RELEASE_TAG"
git push --tags