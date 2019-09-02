set -ex

main() {
    sudo apt-get install libgtest-dev
    cd /usr/src/gtest
    # We need super-user privileges to build in this directory,
    # however, travis only provides make and cmake in the local
    # env. Also, `sudo -E` does not work.
    sudo env "PATH=$PATH" cmake .
    sudo env "PATH=$PATH" make
    sudo mv libgtest* /usr/lib/
    cd "${TRAVIS_BUILD_DIR}"
}

main
