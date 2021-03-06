# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Addded

* (`fitsio`) add support for images which are not 2d

### Changed

* (`fitsio`) change function arguments that were previously `start` and `end` to a `range` `Range<usize>`

### Removed

## [0.11.1] - 2017-11-24

### Addded
### Changed

* (`fitsio`) Fixed problem with writing string columns

### Removed

## [0.11.0] - 2017-11-24

### Addded
### Changed

* (`fitsio`) Updated the documentation to be feature complete as of this version

### Removed

## [0.10.0] - 2017-11-07

### Added

* (`fitsio`) add `iter` [#46][pull-46]
* (`fitsio`) add `hdu_name`, `hdu_names`, `num_hdus`, and `delete` [#45][pull-45]
* (`fitsio`) add `copy_to` [#44][pull-44]
* (`fitsio`) add `insert_column`, `append_column`, and `delete_column` methods to `FitsHdu` [#43][pull-43]
* add contribution guide
* cfitsio license

### Changed

* (`fitsio`) **BREAKING CHANGE**: most methods require passing a mutable `FitsFile` to perform work
* (`fitsio`) Include `SBYTE_IMG`, `USHORT_IMG` and `ULONG_IMG` data types

### Removed

Nothing

## [0.9.0] - 2017-07-15

### Added

* (`fitsio`) Created unified error type `fitsio::errors::Error`
* (`fitsio`) Official (i.e. tested) support for the extended filename syntax
* (`fitsio`) Implemented support for generating the ffi bindings "live" using `bindgen` [#34][pull-34]
* (`fitsio`) Support (unsafely) accessing the underlying `fitsfile` pointer [#32][pull-32]
* (`fitsio`) Implement resizing images [#31][pull-31]

### Changed

* (`fitsio`) Removed _most_ unneeded `unwrap`s from the code
* (`fitsio`) Simplified the implementation of `buf_to_string`
* (`fitsio`) Include image data type in hdu info struct

### Removed

Nothing

[Unreleased]: https://github.com/mindriot101/rust-fitsio/compare/v0.11.1...HEAD
[0.9.0]: https://github.com/mindriot101/rust-fitsio/compare/v0.8.0...v0.9.0
[pull-34]: https://github.com/mindriot101/rust-fitsio/pull/34
[pull-32]: https://github.com/mindriot101/rust-fitsio/pull/32
[pull-31]: https://github.com/mindriot101/rust-fitsio/pull/31
[pull-43]: https://github.com/mindriot101/rust-fitsio/pull/43
[pull-44]: https://github.com/mindriot101/rust-fitsio/pull/44
[pull-45]: https://github.com/mindriot101/rust-fitsio/pull/45
[pull-46]: https://github.com/mindriot101/rust-fitsio/pull/46
[0.10.0]: https://github.com/mindriot101/rust-fitsio/compare/v0.9.0...v0.10.0
[0.11.0]: https://github.com/mindriot101/rust-fitsio/compare/v0.10.0...v0.11.0
[0.11.1]: https://github.com/mindriot101/rust-fitsio/compare/v0.10.0...v0.11.1

---

vim: ft=markdown:textwidth=0:wrap:nocindent
