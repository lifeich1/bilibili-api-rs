
# bilibili-api-rs

[![crates](https://img.shields.io/crates/v/bilibili-api-rs)](https://crates.io/crates/bilibili-api-rs)
[![license](https://img.shields.io/crates/l/bilibili-api-rs)](http://www.wtfpl.net/)
[![latest downloads](https://img.shields.io/crates/dv/bilibili-api-rs)](https://crates.io/crates/bilibili-api-rs)
[![recent downloads](https://img.shields.io/crates/dr/bilibili-api-rs)](https://crates.io/crates/bilibili-api-rs)
[![docs](https://img.shields.io/docsrs/bilibili-api-rs)](https://docs.rs/bilibili-api-rs)
<a href="https://gitmoji.dev">
  <img src="https://img.shields.io/badge/gitmoji-%20😜%20😍-FFDD67.svg?style=flat-square" alt="Gitmoji">
</a>

A rust library project got inspiration from [bilibili-api](https://github.com/Passkou/bilibili-api).

Latest released: 0.2.4

- No plan for cover all apis.
- "GET" like api only.


## Design

data:
- api
  - user::{info, channel\_series}, video::info, xlive::info

code:
- root: user_info, user_videos, xlive_list, ...
  - wbi: access(path, querymap, respschema)

## License

<a href="http://www.wtfpl.net/"><img
       src="http://www.wtfpl.net/wp-content/uploads/2012/12/wtfpl-badge-4.png"
       width="80" height="15" alt="WTFPL" /></a>
