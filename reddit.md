# Reddit-specific snoomark notes

This repo is a fork of comrak called snoomark. It differs in some ways from
upstream.

## Testing

Testing is done through the main.rs function and uses flags to enable or disable
functionality. We can turn on rtjson parsing by passing in the flag `--rtjson` 
so that the parser uses this. Alternativly we can disable this if we want to aswell.

Testing relies on the spec testing format, defined in main.

``` rust
cargo run -- --rtjson --spec specs/rtjson/rtjson.spec
```

The file `script/test.sh` builds and runs the full test suite, and this is what
is run by the CI server. Running with `SPECS_ONLY=1 script/test.sh` will skip
the build and the unit tests and just run the spec tests.

Snoomark is primarily used through python bindings, and so it's often useful to
test through python bindings. The script `script/cm_to_rtjson.py` will run run a
document through the parser and print the resulting json. It can also run
benchmarks and generate flamegraphs. See the comments in that file for details.

`script/genspectest.py` will parse a document and generate output suitable for
copying into a spec file as a new test case.

`script/spec_tests.py` is a fork of the CommonMark spec testing script that
supports testing rtjson with `--rtjson` flag.

## Generating flame graphs

Generating flame graphs requires a nightly compiler and the "flamegraphs" feature.

Build with

    cargo +nightly build --release --features=flamegraphs

Then generate a flamegraph with

    SM_TARGET=release script/cm_to_rtjson.py [FILE] --flame

The flame graph will be in flamegraph.html

## Fuzzing

The `fuzz` directory contains a fuzz target for `cargo-fuzz`. It also requires a
nightly compiler. To fuzz first install cargo fuzz with `cargo install cargo-fuzz`,
then run

    cargo +nightly fuzz run snoofuzz

Probably only works on Linux.

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
