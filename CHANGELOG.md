# Changelog

All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

## [0.10.0](https://github.com/AryanpurTech/BlueEngine/compare/v0.9.4..0.10.0) - 2025-12-07

### Features

- updated egui to the latest version - ([6b070cb](https://github.com/AryanpurTech/BlueEngine/commit/6b070cba689dd0759f9d506e2c7630bbe5b223ad))
- updated utilities to latest version - ([b004018](https://github.com/AryanpurTech/BlueEngine/commit/b004018c90b81a45455e62028c62ba9b8fe75400))

### Refactoring

- removed flume as a dependency of headless - ([1f90fb9](https://github.com/AryanpurTech/BlueEngine/commit/1f90fb93b6650c98c5bae3631b0975c13e7d3909))

## [0.9.4](https://github.com/AryanpurTech/BlueEngine/compare/v0.9.3..0.9.4) - 2025-08-06

### Bug Fixes

- translate moving in the wrong direction - ([4a38a68](https://github.com/AryanpurTech/BlueEngine/commit/4a38a68c3209902ae25431dac8696a49a5df4674))

## [0.9.3](https://github.com/AryanpurTech/BlueEngine/compare/v0.9.2..0.9.3) - 2025-08-05

### Bug Fixes

- set_position and simple_input. Also added raw_input for finer usage - ([d60f522](https://github.com/AryanpurTech/BlueEngine/commit/d60f522a0c042710ff329442cfc95639dac0fe0d))

### Features

- winit input now working - ([f0fd32d](https://github.com/AryanpurTech/BlueEngine/commit/f0fd32d0d6d317a5ad0098e91dbab21040338506))

## [0.9.2](https://github.com/AryanpurTech/BlueEngine/compare/v0.9.1..0.9.2) - 2025-07-27

### Bug Fixes

- #112 the texture colors are now corrected - ([8e7c48b](https://github.com/AryanpurTech/BlueEngine/commit/8e7c48b23da389277965d6f9f6bd10d0cd6b01c7))

### Miscellaneous Chores

- removed rayon - ([5653acf](https://github.com/AryanpurTech/BlueEngine/commit/5653acfce9582fd695797f3e385e5d80236c3edb))

### Refactoring

- updated to a newer wgpu version - ([3b425e9](https://github.com/AryanpurTech/BlueEngine/commit/3b425e9364640565ad86697c8087d4a088572108))

## [0.9.1](https://github.com/AryanpurTech/BlueEngine/compare/v0.9.0..0.9.1) - 2025-05-05

### Bug Fixes

- [**breaking**] #108 - exchanged StringBuffer with AsRef<str> and String with Arc<str> - ([a148472](https://github.com/AryanpurTech/BlueEngine/commit/a148472d83f8902f700bb938dd5f89df2f532b97))
- #110 - Features for a backend is now empty by default for maximum compatibility - ([0e571b8](https://github.com/AryanpurTech/BlueEngine/commit/0e571b8131728f859c9134f7c36544f7659742c6))

### Features

- readded the embedded renderer and renamed `egui` plugin to `egui_plugin` for reduction of confusion - ([0d15919](https://github.com/AryanpurTech/BlueEngine/commit/0d15919c8de94e113c866718b9b9d8cbf3ec821b))

### Miscellaneous Chores

- updated dependencies - ([aa6076f](https://github.com/AryanpurTech/BlueEngine/commit/aa6076fe2ef605a06868a1dc93a1002b582e0cdb))
- updated dependencies - ([59338e8](https://github.com/AryanpurTech/BlueEngine/commit/59338e8427f3957fdc17d15b6d6647827e03aa4c))
- reverted wgpu update - ([73995d2](https://github.com/AryanpurTech/BlueEngine/commit/73995d2c57cfc7cbac1f3e617b61db518ca2e5d1))
- updated utilities to the latest version - ([547c402](https://github.com/AryanpurTech/BlueEngine/commit/547c402a9694312b68ebf9085f32f18a4fb68af4))
- some updates done to the embedded renderer - ([687d50e](https://github.com/AryanpurTech/BlueEngine/commit/687d50ee12af4267025a89130e04706bc71b482e))

## [0.9.0](https://github.com/AryanpurTech/BlueEngine/compare/v0.8.0..0.9.0) - 2025-04-13

### Bug Fixes

- #107 - ([daa2233](https://github.com/AryanpurTech/BlueEngine/commit/daa223316b000586bba898f68b0e9a8332bdd24c))
- camera issues with the headless mode - ([dcbe5a8](https://github.com/AryanpurTech/BlueEngine/commit/dcbe5a85f6b277db1fdb125a62840862662530dc))

### Features

- **(api)** updated the signals and runtime api - ([e8b6baa](https://github.com/AryanpurTech/BlueEngine/commit/e8b6baae45ca21f2df898e70fbbe4a62ec335036))
- **(api)** [**breaking**] updated the signals and runtime api - ([e8b6baa](https://github.com/AryanpurTech/BlueEngine/commit/e8b6baae45ca21f2df898e70fbbe4a62ec335036))
- **(api)** [**breaking**] updated the signals and runtime api - ([e8b6baa](https://github.com/AryanpurTech/BlueEngine/commit/e8b6baae45ca21f2df898e70fbbe4a62ec335036))
- initial version for headless mode - ([834bedd](https://github.com/AryanpurTech/BlueEngine/commit/834bedd7d5d3ca115f82d9e92bd83780214775fe))
- winit and windowing is now a dependency instead of hard coded within the engine. - ([daa2233](https://github.com/AryanpurTech/BlueEngine/commit/daa223316b000586bba898f68b0e9a8332bdd24c))
- headless feature is now fully available with it's own runtime - ([088e9fe](https://github.com/AryanpurTech/BlueEngine/commit/088e9fe8653d01ca9755bba7259b4c906fc1d8a0))
- [**breaking**] headless feature is now fully available with it's own runtime - ([088e9fe](https://github.com/AryanpurTech/BlueEngine/commit/088e9fe8653d01ca9755bba7259b4c906fc1d8a0))
- added a headless mode example - ([f39ad24](https://github.com/AryanpurTech/BlueEngine/commit/f39ad24c80fa4c8d2106d3e8fc2d859504613747))

### Miscellaneous Chores

- updated utilities version - ([10cd06f](https://github.com/AryanpurTech/BlueEngine/commit/10cd06f9891e4c8219d9346fad3c7964758959b4))
- [**breaking**] `WindowDescriptor` is now called `EngineSettings` - ([8a93cd8](https://github.com/AryanpurTech/BlueEngine/commit/8a93cd848b3ac0b06c929826674f05fdbc120815))
- updated lock dependencies - ([2fb1a23](https://github.com/AryanpurTech/BlueEngine/commit/2fb1a23172b281ae924768a547d06487466e02c3))
- [**breaking**] renamed `engine.input_events` into `engine.simple_input` to convey the meaning clearly - ([9f5beb5](https://github.com/AryanpurTech/BlueEngine/commit/9f5beb5c82d3caa73f3dd0136416b30ce114a1a1))
- moved the engine definitions into its own file - ([5511e6a](https://github.com/AryanpurTech/BlueEngine/commit/5511e6af7e1f5038551f0c340f84abb6093b5bef))
- added ObjectSettings parameter to the 3D primitve shapes - ([9a0ad36](https://github.com/AryanpurTech/BlueEngine/commit/9a0ad36923f7b8eff58e01f7cfe2bc4284a42d3e))
- updated versions - ([feb169f](https://github.com/AryanpurTech/BlueEngine/commit/feb169f7968de9b578b8a41ea156fbbc72029e3c))

## [0.8.0] - 2025-04-05

### Bug Fixes

- Object transformations ([e433121](https://github.com/AryanpurTech/BlueEngine/commit/e433121c79efb86c32273e549b3a6cce8e7329e3))
- Transformations and function updates in the Object ([88e2f86](https://github.com/AryanpurTech/BlueEngine/commit/88e2f86807031eda92c637504d64c262162a204d))

### Features

- Added README to the inner crates ([15f20cf](https://github.com/AryanpurTech/BlueEngine/commit/15f20cf03f82bcc7fcc718c94ab4ba0d4313f27c))
- Speedup with object update and return ([43bcc79](https://github.com/AryanpurTech/BlueEngine/commit/43bcc796a518507abfbbe5eed5570a8d47f66c18))

### Miscellaneous Tasks

- Updated the README ([d762d76](https://github.com/AryanpurTech/BlueEngine/commit/d762d76e93924af7965d80b6cdd916b9a3e647df))
- Updated versions in the crates ([cc710d2](https://github.com/AryanpurTech/BlueEngine/commit/cc710d23966b995e42b7189eb3867a1b6ef11cf2))
- Split the object code into readable chunks ([34c03b8](https://github.com/AryanpurTech/BlueEngine/commit/34c03b8666e8c08cc66442ccb3bad525f3b5a4dc))
- Updated the utilities crate ([40157cc](https://github.com/AryanpurTech/BlueEngine/commit/40157cc550ef7a5c44d2dccadbe7da15c4cbc244))

## [0.7.1] - 2025-04-01

### Bug Fixes

- The camera target being inaccurately set ([5062995](https://github.com/AryanpurTech/BlueEngine/commit/5062995f72e7479a7055131826ab4427647936fc))
- Camera aspect ratio fix ([09302c7](https://github.com/AryanpurTech/BlueEngine/commit/09302c7a58be3ccb3ea7d9784a62af51fcecfd69))

### Miscellaneous Tasks

- Updated version numbers ([d49b531](https://github.com/AryanpurTech/BlueEngine/commit/d49b5312c6fa21eadef4d3acef8bbbc1e5753525))

## [0.7.0] - 2025-03-31

### Bug Fixes

- #92 and some cleanups ([0c2457f](https://github.com/AryanpurTech/BlueEngine/commit/0c2457f0c62a360b486bca230b7ea163ebfe1216))

### Features

- Starting to autogenerate the Vector types ([378e51d](https://github.com/AryanpurTech/BlueEngine/commit/378e51d1081b43abd1c32e1e1a5ed4a0e54dee85))

### Miscellaneous Tasks

- Included the utilities in the main repository ([42b93a9](https://github.com/AryanpurTech/BlueEngine/commit/42b93a95694752c81308fd4815a77199866ec279))
- Finalized some documentation and cleanups ([7e35283](https://github.com/AryanpurTech/BlueEngine/commit/7e352837d2fc5ec79479be8234a01cbd89431521))
- Moved from nalgebra to glam, and moved native Vector types to glam Vector ([cb43a08](https://github.com/AryanpurTech/BlueEngine/commit/cb43a08cab00cfa13fb70b4b5692a5cdbaef2a52))
- Removed dev example ([5e11dca](https://github.com/AryanpurTech/BlueEngine/commit/5e11dcab46954df13ac3761e428316f565ba7f8a))
- Moved structures from the header to their respective components ([0c2c941](https://github.com/AryanpurTech/BlueEngine/commit/0c2c941af2a436e215ad7eab9f122a731a9d506e))
- Moved prelude, utils and primitive shapes to a cleaner format ([3f95a40](https://github.com/AryanpurTech/BlueEngine/commit/3f95a40f631782da72fa93533181091332fd5112))
- Renamed header to prelude ([2e58c8d](https://github.com/AryanpurTech/BlueEngine/commit/2e58c8df62e4a89fc46e6936628579760f5fcab8))

## [0.6.5] - 2025-03-15

### Miscellaneous Tasks

- Updated versions of all parts ([9a9a2b0](https://github.com/AryanpurTech/BlueEngine/commit/9a9a2b09e8dcd609b76ac84ff6f7fca203b04d60))
- Updated wgpu to the latest version ([5f45266](https://github.com/AryanpurTech/BlueEngine/commit/5f4526608e145e7a09750403e76a219652506372))
- Cleaned up github actions ([fbf01a9](https://github.com/AryanpurTech/BlueEngine/commit/fbf01a9bb0bd85090f5e13e7c20a0f6f54c2efa7))

## [0.6.3] - 2025-03-15

### Bug Fixes

- New edition ref issues ([57a0238](https://github.com/AryanpurTech/BlueEngine/commit/57a023854f3ebaa93c157d81ed6fd371e2c9c418))
- Docs and upgraded to 2024 edition ([b561ccc](https://github.com/AryanpurTech/BlueEngine/commit/b561ccc790d90218a238cac52523b2a6dc58b469))

### Miscellaneous Tasks

- Updated versions ([7265c25](https://github.com/AryanpurTech/BlueEngine/commit/7265c257358b9d23891fde2ad38fd6cec2adb341))

## [0.6.1] - 2025-01-11

### Features

- Adding Vector3 for all the [f32; 3] ([b84b193](https://github.com/AryanpurTech/BlueEngine/commit/b84b1934c426fb63348aec368b4e53003945ad3a))
- Finalized vector.rs ([da69d1e](https://github.com/AryanpurTech/BlueEngine/commit/da69d1e2aee9abe251a084d3d22e8b7dc52f245a))
- Added Indexing & Neg to Vector3 & Vector2 ([c648c14](https://github.com/AryanpurTech/BlueEngine/commit/c648c14637fb10babe6bf89d7409b9e22dfcac22))
- Added Position3D for all of the position: [f32; 3] ([eff4e4e](https://github.com/AryanpurTech/BlueEngine/commit/eff4e4e6a31f7989034d34adb63972f924c7dfd6))

### Miscellaneous Tasks

- Remove Cargo.lock ([820c960](https://github.com/AryanpurTech/BlueEngine/commit/820c960b01ec74ed1329bd2e79bf811e4f930065))
- Changelog from the release workflow ([ec69cb0](https://github.com/AryanpurTech/BlueEngine/commit/ec69cb0f208a1c9307cd0f1ae82438c913197099))

### Refactor

- Replaced every [f32; 2] with Vector2 ([4dbf03c](https://github.com/AryanpurTech/BlueEngine/commit/4dbf03cb112605ced62afbc167fd4edcdbab6ab6))
- Position3D to Vector3 ([6081dc2](https://github.com/AryanpurTech/BlueEngine/commit/6081dc2783ddf96e5d91adc7201b7120c1f7045b))

## [0.6.0] - 2024-12-30

### Features

- Added dynamic linking directly to the engine ([2f4fdea](https://github.com/AryanpurTech/BlueEngine/commit/2f4fdeac92688e9226bc6e4e4fbf900067c87fe8))
- Added custom errors to the engine, and removed the last expects and unwraps. The engine now should be fault tolerant ([489a872](https://github.com/AryanpurTech/BlueEngine/commit/489a872b29d740c0412be4d7a711b858c8f9427c))
- Massively removed unrequired Result(s) ([81efd06](https://github.com/AryanpurTech/BlueEngine/commit/81efd061446a1f68b0037446fa955ac5859e7f6e))

### Miscellaneous Tasks

- Added versions to the other packages as well ([873904c](https://github.com/AryanpurTech/BlueEngine/commit/873904cfe8b51337fe9361f48372691bb287036b))
- Changed to version instead of path ([b4dadc1](https://github.com/AryanpurTech/BlueEngine/commit/b4dadc13ba97eef43e635935f24385e559fb0514))
- Removed unwraps and results as much as possible ([cd7d3fb](https://github.com/AryanpurTech/BlueEngine/commit/cd7d3fb486d5a3e7ab80531664145b3d460b7499))
- Moved the engine into a core folder ([891a56d](https://github.com/AryanpurTech/BlueEngine/commit/891a56ddcc93197c23e4875e3ad61ee7dc6c59a7))

## [0.5.21] - 2024-11-09

### Features

- Added more methods for rotation of objects ([903a87a](https://github.com/AryanpurTech/BlueEngine/commit/903a87a56ab6942bd25c356a5718c76e374e3da3))
- Removed unnecessary comments ([2d356ca](https://github.com/AryanpurTech/BlueEngine/commit/2d356ca0c546d4fc886eac84e44557b9b8215255))

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

### Bug Fixes

- #62 and cleaned up bloat ([7421479](https://github.com/AryanpurTech/BlueEngine/commit/74214791584833e27b008ba56995a7aebbfe439e))
- #64, #59 ([8a6ac06](https://github.com/AryanpurTech/BlueEngine/commit/8a6ac06d39b77adacb77b8ee329cb671694bfd83))
- #59 #64 and clippy improvements ([9557edf](https://github.com/AryanpurTech/BlueEngine/commit/9557edf396f2830f41a8a47e95a431b8277b0671))
- Potential fix for view_texture error on older hardware ([4762fbd](https://github.com/AryanpurTech/BlueEngine/commit/4762fbd76d80ab372aaf4f8241b39e5575c91da9))
- #60 ([8da08c6](https://github.com/AryanpurTech/BlueEngine/commit/8da08c6de944f512f4c80dbcb7f5ae2dee3136da))
- #49 ([67c47cd](https://github.com/AryanpurTech/BlueEngine/commit/67c47cd28393c41a2264a66fd33075421ddaf266))
- #61 by introducing limits ([62ad461](https://github.com/AryanpurTech/BlueEngine/commit/62ad4618df711c7295574e91b8bc9d6416514fd4))
- Surface error for non zero size on windows ([8426db3](https://github.com/AryanpurTech/BlueEngine/commit/8426db3e46bd709f0df98cf890ffdd73c87ecaef))
- Fixed examples and inputs from 0.5.1 changes ([cad944e](https://github.com/AryanpurTech/BlueEngine/commit/cad944edc557afcd5d1b5c71eb7cf509adfee18b))
- Defined wgpu transform matrix see: https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/#a-perspective-camera ([5204406](https://github.com/AryanpurTech/BlueEngine/commit/520440645985ff0dd313d108d411634d0aeed3fe))
- Transparency, and fix #43 ([0d4037d](https://github.com/AryanpurTech/BlueEngine/commit/0d4037dec55495c1eed55c6fb36fd470fb47bd98))
- Potential fix for #27 ([641e4d6](https://github.com/AryanpurTech/BlueEngine/commit/641e4d6b96a6bdc3e75fcb0ae2aa52a7e486d7b0))

### Documentation

- Added documentation to the entire engine ([5a86e7e](https://github.com/AryanpurTech/BlueEngine/commit/5a86e7ea71a4465e1c5d1e9dcdb10c2d0937d020))

### Features

- Added window functions that goes to effect during update_loop initialization ([4061da7](https://github.com/AryanpurTech/BlueEngine/commit/4061da79430c058cc58562e717de804248ca0e6b))
- More sane setter functions ([1913327](https://github.com/AryanpurTech/BlueEngine/commit/191332702b9c5cd52ccdc261acb7c9ce47dd8dda))
- Objects now return reference for chained setters ([6a083aa](https://github.com/AryanpurTech/BlueEngine/commit/6a083aa18ef4d4f0ef70d3aa529ba8ce3a554437))
- Camera is now a collection than a single entity, where the operations done on camera is done on "main" camera ([790dcf8](https://github.com/AryanpurTech/BlueEngine/commit/790dcf8e76412136b6f7362fe6a9d00251d6416d))
- Android builds working fine again ([d3af15a](https://github.com/AryanpurTech/BlueEngine/commit/d3af15a1723af982cbd7d045b683a753feec557b))
- Added docker file for mobile builds ([c4f1918](https://github.com/AryanpurTech/BlueEngine/commit/c4f19186899c5e9d6a286bb54cc097228fb2e25e))
- Renamed to Signal and will be used as internal plugin ([da1959c](https://github.com/AryanpurTech/BlueEngine/commit/da1959cdd1c1fe6917e588e55878fc6518d86058))
- Added `control_flow`, `present_mode`, `alpha_mode`, and `desired_maximum_frame_latency` options ([60513a5](https://github.com/AryanpurTech/BlueEngine/commit/60513a547b30284cc2bf0e977d462c69f9a8fb36))
- Fixed scissor bounds bug, added examples ([9a89185](https://github.com/AryanpurTech/BlueEngine/commit/9a89185451f55be11d2821c8c33d8eb1650aee88))
- Added scissor and clear color finally ([ee77156](https://github.com/AryanpurTech/BlueEngine/commit/ee771568340f74374023212e20c6845c5c14b253))
- Changed `TextureData::Path` from static str to String ([5e6bc45](https://github.com/AryanpurTech/BlueEngine/commit/5e6bc453970368cb6ca0156070dd63c3f54dad4c))
- Added render_order to the objects to control when they are sent to the gpu ([e00910a](https://github.com/AryanpurTech/BlueEngine/commit/e00910a2b91149895b00acb79d5d9fe909b67efb))
- Implemented switching between perspective and orthographic projection ([2cd24c7](https://github.com/AryanpurTech/BlueEngine/commit/2cd24c7f7a45d6064494b8621d2150a1a2f8091e))
- Option to set perspective or orthographic projection ([297f67e](https://github.com/AryanpurTech/BlueEngine/commit/297f67e87f7cfabb8be1f88ee87d8af9c17d4602))
- Added projection enum ([5177e4a](https://github.com/AryanpurTech/BlueEngine/commit/5177e4ac16f3a9b38068dffc5aef21813f11cdc9))
- Instancing now works, with example. fix #40 ([8e5e2db](https://github.com/AryanpurTech/BlueEngine/commit/8e5e2db84775e91e8ccf919c82e8f5f40312885b))
- Transparency in textures now working ([1dafadf](https://github.com/AryanpurTech/BlueEngine/commit/1dafadfcaea8ae0acf2a8d8ac80e54c3f4c6dfed))
- Resource sharing now fully working. with example ([be915ed](https://github.com/AryanpurTech/BlueEngine/commit/be915edf0e4f920b5b0f8578d71b33e4635ac8ed))
- PipelineData added to allow sharing pipeline resources ([e4c58d7](https://github.com/AryanpurTech/BlueEngine/commit/e4c58d792b22a3dbd419648437c30664929c4ab6))
- Added inline impl for ObjectStorage functions for cheaper trait usage ([b0d48bb](https://github.com/AryanpurTech/BlueEngine/commit/b0d48bb8fc47b83dac261352d34c5f1be88024d4))
- Added justfiles instead of clog ([050ed51](https://github.com/AryanpurTech/BlueEngine/commit/050ed51a8fa8e4d0c8a089d76061dec3f30d9ac3))
- Added backends option to the window descriptor ([9e86772](https://github.com/AryanpurTech/BlueEngine/commit/9e867729325b47e69d8583cac0539a21092f6620))
- \_and_return functions now return buffers and bindgroups - Objects can now be hidden from rendering ([e68fbd5](https://github.com/AryanpurTech/BlueEngine/commit/e68fbd5a3e214a3128e864c15a91b4cff253e027))

### Miscellaneous Tasks

- Fix: #68 - updated dependencies to latest version ([f13bcf8](https://github.com/AryanpurTech/BlueEngine/commit/f13bcf8f7ed69cb057dbb4efa36629d6524de8a1))
- New version and updated documentation ([b9481b9](https://github.com/AryanpurTech/BlueEngine/commit/b9481b92bf5a3d85b5140e7927544420b1196359))
- Updated changelog ([5f3f232](https://github.com/AryanpurTech/BlueEngine/commit/5f3f232e7cf74a7447a68f8a0ba49c3649e2c61d))
- New version ([77d2054](https://github.com/AryanpurTech/BlueEngine/commit/77d2054908a0b12afd6030961937c678c779009c))
- Fixed typos ([a2f5559](https://github.com/AryanpurTech/BlueEngine/commit/a2f5559c02e9a2a82823f55b624d3079a1824116))
- New version for Signals update ([ee2f86c](https://github.com/AryanpurTech/BlueEngine/commit/ee2f86c6567b8a18aa0209502489dc8847a98998))
- The signal methods are now optional ([5abe028](https://github.com/AryanpurTech/BlueEngine/commit/5abe028aed72c1a48ee2dd42b739717cf9afd4b1))
- Changed the naming of plugins to more appropriate: live_events ([87280c1](https://github.com/AryanpurTech/BlueEngine/commit/87280c114c53122a52c86c9d7df0f68dbbf3de8d))
- Clear color example and updates ([6e2f434](https://github.com/AryanpurTech/BlueEngine/commit/6e2f4343e501ce860f4154ddca38f2ad01e11076))
- Updated to wgpu 0.19 ([d8165f3](https://github.com/AryanpurTech/BlueEngine/commit/d8165f3f0d95a5e7f433901e57450918f2fec6ba))
- Updated to latest wgpu and winit version. ([4e213a7](https://github.com/AryanpurTech/BlueEngine/commit/4e213a73b3832b86b5f427f9f88185b3badca406))
- Moving to wgpu 0.18 ([9f10faf](https://github.com/AryanpurTech/BlueEngine/commit/9f10faf19b5d35089b51936602e0d8447f8614c8))
- Added some fixes to the PR ([e1b9217](https://github.com/AryanpurTech/BlueEngine/commit/e1b9217791797f609c6deb794632f08d6b468a8c))
- Added some fixes to the PR ([2559d6b](https://github.com/AryanpurTech/BlueEngine/commit/2559d6b301f6268304bebaa7e5d55d9019b6edfd))
- Updated changelog ([0eee907](https://github.com/AryanpurTech/BlueEngine/commit/0eee90719eb749ff547f1301f484f45f38025e1b))
- Added some doc to lib.rs ([142a683](https://github.com/AryanpurTech/BlueEngine/commit/142a6832e71fe43b48dbfa18931f17ce82da2ce2))
- Added appropriate function names to objects ([7924440](https://github.com/AryanpurTech/BlueEngine/commit/79244405b7161972c17998355fc10cd61e3685ab))
- Upgraded versions of dependency and engine ([df18704](https://github.com/AryanpurTech/BlueEngine/commit/df18704c1ed908666deb985bc7fefb95c33fbed9))
- Added a docs and changes to the downstream ([39632ab](https://github.com/AryanpurTech/BlueEngine/commit/39632ab005f39b4ea2bb0e2261a2f04ad2e02920))
- Deleted changelog workflow as not working ([c94967f](https://github.com/AryanpurTech/BlueEngine/commit/c94967f50a9bb3839c85239efbe54fb0061bcc38))
- Added CHANGELOG and workflow for it ([9d594ef](https://github.com/AryanpurTech/BlueEngine/commit/9d594ef720dc50ff4624caafe925bd0c9dd077e0))
- Updated all dependency versions to latest ([ae44f76](https://github.com/AryanpurTech/BlueEngine/commit/ae44f7626d2344939dd885ed30064cf287a97ba7))

### Refactor

- Added some docs, and cleaned up code a bit ([afdf33e](https://github.com/AryanpurTech/BlueEngine/commit/afdf33eba9d00244d886ec4b9e95fae8026e4c26))
- Moved light and model loader and added clog for changelog ([75a93d0](https://github.com/AryanpurTech/BlueEngine/commit/75a93d034ccf759b3cb443c1abb78f8ba4cda5df))

### Build

- Bump bumpalo from 3.9.1 to 3.14.0 ([1f73ed2](https://github.com/AryanpurTech/BlueEngine/commit/1f73ed2e7baafce73d8f26df74a21db3ff1b3e37))
- Bump rustix from 0.36.5 to 0.36.17 ([e93128f](https://github.com/AryanpurTech/BlueEngine/commit/e93128fc1c01196a8aaf2eb446d7a868f6589855))

<!-- generated by git-cliff -->
