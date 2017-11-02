[![Build Status](https://travis-ci.org/kivikakk/comrak.svg?branch=master)](https://travis-ci.org/kivikakk/comrak)
![Spec Status: 643/643](https://img.shields.io/badge/specs-643%2F643-brightgreen.svg)
[![crates.io version](https://img.shields.io/crates/v/comrak.svg)](https://crates.io/crates/comrak)
[![docs.rs](https://docs.rs/comrak/badge.svg)](https://docs.rs/comrak)

Rust port of [github's `cmark-gfm`](https://github.com/github/cmark).

* [Usage](#usage)
* [Extensions](#extensions)
* [Legal](#legal)

## Usage

A binary is included which does everything you typically want:

```
$ comrak --help
comrak 0.1.6
Yuki Izumi <yuki@kivikakk.ee>
CommonMark parser with GitHub Flavored Markdown extensions

USAGE:
    comrak [FLAGS] [OPTIONS] [--] [<FILE>]

FLAGS:
        --github-pre-lang    Use GitHub-style <pre lang> for code blocks
        --hardbreaks         Treat newlines as hard line breaks
    -h, --help               Prints help information
    -V, --version            Prints version information

OPTIONS:
    -e, --extension <EXTENSION>...    Specify an extension name to use [values: strikethrough, tagfilter, table, autolink, superscript]
    -t, --to <FORMAT>                 Specify output format [default: html]  [values: html, commonmark]
        --width <WIDTH>               Specify wrap width (0 = nowrap) [default: 0]

ARGS:
    <FILE>...    The CommonMark file to parse; or standard input if none passed
```

And there's a Rust interface.  You can use `comrak::markdown_to_html` directly:

``` rust
use snoomark::{cm_to_rtjson, ComrakOptions};
assert_eq!(cm_to_rtjson("Hello, **世界**!", &ComrakOptions::default()),
           "{\"document\":[{\"e\":\"par\",\"c\":[{\"e\":\"text\",\"t\":\"Hello, 世界!\",\"f\":[[1, 7, 6]]}]}]}");
```

Or you can parse the input into an AST yourself, manipulate it, and then use your desired
formatter:

``` rust
extern crate snoomark;
extern crate typed_arena;
use typed_arena::Arena;
use snoomark::{parse_document, format_html, ComrakOptions};
use snoomark::nodes::{AstNode, NodeValue};

// The returned nodes are created in the supplied Arena, and are bound by its lifetime.
let arena = Arena::new();

let root = parse_document(
    &arena,
    "This is my input.\n\n1. Also my input.\n2. Certainly my input.\n",
    &ComrakOptions::default());

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
    where F : Fn(&'a AstNode<'a>) {
    f(node);
    for c in node.children() {
        iter_nodes(c, f);
    }
}

iter_nodes(root, &|node| {
    match &mut node.data.borrow_mut().value {
        &mut NodeValue::Text(ref mut text) => {
            *text = text.replace("my", "your");
        }
        _ => (),
    }
});

let html: String = format_html(root, &ComrakOptions::default());

assert_eq!(
    html,
    "<p>This is your input.</p>\n\
     <ol>\n\
     <li>Also your input.</li>\n\
     <li>Certainly your input.</li>\n\
     </ol>\n");
```

## Testing 

Testing is done through the main.rs function and uses flags to enable or disable
functionality. We can turn on rtjson parsing by passing in the flag `--rtjson` 
so that the parser uses this. Alternativly we can disable this if we want to aswell.

Testing relies on the spec testing format, defined in main.

``` rust
cargo run -- --rtjson --spec specs/rtjson/rtjson.spec
```

## Extensions

Comrak supports the five extensions to CommonMark defined in the
[GitHub Flavored Markdown Spec](https://github.github.com/gfm/):

* [Tables](https://github.github.com/gfm/#tables-extension-)
* [Task list items](https://github.github.com/gfm/#task-list-items-extension-)
* [Strikethrough](https://github.github.com/gfm/#strikethrough-extension-)
* [Autolinks](https://github.github.com/gfm/#autolinks-extension-)
* [Disallowed Raw HTML](https://github.github.com/gfm/#disallowed-raw-html-extension-)

as well as superscript.

By default none are enabled; they are individually enabled with each parse by
setting the appropriate values in the
[`ComrakOptions` struct](https://docs.rs/comrak/newest/comrak/struct.ComrakOptions.html).

## Legal

Copyright (c) 2017, Yuki Izumi.  Licensed under the [2-Clause BSD License](https://opensource.org/licenses/BSD-2-Clause).

`cmark` itself is is copyright (c) 2014, John MacFarlane.

See [COPYING](COPYING) for all the details.

## Contributors

Thank you for PRs and issues opened!

* [ConnyOnny](https://github.com/ConnyOnny)
* [killercup](https://github.com/killercup)
* [bovarysme](https://github.com/bovarysme)

## Installing on Reddit production servers

Deploying changes to snoomark to Reddit is straightforward, but requires a precise chain of events.

1. [Make updates to snoomark](#make-updates-to-snoomark)
2. [Determine a new version number to cut](#determine-a-new-version-number-to-cut)
3. [Update Cargo with new version](#update-cargo-with-new-version)
4. [Commit changes and push](#commit-changes-and-push)
5. [Tag a new git version](#tag-a-new-git-version)
6. [Push git tag and retrieve shasum](#push-git-tag-and-retrieve-shasum)
7. [Update puppet manifest with new version and shasum](#update-puppet-manifest-with-new-version-and-shasum)
8. [Verify the update](#verify-the-update)

### Make updates to snoomark

Make whatever changes you want in production and save them. If you _don't_ need the changes to immediately go to production, then stop here and commit your work. Your changes will be reflected the next time the entire deploy-to-production process is completed.

### Determine a new version number to cut

Have a look at the list of existing git tags: https://github.com/reddit/snoomark/tags

You'll want to select an unused version tag. For fixups and minor updates, consider using a sequential _minor-minor_ version. Try to reserve _major_ and _minor_ versions for more significant releases such as feature releases.

### Update Cargo with new version

Under the `[package]` section of [`Cargo.toml`](https://github.com/reddit/snoomark/blob/master/Cargo.toml), update `version` to be the string representation of the newest version, like so:

```
[package]
...
version = "<version>"
...
```

### Commit changes and push

Once you'd committed both your changes as well as the updated version in `Cargo.toml` (can be in the same commit), commit and push your changes. Merge to master once approved.

### Tag a new git version

Issue a new git tag to your repository using the following command:

```
git tag <version>
```

### Push git tag and retrieve shasum

Enter the following to push your tag to GitHub:

```
git push <origin/upstream> <version>
```

This will push the new tag to GitHub. The tagged version will include all commits going back to the last time a new tag was pushed.

Once pushed, view the list of Drone builds for the snoomark repo: https://drone.reddit.ue1.snooguts.net/reddit/snoomark

Find the build corresponding to the last commit made before tagging a release. The message under the commit should read something like `<author> authored <date> to refs/tags/<version>`. Click on the build to view, then scroll towards the bottom of the log, where you should find some output matching the following:

```
SHA256 hash of the built libsnoomark release:
$ shasum -a 256 target/release/libsnoomark-<version>.so
<shasum_hash>  target/release/libsnoomark-<version>.so
```

In the above example `<shasum_hash>` is the shasum of the file. You'll need to keep record of this to complete the deploy.

### Update puppet manifest with new version and shasum

In [`reddit/puppet`](https://github.com/reddit/puppet), update [`common.yaml`](https://github.com/reddit/puppet/blob/master/hiera/common.yaml) with the version and shasum of the latest version:

```
# reddit_service
reddit_service::snoomark::version: "<version>"
reddit_service::snoomark::version_checksum: "<shasum_hash>"
```

Then, follow the deploy instructions for puppet to make your changes and update snoomark in production. Shortly after puppet has been rolled, the production version of snoomark should be updated.

### Verify the update

Open a `reddit-shell` instance on `tools-01`. Issue the following:

```
import snoomark
snoomark.__doc__
```

If your update has been successfully made, the response will read:

```
[snoomark <version>] This module is implemented in Rust.
```
