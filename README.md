# bezeC - Open Container Initiative Runtime Specification

bezeC [bežec - Slovak for "runner"] is an implementation of the [Open Container Initiative Runtime Specification](https://github.com/opencontainers/runtime-spec) written in [Rust](https://www.rust-lang.org/).

## Motivation

The main motivation for this project is to learn more about the Open Container Initiative Runtime Specification and the Rust programming language. It is not intended to be a production-ready container runtime. It is also not intended to be a replacement for existing container runtimes like [runc](https://github.com/opencontainers/runc) or [crun](https://github.com/containers/crun) or the others.

The goal is to implement linux only for now. Windows, Solaris, Z/OS, and macOS (and others) are not supported.

## Features

[ ] Read the OCI runtime specification
[ ] Implement the OCI runtime specification
[ ] Create a container from an OCI image
[ ] Run a container
[ ] Stop a container
[ ] Remove a container
[ ] List containers
[ ] Get container information
[ ] Execute a command in a running container
[ ] Copy files to/from a container

## Dependencies

- [Rust](https://www.rust-lang.org/)
- [taskfile](https://taskfile.dev/)
- [podman](https://podman.io/)

##
