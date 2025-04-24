ci:
  cargo xtask ci

publish:
  cargo xtask publish

example:
  cargo run --example user

pull-api pyrepo="../../Code/z/github.com/Nemo2011/bilibili-api": \
  (_pull-api-file pyrepo "live" ) \
  (_pull-api-file pyrepo "user" ) \
  (_pull-api-file pyrepo "video" ) \
  (_pull-api-file pyrepo "credential" )

_pull-api-file pyrepo name:
  cp {{ pyrepo }}/bilibili_api/data/api/{{ name }}.json ./bilibili-api-rs/src/api_info/{{ name }}.json
