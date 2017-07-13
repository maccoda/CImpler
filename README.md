# CImpler (or we could call it CInder)
A quick and dirty CI tool built in Rust.

Then check the irony that this has got a build badge from
a different CI!

[Documentation](https://maccoda.gitlab.io/cimpler/doc/cimpler/)

## YAML Build File
### File Name
CImpler will look for `.cimpler.yml`

### Sections
#### Before Build
This is where will set up all packages and download all
dependencies.

#### Build
This is where we need you to tell us how to build the
project.

#### After Build
This is where any of the after build sections need to be.
Whether you want to tell the world it works or send
someone an email to fix the sh*t they broke.


Goals:
- [ ] Parse how to build from YAML file (how original)
- [ ] Sync up to git repository for build (webhook perhaps)
- [ ] Be able to create new build when new branch is made
- [ ] Do some stuff with pull requests to test the merge
- [ ] Be able to create new workspaces for each build
- [ ] Have a web interface to show the shell of the build being executed
- [ ] Look into using Docker for containers
- [ ] Consider its use with dependent other builds. That is if a library
  component is also built on the server look into how it could be that it can
  trigger downstream builds and even save space by using the artifacts. (This
  will mean the build file needs to specify the artifacts)
- [ ] Keep track of previous build history (database)
