#!/bin/bash

set -ex

git checkout -f *.gir
./fix.sh

[ -x ~/.cargo/bin/rustdoc-stripper ] || cargo install rustdoc-stripper

# Use downloaded rustdoc-stripper 
export PATH=$HOME/.cargo/bin:$PATH

[ -n "$CI_COMMIT_BRANCH" ] || export CI_COMMIT_BRANCH='main'
if [ -z "$CI_PROJECT_URL" ]; then 
  echo "Not running in CI, faking project URL"
  export CI_PROJECT_URL='https://example.com'
fi

./generator.py --embed-docs
eval $(./gir-rustdoc.py --branch=$CI_COMMIT_BRANCH pre-docs)
cargo doc --all-features --no-deps
./gir-rustdoc.py html-index
