# RCI
A quick and dirty CI tool built in Rust.

Then check the irony that this has got a build badge from
a different CI!

## YAML Build File
### File Name
RCI will look for `.rci-build.yml`

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
- [ ] Sync up to git repository for build
- [ ] Be able to create new build when new branch is made
- [ ] Do some stuff with pull requests to test the merge
- [ ] Be able to create new workspaces for each build