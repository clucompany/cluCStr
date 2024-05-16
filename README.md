<div id="header" align="center">

  <b>[clucstr]</b>
  
  ( Safe and efficient creation of "CStr" with zero-byte checking and support for concatenating multiple values. )
  </br></br>

<div id="badges">
  <a href="./LICENSE">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/apache2.png?raw=true" alt="apache2"/>
  </a>
  <a href="https://crates.io/crates/cluCStr">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/cratesio.png?raw=true" alt="cratesio"/>
  </a>
  <a href="https://docs.rs/cluCStr">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/docrs.png?raw=true" alt="docrs"/>
  </a>
  <a href="https://github.com/denisandroid">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/uproject.png?raw=true" alt="uproject"/>
  </a>
  <a href="https://github.com/clucompany">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/clulab.png?raw=true" alt="clulab"/>
  </a>

  [![CI](https://github.com/clucompany/cluCStr/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/clucompany/cluCStr/actions/workflows/CI.yml)

</div>
</div>

## Note

<b>You can use `c"wow"` since Rust 1.77.0 instead of `cstr!("wow")` from this crate. This new feature provides more concise code and faster compilation. If you are using an older Rust API (like 1.66), this crate will still be relevant for some time.</b>

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
clucstr = "1.2.0"
```

and this to your source code:

```rust
use cluCStr::cstr;
use core::ffi::CStr;
```

## Example

```rust
use cluCStr::cstr;
use core::ffi::CStr;

fn main() {
	let cstr = cstr!(b"How are you?");
	
	assert_eq!(cstr.to_bytes_with_nul(), b"How are you?\0");
}
```

<a href="./examples">
  See all
</a>

## License

This project has a single license (LICENSE-APACHE-2.0).

<div align="left">
  <a href="https://github.com/denisandroid">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/uproject.png?raw=true" alt="uproject"/>
  </a>
  <b>&nbsp;Copyright (c) 2019-2024 #UlinProject</b>

  <b>&nbsp;(Denis Kotlyarov).</b>
  </br></br></br>
</div>

### Apache License

<div align="left">
  <a href="./LICENSE">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/apache2.png?raw=true" alt="apache2"/>

  </a>
  <b>&nbsp;Licensed under the Apache License, Version 2.0.</b>
  </br></br></br></br>
</div>
