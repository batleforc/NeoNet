# NeoNet


## Contributing

Has of now, the best choice for contributing and having a ready to dev env is to use a "eclipse che" like env based on the image that i provide.

BUT if you need to make your own here what's needed:

- Rust (latest)
- [Cargo-bump](https://crates.io/crates/cargo-bump)
- [Cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)
- [Cocogitto](https://github.com/cocogitto/cocogitto)
- [GitLeaks](https://github.com/gitleaks/gitleaks)

Please do `cog install-hook --all` before your first commit, the hooks include a pre commit that will check for any secret and possible clippy error.

## CICD ?

This repo has two CICD:

- Build and Release that will create a release draft on each Tag
- Clippy that will check for possible improvement

### Release

NO TAG SHOULD BE MANUALY MADE !!

To make a tag use :

```shell
cog bump [ --patch | --minor | --major ]
```

Doing it with the cli will:

- Increment the past version depending on the choice made (path/minor/major)
- Change the version in Cargo.toml
- Generate the changelog
- Trigger the pipeline that will create a draft with possible package

#### If you want to undraft the release

- Go to the [Github release page](https://github.com/NeoNet-app/NeoNet/releases)
- Select the release to undraft
- Give it a name like `0.2.2 - Dalek` and a descrption to your need
- Publish the release and enjoy !!