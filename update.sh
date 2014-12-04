#!/bin/sh

set -x

cd $(dirname $0)

git submodule update --init --merge
git submodule foreach git pull origin master

cd rust

echo "<title>BitRust - Breaking Changes in rust-lang/rust</title>" > ../index.html
echo '<pre>' >> ../index.html
git log -n100 --grep '\[breaking-change\]' >> ../index.html
echo '</pre>' >> ../index.html

cd ..

git commit --author='BitRust Bot <bitrustbot@octarineparrot.com>'  -am 'Automatic update'
git push origin gh-pages