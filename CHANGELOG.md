# v1.2.2

*2024-03-10*

Fixes:

* Make dependency on 'time' optional ([#38](https://github.com/progval/unicode_names2/pull/38/))
* Remove 'cargo:rerun-if-changed=generator/' ([b25d7610dcd](https://github.com/progval/unicode_names2/commit/b25d7610dcdb1b4f8f0e461a8c7570370e55478f))

Internal:

* ci: On non-nightly, build all crates instead of only generator ([#39](https://github.com/progval/unicode_names2/pull/39))

# v1.2.1

*2023-12-14*

Internal:

* include required license texts in all published crates ([#35](https://github.com/progval/unicode_names2/pull/35))

# v1.2.0

*2023-10-14*

Data:

* Update data to Unicode 15.1 ([#32](https://github.com/progval/unicode_names2/pull/32))

Fixes:

* Fix i686 host builds ([#34](https://github.com/progval/unicode_names2/pull/34))
* Fix support for platforms using CRLF ([#31](https://github.com/progval/unicode_names2/pull/31) and [#33](https://github.com/progval/unicode_names2/pull/33))

Internal:

* Fixes lint issues ([#33](https://github.com/progval/unicode_names2/pull/33))

# v1.1.0

*2023-08-13*

Feature:

* Add alias search fallback to character() ([#12](https://github.com/progval/unicode_names2/pull/12))
  (this adds a dependency on the `ph` crate and is slightly slower when requesting an alias
  or non-existing character, this might be fixed in a future release by merging the two hash functions)

Internal:

* Avoid unnecessary rebuilds when neither data/ nor generator/ changed
* Add example executables to get name from character and vice versa ([#25](https://github.com/progval/unicode_names2/pull/15))

# v1.0.0

*2023-08-13*

Breaking:

* Bump minimum Rust version from 1.48 to 1.63
* Bump dependencies ([#22](https://github.com/progval/unicode_names2/pull/22), [#23](https://github.com/progval/unicode_names2/pull/23))

Features:

* Build the perfect-hash function deterministically ([#13](https://github.com/progval/unicode_names2/pull/13)
* Build the perfect-hash function at compile time from unicode data, instead of being
  in version control and shipped on crates.io ([#17](https://github.com/progval/unicode_names2/pull/17))

Internal:

* Run Rustfmt + Clippy on CI and fix warnings ([#14](https://github.com/progval/unicode_names2/pull/14), [#15](https://github.com/progval/unicode_names2/pull/15))

# v0.6.0

*2022-10-13*

Data:

* Update data for Unicode 15 ([#10](https://github.com/progval/unicode_names2/pull/10))

# v0.5.1

*2022-08-09*

Bug fixes:

* Fix panic when character() is passed a string over 88 chars ([#7](https://github.com/progval/unicode_names2/pull/7))
* Fix compilation warnings ([#5](https://github.com/progval/unicode_names2/pull/5))

Internal:

* Replace Travis with Github Workflows as CI ([#8](https://github.com/progval/unicode_names2/pull/8))
* Run CI on sub-crates ([#9](https://github.com/progval/unicode_names2/pull/9))

# v0.5.0

*2022-02-06*

Data:

* Update to Unicode 14.0.0 ([#4](https://github.com/progval/unicode_names2/pull/4))
