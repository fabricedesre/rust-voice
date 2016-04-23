PocketSphinx Rust bindings
==========================

The `pocketsphinx-sys` crate provides FFI declarations and linkage
for the voice recognition `pocketsphinx`C library.

Following the `*-sys` package conventions,
the `pocketsphinx-sys` create does not define higher-level abstractions over
the native `pocketsphinx` library functions.

See `https://github.com/kriomant/pocketsphinx-rs` for higher-level bindings.

Dependencies
------------

In order to use the this crate, you must have the `libpocketsphinx` library
installed where it can be found by `pkg-config`.

On Debian-based Linux distributions, install the `libpocketsphinx1` package:

```
sudo apt-get install libpocketsphinx1
```

On OS X, install `cmu-pocketsphinx` with Homebrew:

```
brew install --HEAD cmu-sphinxbase
brew install --HEAD cmu-pocketsphinx
```

For building custom version of CMU PocketSphinx refer to
[official building documentation][ps-build].

Usage
-----

Add `pocketsphinx-sys` as a dependency in `Cargo.toml`:

```toml
[dependencies]
pocketsphinx-sys = "0.4.0"
```

Import the `pocketsphinx_sys` crate and use the functions.

```rust
extern crate pocketsphinx_sys;
```

Documentation
-------------

Since `pocketsphinx-sys` does nothing more than export symbols from the native `libpocketsphinx` library, the best source for help is the information already available for the *pocketsphinx* project:

* [Homepage](http://cmusphinx.sourceforge.net/)
* Source Code for [pocketsphinx](https://github.com/cmusphinx/pocketsphinx) and [sphinxbase](https://github.com/cmusphinx/sphinxbase)
* [Wiki](http://cmusphinx.sourceforge.net/wiki)
* [Doxygen documentation](http://cmusphinx.sourceforge.net/doc/pocketsphinx)

License
-------

Copyright © 2016 Mikhail Trishchenkov

Distributed under the [MIT License](LICENSE).

*Note:* By using this crate, your executable will link to the `libpocketsphinx` C library, which is available
under the [simplified BSD license](https://github.com/cmusphinx/pocketsphinx/blob/master/LICENSE).

[ps-build]: http://cmusphinx.sourceforge.net/wiki/tutorialpocketsphinx
