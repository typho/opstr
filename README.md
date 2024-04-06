# README

**author:** tajpulo <br/>
**version:** 1.0.0

## What is it about?

As a software developer, I often need to look at strings and apply operations to them.
I frequently use python on the commandline or resort to client-side web applications.
But the operations are always the same and should be accessible with one CLI call.

I built opstr, so you can throw a bunch of strings in and get the result of various operations out.
Or you specify an operation and get a predictable result.
It also simplifies to run string operations in your shell.

## Why should I use it?

To apply operations to strings.

## Who should use it?

Anyone working with text strings (in the Unicode sense, so as sequence of codepoints).

## How to install

Install me via crates.io:

```
cargo add opstr
```

## How to run

1. Go to https://github.com/typho/opstr
2. Click on the Releases link
3. Scroll down, choose the download appropriate for your platform
4. Once the download has finished, extract the files of the tar-gz archive
5. Add executable rights to the file of your platform
6. Run the executable opstr on the command line, example: `opstr --op utf8-bytes "hello"` to get `[104, 101, 108, 108, 111]`

## How to configure

Please lists the help menu to see all options to configure `opstr`.
Here I would like to mention that most options can also be provided as environment variable.
Hence you can avoid to specify the option at every CLI call, but one set them once.
The list of environment variables is:

* `OPSTR_RADIX`: the radix used for integers printed out
* `OPSTR_HEX_UPPER`: print hexadecimal alphabetic digits with uppercase letters, not lowercase letters
* `OPSTR_COLOR_SCHEME`: the color scheme for the output
* `OPSTR_LOCALE`: locale to use for locale-dependent operations (only `en-US` works per default)
* `OPSTR_SYNTAX`: the output representation syntax to use

Locales are tricky, because the executable would be impractically large if I ship all locales.
Instead, you need to generate locale data yourself; compare with [icu4x data management](https://github.com/unicode-org/icu4x/blob/main/tutorials/data_management.md) and replace `en-us` with your locale in this call:

`icu4x-datagen -W -o data/icu4x_en-us.blob2 --include-collations search-all --trie-type small --locales en-us --keys all --format blob`

The environment variable `OPSTR_LOCALE_DATAFILE` needs to point to the `.blob2` file to load and you need to specify the locale as CLI argument or enviroment variable to make it work properly. Since you might have a different path for every locale you specify, the string `{filepath}` inside the environment variable will be replaced by the specified locale.

## Adding your own function

If you have a new function to implement …

1. Decide upon a function NAME
2. Create the file src/ops/NAME.rs (with underscores instead of hyphens in the basename)
4. Add the function to src/ops/mod.rs
5. The file must implement the [Op trait](src/ops/traits.rs)

## Compatibility guarantees

We follow semver principles:

* Breaking the API requires a major version update. Changing the behavior of functions or extending non-exhaustive API elements requires a minor version update. Security bugfixes or severe issues (if they can be fixed in a backwards-compatible manner) are fixed with a patch release.
* The op names are fixed since the 1.0 release. The ops will never disappear. The ops will always implement what they describe. Requiring a different number of arguments or changing the arguments requires a major version update.
* The ordering of the operations when no `--op` is specified (more specifically, the internal priority) only requires a patch release
* The software license does not change.

## Release management

What to pay attention to before creating a new release:

1. Update [UnicodeData](https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt)
2. Update [NamesList](https://www.unicode.org/Public/UCD/latest/ucd/NamesList.txt)
3. Update [SpecialCasing](https://www.unicode.org/Public/UCD/latest/ucd/SpecialCasing.txt) (TODO not yet in use)
4. [Regenerate CLDR data](https://github.com/unicode-org/icu4x/blob/main/tutorials/data_management.md) with `icu4x-datagen -W -o data/icu4x_en-US.blob2 --include-collations search-all --trie-type small --locales en-us --keys all --format blob`
5. Review which crate versions to update
6. Unicode "scalar"/"char"/"codepoint"? codepoint! Plural/singular? depends on the meaning. One? singular! Many? plural! Unknown? plural!
7. verify whether you plan a major/minor/patch release
8. verify that the Op rust type matches its reported name string (TODO build automated tool for this?)
9. update the version number in README.adoc and main.rs

## Note: approach for Unicode/ASCII

We have one generic op name. If the user specifies a locale, we need to supply a correct Unicode-compatible result (maybe require a proper `OPSTR_LOCALE_DATAFILE`). If the user specifies no locale, we need to provide a best-effort Unicode-less alternative.

We can also expose the Unicode-less algorithm as additional operation (e.g. `sort` versus `sort-lexicographically`), because a suffix like `lexicographically` indicates that the sorting algorithm does not need/consider Unicode.

## Note: Strings versus bytes in terminals

Currently I only accept UTF-8 strings as arguments. The architecture allows strings as well as bytes as arguments. No op supports bytes though. As long as I cannot see a clear path how to support bytes supplied to rust through the CLI, I won't pursue that path (NOTE: rust abstracts CLI argument types away because Windows supplies UTF-16 and POSIX supplies bytes).

## Source Code

The source code is available at [Github](https://github.com/typho/opstr).

## License

See [the LICENSE file](LICENSE) (Hint: MIT license).

## Changelog

**0.7.0:** first public release <br/>
**0.9.0:** final evaluation release <br/>
**1.0.0:** uses Unicode Version 15.0, release with backwards compatibility guarantees

## Issues

Please report any issues on the [Github issues page](https://github.com/typho/opstr/issues).
