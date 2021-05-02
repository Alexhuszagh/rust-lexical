set -ex

main() {
    local target=
    if [ $TRAVIS_OS_NAME = linux ]; then
        target=x86_64-unknown-linux-musl
        sort=sort
    else
        target=x86_64-apple-darwin
        sort=gsort  # for `sort --sort-version`, from brew's coreutils.
    fi

    # Builds for iOS are done on OSX, but require the specific target to be
    # installed.
    # TODO(ahuszagh, priority=low) Remove 32-bit Apple builds when we
    # remove support for Rustc <= 1.41.0.
    case $TARGET in
        aarch64-apple-ios)
            rustup target install aarch64-apple-ios
            ;;
        armv7-apple-ios)
            rustup toolchain install 1.41.0 --target=armv7-apple-ios
            rustup default 1.41.0
            ;;
        armv7s-apple-ios)
            rustup toolchain install 1.41.0 --target=armv7s-apple-ios
            rustup default 1.41.0
            ;;
        i386-apple-ios)
            rustup toolchain install 1.41.0 --target=i386-apple-ios
            rustup default 1.41.0
            ;;
        x86_64-apple-ios)
            rustup target install x86_64-apple-ios
            ;;
        i686-apple-darwin)
            rustup toolchain install 1.41.0 --target=i686-apple-darwin
            rustup default 1.41.0
            ;;
    esac

    # This fetches latest stable release
    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/japaric/cross \
                       | cut -d/ -f3 \
                       | grep -E '^v[0.1.0-9.]+$' \
                       | $sort --version-sort \
                       | tail -n1)
    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git japaric/cross \
           --tag $tag \
           --target $target
}

main
