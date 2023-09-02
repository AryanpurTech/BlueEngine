# This file uses Just https://github.com/casey/just | This file also assumes you use a shell

range := "HEAD..HEAD"
release_tag := "1.0.0"

# use this syntax: just release_tag="0.0.0" range="start..end" update-changelog
update-changelog:
    git cliff {{range}} --unreleased --tag {{release_tag}} --prepend CHANGELOG.md