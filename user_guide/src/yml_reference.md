# .cimple.yml Reference

- [.cimple.yml Reference](#cimple-yml-reference)
    - [Builds](#builds)
        - [`before_build`](#before_build)
        - [`build`](#build)
        - [`after_build`](#after_build)

## Builds

Of course the main reason you would want to use a CI is to perform some form of
build to check changes pass all tests, etc. These are what you will need to use
to configure your build.

### `before_build`

This is the set of commands that will be executed before the build to set up the environment.

```yml
before_build:
    - apt-get update -yqq
    - apt-get install -yqq pkg-config libssl-dev make
```

### `build`

The actual build/test step.

```yml
build:
    - rustc --version && cargo --version
    - cargo test --verbose
```

### `after_build`

This is the checks or actions that want to be performed after a build. Perhaps a
code coverage check or a deploy.

```yml
after_build:
    - cargo doc
```

