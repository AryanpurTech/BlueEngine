# Changelog

All notable changes to this project will be documented in this file.

## [0.5.20] - 2024-09-10

### Features

- Moved unsigned int types into feature flags ([fd5f495](https://github.com/AryanpurTech/BlueEngine/commit/fd5f495f6bb302a4e4826e2fb7b9a4370644b080))
- Added 2 more signals ([8a72062](https://github.com/AryanpurTech/BlueEngine/commit/8a72062f432735c346b91beb837012469d5ea81c))

## [0.5.19] - 2024-09-08

### Bug Fixes

- Default data and objects rebuilt upon window creation for correct texture formats ([540931d](https://github.com/AryanpurTech/BlueEngine/commit/540931d5f851823b12f26f7d72d5bcc3cdc53086))
- Events signal not firing ([31b53f9](https://github.com/AryanpurTech/BlueEngine/commit/31b53f9b10bca6cf7bc04f5589f0798155b91d1d))

### Documentation

- Added signals example! ([0e3aa61](https://github.com/AryanpurTech/BlueEngine/commit/0e3aa613b66e34c66b310ad509d6a459095adc82))

### Features

- Increased indices from u16 to u32 ([11d6aa9](https://github.com/AryanpurTech/BlueEngine/commit/11d6aa9299eca5350d6054805ac69d1c71d9e7bc))

## [0.5.16] - 2024-08-12

### Features

- Added window functions that goes to effect during update_loop initialization ([4061da7](https://github.com/AryanpurTech/BlueEngine/commit/4061da79430c058cc58562e717de804248ca0e6b))
- More sane setter functions ([1913327](https://github.com/AryanpurTech/BlueEngine/commit/191332702b9c5cd52ccdc261acb7c9ce47dd8dda))

### Miscellaneous Tasks

- Fix: #68 - updated dependencies to latest version ([f13bcf8](https://github.com/AryanpurTech/BlueEngine/commit/f13bcf8f7ed69cb057dbb4efa36629d6524de8a1))

## [0.5.10] - 2024-06-06

### Bug Fixes

- #60 ([8da08c6](https://github.com/AryanpurTech/BlueEngine/commit/8da08c6de944f512f4c80dbcb7f5ae2dee3136da))
- #49 ([67c47cd](https://github.com/AryanpurTech/BlueEngine/commit/67c47cd28393c41a2264a66fd33075421ddaf266))

### Miscellaneous Tasks

- New version ([77d2054](https://github.com/AryanpurTech/BlueEngine/commit/77d2054908a0b12afd6030961937c678c779009c))

## [0.5.7] - 2024-02-17

### Bug Fixes

- Surface error for non zero size on windows ([8426db3](https://github.com/AryanpurTech/BlueEngine/commit/8426db3e46bd709f0df98cf890ffdd73c87ecaef))

### Features

- Added `control_flow`, `present_mode`, `alpha_mode`, and `desired_maximum_frame_latency` options ([60513a5](https://github.com/AryanpurTech/BlueEngine/commit/60513a547b30284cc2bf0e977d462c69f9a8fb36))
- Fixed scissor bounds bug, added examples ([9a89185](https://github.com/AryanpurTech/BlueEngine/commit/9a89185451f55be11d2821c8c33d8eb1650aee88))
- Added scissor and clear color finally ([ee77156](https://github.com/AryanpurTech/BlueEngine/commit/ee771568340f74374023212e20c6845c5c14b253))

### Miscellaneous Tasks

- Clear color example and updates ([6e2f434](https://github.com/AryanpurTech/BlueEngine/commit/6e2f4343e501ce860f4154ddca38f2ad01e11076))

## [0.5.0] - 2023-09-14

### Documentation

- Added documentation to the entire engine ([5a86e7e](https://github.com/AryanpurTech/BlueEngine/commit/5a86e7ea71a4465e1c5d1e9dcdb10c2d0937d020))

## [0.4.32] - 2023-09-13

### Bug Fixes

- Defined wgpu transform matrix [see](https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/#a-perspective-camera) ([5204406](https://github.com/AryanpurTech/BlueEngine/commit/520440645985ff0dd313d108d411634d0aeed3fe))
- Transparency, and fix #43 ([0d4037d](https://github.com/AryanpurTech/BlueEngine/commit/0d4037dec55495c1eed55c6fb36fd470fb47bd98))

### Features

- Added `render_order` to the objects to control when they are sent to the gpu ([e00910a](https://github.com/AryanpurTech/BlueEngine/commit/e00910a2b91149895b00acb79d5d9fe909b67efb))
- Implemented switching between `perspective` and `orthographic` projection ([2cd24c7](https://github.com/AryanpurTech/BlueEngine/commit/2cd24c7f7a45d6064494b8621d2150a1a2f8091e))
- Option to set perspective or orthographic projection ([297f67e](https://github.com/AryanpurTech/BlueEngine/commit/297f67e87f7cfabb8be1f88ee87d8af9c17d4602))
- Added projection enum ([5177e4a](https://github.com/AryanpurTech/BlueEngine/commit/5177e4ac16f3a9b38068dffc5aef21813f11cdc9))

### Miscellaneous Tasks

- Added some fixes to the PR ([e1b9217](https://github.com/AryanpurTech/BlueEngine/commit/e1b9217791797f609c6deb794632f08d6b468a8c))
- Added some fixes to the PR ([2559d6b](https://github.com/AryanpurTech/BlueEngine/commit/2559d6b301f6268304bebaa7e5d55d9019b6edfd))

## [0.4.30] - 2023-09-02

### Features

- Instancing now works, with example. fix #40 ([8e5e2db](https://github.com/AryanpurTech/BlueEngine/commit/8e5e2db84775e91e8ccf919c82e8f5f40312885b))
- Transparency in textures now working ([1dafadf](https://github.com/AryanpurTech/BlueEngine/commit/1dafadfcaea8ae0acf2a8d8ac80e54c3f4c6dfed))

## [0.4.29] - 2023-09-02

### Features

- Resource sharing now fully working. with example ([be915ed](https://github.com/AryanpurTech/BlueEngine/commit/be915edf0e4f920b5b0f8578d71b33e4635ac8ed))
- PipelineData added to allow sharing pipeline resources ([e4c58d7](https://github.com/AryanpurTech/BlueEngine/commit/e4c58d792b22a3dbd419648437c30664929c4ab6))
- Added inline impl for ObjectStorage functions for cheaper trait usage ([b0d48bb](https://github.com/AryanpurTech/BlueEngine/commit/b0d48bb8fc47b83dac261352d34c5f1be88024d4))

### Miscellaneous Tasks

- Added some doc to lib.rs ([142a683](https://github.com/AryanpurTech/BlueEngine/commit/142a6832e71fe43b48dbfa18931f17ce82da2ce2))

## [0.4.28] - 2023-07-24

### Bug Fixes

- Potential fix for #27

### Features

- _and_return functions now return buffers and bindgroups - Objects can now be hidden from rendering
- Added backends option to the window descriptor
- Added justfiles instead of clog

### Miscellaneous Tasks

- Updated all dependency versions to latest
- Added CHANGELOG and workflow for it
- Deleted changelog workflow as not working
- Added a docs and changes to the downstream
- Upgraded versions of dependency and engine

### Refactor

- Moved light and model loader and added clog for changelog
- Added some docs, and cleaned up code a bit

## [0.4.26] - 2023-04-09

### Features

- _and_return functions now return buffers and bindgroups - Objects can now be hidden from rendering

### Miscellaneous Tasks

- Updated all dependency versions to latest

### Refactor

- Moved light and model loader and added clog for changelog
- Added some docs, and cleaned up code a bit

<!-- generated by git-cliff -->
