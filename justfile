# This file uses Just https://github.com/casey/just | This file also assumes you use a shell

update-changelog release_tag:
    git cliff --unreleased --tag {{release_tag}} --prepend CHANGELOG.md

publish release_tag:
    @cd crates/blue_engine_core && cargo publish
    @cd crates/blue_engine_dynamic && cargo publish
    @cargo publish --allow-dirty
    @just update-changelog {{release_tag}}
