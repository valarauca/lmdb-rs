lmdb-rs
=======

Rust bindings for [LMDB](http://symas.com/mdb/)

[Documentation (master branch)](http://valarauca.github.io/lmdb-rs/lmdb_rs/)

Building
========

LMDB is bundled as submodule so update submodules first:

`git submodule update --init`

And then

`cargo build`

Notes
========

This is a fork of [vhbit crate](https://github.com/vhbit/lmdb-rs)

In short I ran into a lot of headaches with

* Bad life time management.
* Transaction handles not being dropped.
* No support for zero-sized types which make building abstractions above
this project difficult.
* Panicing on non-utf8 strings.

This is my attempt at cleaning things up slightly.

There is still a _fair_ amount of unsafety. Hopefully in the future I'll be
able to release a higher level crate with _saner_ bindings above this.
