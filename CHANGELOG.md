# Changelog

## v0.4.1 (2025-04-30)

 * Update to [thiserror] 2.0 (PR [#11][PR11], thanks to [Samuel Collins (sd-collins)][sd-collins])
 * Add support for padding strings and chars (e.g. `%10s`) (PR [#13][PR13], thanks to [Samuel Collins (sd-collins)][sd-collins])

[PR11]: https://github.com/tjol/sprintf-rs/pull/11
[PR13]: https://github.com/tjol/sprintf-rs/pull/13

## v0.4.0 (2024-12-05)

 * `FormatElement` now borrows a `&str` instead of owning a `String`, leading to fewer allocations and a significant performance improvement (PR [#10][PR10], thanks to [Samuel Collins (sd-collins)][sd-collins]).
   - this is a __breaking API change__ to the lower-level v0.2 API
   - the original and primary (v0.1) API is unchanged

[PR10]: https://github.com/tjol/sprintf-rs/pull/10
[sd-collins]: https://github.com/sd-collins

## v0.3.1 (2024-07-14)

 * pointer types can be formatted with `%p` in the same way as `usize` (PR [#9][PR9], thanks to  [Flávio J. Saraiva (flaviojs)][flaviojs])

[PR9]: https://github.com/tjol/sprintf-rs/pull/9

## v0.3.0 (2024-05-31)

 * More standard string and character types (PR [#7][PR7], thanks to  [Flávio J. Saraiva (flaviojs)][flaviojs])
    * Support `CString` and `&CStr` for `%s`, assuming the're UTF-8 encoded
    * Support `u8` and `i8` (ASCII), `u16` (UCS-2) and `u32` (UCS-4) for `%c`

[PR7]: https://github.com/tjol/sprintf-rs/pull/7
[flaviojs]: https://github.com/flaviojs

## v0.2.1 (2024-02-12)

 * Fix accidental backwards-incompatible API change in v0.2.0

## v0.2.0 (2024-02-12)

 * Expose the some of the `sprintf::parser` module in the API to allow other to use the `parse_format_string` function (PR [#5][PR5], thanks to [David Alexander Bjerremose (DaBs)][DaBs])
 * `PrintfError` now implements `std::error::Error`

[PR5]: https://github.com/tjol/sprintf-rs/pull/5
[DaBs]: https://github.com/DaBs

## v0.1.4 (2023-09-10)

 * Fix parsing of `ll` length specifier (PR [#4][PR4], thanks to [Ido Yariv (codido)][codido])

[PR4]: https://github.com/tjol/sprintf-rs/pull/4
[codido]: https://github.com/codido

## v0.1.3 (2022-09-23)

 * Fix float rounding: 9.99 should round to 10.0, not 9.0. (Issue [#2][bug2], thanks to [Nicholas Ritchie][NicholasWMRitchie])

[bug2]: https://github.com/tjol/sprintf-rs/issues/2
[NicholasWMRitchie]: https://github.com/NicholasWMRitchie

## v0.1.2 (2021-11-06)

 * Fix formatting of large floats (PR [#1][PR1], thanks to [Kuba (pierd)][pierd]

[PR1]: https://github.com/tjol/sprintf-rs/pull/1
[pierd]: https://github.com/pierd

## v0.1.1 (2021-08-30)

 * Fix bug in padding of fixed-width fields

## v0.1.0 (2021-08-24)

 * Initial release
