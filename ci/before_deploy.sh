# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    cross rustc --bin jg --target $TARGET --release -- -C lto

    cp target/$TARGET/release/jg $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage

    if [ -z "$RPMISE" ]
    then
        echo "No RPMising configured"
    else
        echo "RPMising..."
        cargo install cargo-rpm
        cargo rpm build -v
        echo "Moving RPM from target/$TARGET/release/rpmbuild/RPMS/$RPMISE/$CRATE_NAME-$TRAVIS_TAG-1.$RPMISE.rpm to $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.rpm"
        cp target/$TARGET/release/rpmbuild/RPMS/$RPMISE/$CRATE_NAME-$TRAVIS_TAG-1.$RPMISE.rpm $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.rpm
    fi
}

main
