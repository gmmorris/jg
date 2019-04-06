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

    if [ -z "$RPMISE" ]
    then
        echo "Skipping RPM"
    else
        echo "RPMising"
        cargo install cargo-rpm
        sed -i "s/\$TARGET/$TARGET/g" $src/Cargo.toml
        cat $src/Cargo.toml
        cargo rpm build -v
        mv target/$TARGET/release/rpmbuild/RPMS/$RPMISE/*.rpm $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.rpm
    fi

    cp target/$TARGET/release/jg $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
