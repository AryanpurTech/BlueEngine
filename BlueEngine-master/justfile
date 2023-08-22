# This file uses Just https://github.com/casey/just | This file also assumes you use a shell


release_tag := "1.0.0"

# use this syntax: just release_tag="0.0.0" update-changelog
update-changelog:
    git cliff --unreleased --tag {{release_tag}} --prepend CHANGELOG.md