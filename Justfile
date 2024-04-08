default:
    @just --list --unsorted | grep -v '  default'

build:
    @just web/build
    @just server/build
