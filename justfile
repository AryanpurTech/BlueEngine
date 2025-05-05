# This file uses Just https://github.com/casey/just | This file also assumes you use a shell

update-changelog release_tag:
    git cliff --unreleased --tag {{release_tag}} --prepend CHANGELOG.md

publish:
    @cd crates/blue_engine_core && cargo publish --allow-dirty
    @cd crates/blue_engine_dynamic && cargo publish --allow-dirty
    @cargo publish --allow-dirty
