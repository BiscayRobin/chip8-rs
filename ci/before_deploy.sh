# This script takes care of building your crate and packaging it for release

set -ex

git fetch --tags

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

    # TODO Update this to build the artifacts that matter to you
    cross rustc --bin chip8-rs --target $TARGET --release -- -C lto

    
    # TODO Update this to package the right artifacts
    case $TARGET in
	*windows*)
	    cp target/$TARGET/release/chip8-rs.exe $stage/
	    ;;
	*)		
    	    cp target/$TARGET/release/chip8-rs $stage/
	    ;;
    esac
    cp $src/README.md $stage/
    cp -r $src/c8games $stage/

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
