# Changelog

All notable changes to this project will be documented in this file.

## [1.3.1](https://github.com/orhun/git-cliff/compare/v1.3.0..v1.3.1) - 2023-09-30

### ⛰️  Features

- *(args)* Support tilde for options ([#266](https://github.com/orhun/git-cliff/issues/266)) - ([8698bc2](https://github.com/orhun/git-cliff/commit/8698bc2ce4d58fdeb9563c18f7430798b6359029))
- *(ci)* Distribute RPM package ([#159](https://github.com/orhun/git-cliff/issues/159)) - ([baf4da8](https://github.com/orhun/git-cliff/commit/baf4da80639682628ca4ae538b4555ff1b6262da))

### 🐛 Bug Fixes

- *(ci)* Update cargo-tarpaulin arguments - ([83a0371](https://github.com/orhun/git-cliff/commit/83a03711185df20f37bbcbad955e7783b8bdb662))

### 🚜 Refactor

- *(ci)* Simplify cargo-tarpaulin installation - ([95f8d53](https://github.com/orhun/git-cliff/commit/95f8d53ac158d81433f6a49cf0794d92b0eb21ef))

### 📚 Documentation

- *(installation)* Update instructions for Arch Linux - ([291a928](https://github.com/orhun/git-cliff/commit/291a9282888547b4c45d64ccb8f1495448544201))
- *(installation)* Add instructions for Alpine Linux - ([3199bba](https://github.com/orhun/git-cliff/commit/3199bba672e8045141debf1268873811f17ac405))
- *(license)* Re-license under the MIT + Apache 2.0 license ([#303](https://github.com/orhun/git-cliff/issues/303)) - ([cd56344](https://github.com/orhun/git-cliff/commit/cd563444dec852bacc2586dea55a7b0d5dcdc844))
- Update Tera links to the new URL ([#272](https://github.com/orhun/git-cliff/issues/272)) - ([890de00](https://github.com/orhun/git-cliff/commit/890de0007e108059378b134e9b9d0af2a6965027))

### ⚙️ Miscellaneous Tasks

- Remove GPL code ([#293](https://github.com/orhun/git-cliff/issues/293)) - ([e3606ba](https://github.com/orhun/git-cliff/commit/e3606babdf35022b662c870cb720c1f5339e1543))

### ◀️ Revert

- *(args)* Update clap and clap extras to v4 ([#137](https://github.com/orhun/git-cliff/issues/137)) ([#292](https://github.com/orhun/git-cliff/issues/292)) - ([fb4c733](https://github.com/orhun/git-cliff/commit/fb4c733a70a4dbc25060481ee5f3c644bb0bb83b))

## [1.3.0](https://github.com/orhun/git-cliff/compare/v1.2.0..v1.3.0) - 2023-08-31

### ⛰️  Features

- *(changelog)* [**breaking**] Add postprocessors ([#155](https://github.com/orhun/git-cliff/issues/155)) - ([5dc5fb7](https://github.com/orhun/git-cliff/commit/5dc5fb786db922322faacf928cc571a2d785cab2))

### 🐛 Bug Fixes

- *(cd)* Do not publish release notes for pre-releases ([#249](https://github.com/orhun/git-cliff/issues/249)) - ([7a82aa1](https://github.com/orhun/git-cliff/commit/7a82aa1a769b2170ea7563d7df3c59da5a134201))
- *(cd)* Disable win32-arm64 PyPI builds - ([baf34a8](https://github.com/orhun/git-cliff/commit/baf34a81f0b27be30f24f8899d44dfd1f3afaa13))
- *(deps)* Avoid problematic serde release - ([87f74bc](https://github.com/orhun/git-cliff/commit/87f74bc78feb94c390ad421849e9b356e71657ca))
- *(examples)* Add missing newline ([#253](https://github.com/orhun/git-cliff/issues/253)) - ([aad4222](https://github.com/orhun/git-cliff/commit/aad4222986ea9d65807f3dcb06446d19455e3865))
- *(informer)* Do not inform about pre-releases ([#249](https://github.com/orhun/git-cliff/issues/249)) - ([87e47e6](https://github.com/orhun/git-cliff/commit/87e47e68b6154a4da870342e0973123a974cb105))
- Fix previous release references - ([fbb605e](https://github.com/orhun/git-cliff/commit/fbb605e4f663d96ad140767bf816b5af8c97a92e))

### 🚜 Refactor

- *(clippy)* Apply clippy suggestions - ([4c09759](https://github.com/orhun/git-cliff/commit/4c097591584f15f61fa830feb0d738f051a009c3))
- *(clippy)* Apply clippy suggestions - ([a814bbe](https://github.com/orhun/git-cliff/commit/a814bbea34050301cec8140d6a12aa28ff62ae32))
- *(config)* Use a macro for generating commit list - ([c695ca3](https://github.com/orhun/git-cliff/commit/c695ca308e5aada03cbd65085497436ac5c341e9))
- *(docker)* Avoid copying volume inside container ([#142](https://github.com/orhun/git-cliff/issues/142)) - ([65d365c](https://github.com/orhun/git-cliff/commit/65d365c7b521e30ebc173d97f10c41da604582ae))
- *(lib)* Use implicit serde imports - ([c8cf855](https://github.com/orhun/git-cliff/commit/c8cf855939ce588824d7a4109ddf0f1c0828b6c6))

### 📚 Documentation

- *(blog)* Add blog post for 1.3.0 release ([#264](https://github.com/orhun/git-cliff/issues/264)) - ([93d4aa4](https://github.com/orhun/git-cliff/commit/93d4aa41e362d8f1c332930a8a400a2781146625))
- *(example)* Update the header of configuration examples - ([0bf5ebe](https://github.com/orhun/git-cliff/commit/0bf5ebe79b293d28a3ed168ef26e4d7c2477614a))
- *(example)* Add example for generating changelog similar to cocogitto - ([e1cf26e](https://github.com/orhun/git-cliff/commit/e1cf26e2a57266e786b395a76f0fc95a7b723cc3))
- *(readme)* Add contributors graph - ([3c9ced0](https://github.com/orhun/git-cliff/commit/3c9ced0b0526c5f5c63c160a03ae06076624eb6c))

### 🎨 Styling

- *(config)* Further beautify the changelog in this repo - ([fd7446c](https://github.com/orhun/git-cliff/commit/fd7446ce2f977da44620dc61a95a7e642f6fd000))
- *(config)* Apply formatting to configuration files - ([e9aa991](https://github.com/orhun/git-cliff/commit/e9aa9918b650cc88b53e4c96783d5161a94cee9a))
- Fix formatting - ([14725e5](https://github.com/orhun/git-cliff/commit/14725e547bb40cd4c3c152515b7c5994eed4a49e))

### 🧪 Testing

- *(changelog)* Add cases for `docs:` prefix ([#167](https://github.com/orhun/git-cliff/issues/167)) - ([3a717e2](https://github.com/orhun/git-cliff/commit/3a717e25aa8fb757dd7af80463938be426313139))
- *(repo)* Add an informative error message to failing test - ([7d7470b](https://github.com/orhun/git-cliff/commit/7d7470b2d0d030135aab09369d6484837c2bd6c7))
- Fix keep a changelog test case - ([0a6ed62](https://github.com/orhun/git-cliff/commit/0a6ed62f57929657c1c464c371c100217f7eeb50))
- Fix keep a changelog test case - ([f5d3365](https://github.com/orhun/git-cliff/commit/f5d336522682d31ba5f4486c6ef0880e8160de95))

### ⚙️ Miscellaneous Tasks

- *(cargo)* Update MSRV to 1.70.0 - ([1068912](https://github.com/orhun/git-cliff/commit/10689128222865515c32c9730fc5742beb7e6b03))
- *(cd)* Use the latest git-cliff-action for releases - ([3eb97b8](https://github.com/orhun/git-cliff/commit/3eb97b8426ec42e28ed1d623f8d8a15a7a64756e))
- *(cd)* Support creating prereleases on GitHub - ([a22426a](https://github.com/orhun/git-cliff/commit/a22426a37d6278f1d348751d00928a20057b578c))
- *(config)* Add additional parsers for default config - ([b745778](https://github.com/orhun/git-cliff/commit/b7457785e6b753ccd9c163b18ad557a0a621f87c))
- *(config)* Update configuration for alpha/beta/rc releases - ([7e7e5a7](https://github.com/orhun/git-cliff/commit/7e7e5a7e09acf6e91e45b25cb567a8f9bdd56bb5))
- *(config)* Skip dependency updates in the changelog - ([b2edc23](https://github.com/orhun/git-cliff/commit/b2edc231a6d999f3af737a2bff00309b499e13d7))
- *(core)* Make git2 optional - ([5fdf39a](https://github.com/orhun/git-cliff/commit/5fdf39ae32441ce70a942e577e31ba0cd2323137))
- *(docker)* Update versions in Dockerfile - ([0ccab91](https://github.com/orhun/git-cliff/commit/0ccab915868b2e594e7bef0b9e56eb290ca500dc))
- *(github)* Add config for welcome bot - ([a4725d4](https://github.com/orhun/git-cliff/commit/a4725d4ce43340cdb0483ac2bb7566c00dc7a991))
- *(mergify)* Rename mergify configuration file - ([9fd1be6](https://github.com/orhun/git-cliff/commit/9fd1be6dc6a5189911572caab99f13c72903af09))
- *(mergify)* Add configuration file for automatic merge ([#245](https://github.com/orhun/git-cliff/issues/245)) - ([0a79aae](https://github.com/orhun/git-cliff/commit/0a79aae3576aaa17385d17d114ef6d3f3aa1fb48))
- *(pypi)* Publish `git-cliff` on PyPI ([#158](https://github.com/orhun/git-cliff/issues/158)) - ([2b7a1ef](https://github.com/orhun/git-cliff/commit/2b7a1efaafca017c4a21f1af25c6047347119e8d))
- Check without default features - ([e323621](https://github.com/orhun/git-cliff/commit/e323621094141acaab360d6fa42990c7cdd4bac4))

## [1.2.0](https://github.com/orhun/git-cliff/compare/v1.1.2..v1.2.0) - 2023-04-28

### ⛰️  Features

- *(args)* Update clap and clap extras to v4 ([#137](https://github.com/orhun/git-cliff/issues/137)) - ([0e710c2](https://github.com/orhun/git-cliff/commit/0e710c256b1bbb7e0ab9e4cd5e16e01bac037d42))
- *(commit)* Make the fields of `Signature` public - ([104aac9](https://github.com/orhun/git-cliff/commit/104aac93b468071c107e95ba0d377a69993e3403))
- *(config)* Add a custom configuration file for the repository - ([0d4e689](https://github.com/orhun/git-cliff/commit/0d4e689115bdef19b3c44d110bde81820201518f))
- *(config)* Support placing configuration inside pyproject.toml ([#147](https://github.com/orhun/git-cliff/issues/147)) - ([fe5e5b8](https://github.com/orhun/git-cliff/commit/fe5e5b841a27b48c9b9d49483deaf72431c771c4))
- *(docker)* Generate SBOM/provenance for the Docker image - ([2ef259e](https://github.com/orhun/git-cliff/commit/2ef259e8cc79cfaabd5b51cccf85dde3c381e7c6))
- *(parser)* Support using regex group values ([#145](https://github.com/orhun/git-cliff/issues/145)) - ([7767ace](https://github.com/orhun/git-cliff/commit/7767ace8f8c7a38a27c9850438f7fcc752502c81))

### 🐛 Bug Fixes

- *(ci)* Use MUSL build of cargo-tarpaulin - ([98c9e00](https://github.com/orhun/git-cliff/commit/98c9e00aefcf625a27ee335927a24d36dbf25f75))
- *(ci)* Update cargo-tarpaulin installation command for CI - ([3019eff](https://github.com/orhun/git-cliff/commit/3019eff86929289b8f1d6b13705d95a9a3cc7952))
- *(config)* [**breaking**] Nested environment config overrides ([#157](https://github.com/orhun/git-cliff/issues/157)) - ([bb9a889](https://github.com/orhun/git-cliff/commit/bb9a889f1b06dd44f7183771644ce95520995e58))
- *(config)* Set max of `limit_commits` to the number of commits ([#140](https://github.com/orhun/git-cliff/issues/140)) - ([06f6732](https://github.com/orhun/git-cliff/commit/06f6732b77c19e69333fe53196e2d3b4ac5eb557))
- *(deploy)* Set the node cache dependency path - ([020b02d](https://github.com/orhun/git-cliff/commit/020b02d3e3fa2ea40057d001028f38335d402dbf))
- *(docker)* Remove target directory from .dockerignore - ([c0f97bd](https://github.com/orhun/git-cliff/commit/c0f97bd0b7a4ffbfd898dfefa614bf5a928d75f0))
- *(release)* Use the correct argument in release script - ([b3eef4a](https://github.com/orhun/git-cliff/commit/b3eef4a6889cdedac1c0760455849067ef2d887d))
- *(website)* Fix broken links - ([77dda53](https://github.com/orhun/git-cliff/commit/77dda538431b94b4577223ea381db9f756519e3d))

### 🚜 Refactor

- *(cd)* Remove unnecessary config update - ([e42f910](https://github.com/orhun/git-cliff/commit/e42f91013b8be1d0cba55fa638f702b9bfa85df2))
- *(ci)* Test the website deployment with a different job - ([2d72125](https://github.com/orhun/git-cliff/commit/2d721259ca747813c8f9502d84f41b69d5fc685c))
- *(lib)* [**breaking**] Move changelog module to git-cliff-core - ([2ab2c8f](https://github.com/orhun/git-cliff/commit/2ab2c8fb5e0c56b5ec51689ea33ebd4ec98a5310))
- *(test)* Handle errors for changelog module tests - ([ed389b7](https://github.com/orhun/git-cliff/commit/ed389b70e22f056549637573917a23adcb345733))
- *(website)* Update header location - ([72af8ac](https://github.com/orhun/git-cliff/commit/72af8ac129bce5cb1dee119d3e8743112506a939))

### 📚 Documentation

- *(blog)* Add blog post about what's new in 1.2.0 - ([e0a29ef](https://github.com/orhun/git-cliff/commit/e0a29efe3f4cc6a4162727f7322af2aaf73da619))
- *(blog)* Update the blog post style - ([74cf47e](https://github.com/orhun/git-cliff/commit/74cf47e380d3dae4622a54ad1f0b3e31a96c59c6))
- *(config)* Update the sections - ([c402452](https://github.com/orhun/git-cliff/commit/c4024523fac06f1a886979d2fe748078f7b3267d))
- *(config)* Add comments to the default configuration file - ([7e3adb0](https://github.com/orhun/git-cliff/commit/7e3adb0442384e76219dc5f92083f6cbd2a59968))
- *(contributing)* Mention the nightly requirement for rustfmt - ([a5a78fc](https://github.com/orhun/git-cliff/commit/a5a78fc4c5a44353b570327e7c21d51266d8e712))
- *(contributing)* Update MSRV - ([a824f48](https://github.com/orhun/git-cliff/commit/a824f48f12842ed730fd46413937212763cb38c9))
- *(examples)* Move examples to separate file - ([db43437](https://github.com/orhun/git-cliff/commit/db434371518005d43819fd361617d94ac9e613f7))
- *(github)* Update the pull request template about GitHub labels - ([54f735a](https://github.com/orhun/git-cliff/commit/54f735ab1ab0528447b4a3cd70f583d563b99d8c))
- *(github)* Update pull request template - ([c261fad](https://github.com/orhun/git-cliff/commit/c261fad137a5332cd1ab611970c845565a3b57fe))
- *(github)* Update issue templates - ([6a076ca](https://github.com/orhun/git-cliff/commit/6a076cabeb725a851530cda707355f6493a9fd68))
- *(github)* Update funding options - ([6e35834](https://github.com/orhun/git-cliff/commit/6e3583423698f0b9b18bef236e9f22f30272e848))
- *(github)* Add security policy - ([823d272](https://github.com/orhun/git-cliff/commit/823d272a9e54faae287cd9b782855026b1476567))
- *(readme)* Update README.md about documentation website - ([56240fd](https://github.com/orhun/git-cliff/commit/56240fd11f4d7142d1b734f1afdd419ba2a6e6d8))
- *(readme)* Add tj-actions/git-cliff to the list of actions ([#152](https://github.com/orhun/git-cliff/issues/152)) - ([2520dd2](https://github.com/orhun/git-cliff/commit/2520dd2fda2e88aa3dd1888e559260c9819c4844))
- *(readme)* Add discord badge - ([9fa5dd4](https://github.com/orhun/git-cliff/commit/9fa5dd44c1b65cd801db6d4a677e77ba12ddf9d8))
- *(readme)* Add release-plz to related projects ([#151](https://github.com/orhun/git-cliff/issues/151)) - ([f781e29](https://github.com/orhun/git-cliff/commit/f781e299ca4a631dbd0c686e640804a98d85fbf6))
- *(readme)* Fix typos in README.md - ([38943e8](https://github.com/orhun/git-cliff/commit/38943e8bac1d9535118121a4fea1821f85b93566))
- *(readme)* Remove unneeded word in README.md ([#141](https://github.com/orhun/git-cliff/issues/141)) - ([6f1459b](https://github.com/orhun/git-cliff/commit/6f1459bcc06f06b72e33bf450bd57958cbdf5830))
- *(readme)* Add link to the Console #141 interview about git-cliff - ([d057390](https://github.com/orhun/git-cliff/commit/d057390b14cb72db2a6b159790d1b4bdb0cb2b55))
- *(website)* Add Twitter link to banner - ([ee5ea85](https://github.com/orhun/git-cliff/commit/ee5ea855b85ac809b9b732165f71dd84471e4ec5))
- *(website)* Move documentation to the website ([#153](https://github.com/orhun/git-cliff/issues/153)) - ([56d3544](https://github.com/orhun/git-cliff/commit/56d3544454633927b889d8b921a465b967b4e002))

### 🎨 Styling

- *(docs)* Fix the grammar for tj-actions - ([53c0c1d](https://github.com/orhun/git-cliff/commit/53c0c1d0758fe20734d4b7ec792faa5413d4c533))
- *(docs)* Update the formatting for python integration example - ([3ee6724](https://github.com/orhun/git-cliff/commit/3ee672483790ec71c700907a6e93af4698492026))
- *(readme)* Update the style for project name - ([c41bbe0](https://github.com/orhun/git-cliff/commit/c41bbe05a2594ea945994de97f77b1ec292f49b1))
- *(readme)* Apply formatting - ([a4290c7](https://github.com/orhun/git-cliff/commit/a4290c7be832aaac894f9b4e69837f0eb13d9287))
- *(website)* Update the style for environment variable section - ([24ba433](https://github.com/orhun/git-cliff/commit/24ba43330a3b98d1d2c11dfa2e92c44eeabe2b7d))

### 🧪 Testing

- *(deploy)* Test the website deployment for pull requests - ([6f8a2f9](https://github.com/orhun/git-cliff/commit/6f8a2f96da1239f1261e597a6a358d9261f2c5e0))

### ⚙️ Miscellaneous Tasks

- *(cargo)* Update MSRV to 1.64.0 - ([84f20f9](https://github.com/orhun/git-cliff/commit/84f20f906fbb7995c656bd0ea14432ff6ff8d969))
- *(cd)* Temporarily switch back to action-rs/toolchain - ([1f897e3](https://github.com/orhun/git-cliff/commit/1f897e3d70dfe278de824c508febec42855878f3))
- *(ci)* Switch to dtolnay/rust-toolchain action - ([d54f72a](https://github.com/orhun/git-cliff/commit/d54f72aff416fcf6f4897ff69b0c114796f154e1))
- *(ci)* Update runner images - ([07e7938](https://github.com/orhun/git-cliff/commit/07e7938b6eb4dc9cf789e84bd007f02c500d31e9))
- *(docker)* Update versions in Dockerfile - ([95e8408](https://github.com/orhun/git-cliff/commit/95e8408ae661bb5fe003e2388d8449b1eb7f168c))
- *(docker)* Bump the action versions in docker workflow - ([c02538f](https://github.com/orhun/git-cliff/commit/c02538f85c1cf4e8ecba8556dad9c9e806de9e49))
- *(docker)* Bump build-push-action to v4 - ([0c84ed8](https://github.com/orhun/git-cliff/commit/0c84ed8eea663d9d78912080d3fe953f35c39216))
- *(editorconfig)* Fix editorconfig syntax - ([e35d83d](https://github.com/orhun/git-cliff/commit/e35d83d2cfb88b0552f648eda9193ffab9f0bbd4))
- *(editorconfig)* Update editorconfig for better code readability - ([8a4f928](https://github.com/orhun/git-cliff/commit/8a4f928f9e925cbf70ad373d71b4f677817ac8f2))
- *(examples)* Remove EXAMPLES.md - ([8038659](https://github.com/orhun/git-cliff/commit/80386597033090e427d1359b4797ee5dbbf113e5))
- *(github)* Integrate Dependabot - ([fe5a4dd](https://github.com/orhun/git-cliff/commit/fe5a4dd8f0fe4997f340967082ee9204f345a2a4))
- *(github)* Integrate bors - ([1ab6f39](https://github.com/orhun/git-cliff/commit/1ab6f39c849fe191992257622ea03618d76b3464))
- *(github)* Add contact links for issues - ([d3cb25b](https://github.com/orhun/git-cliff/commit/d3cb25bc9962b348e240b73ccd513b90c2b765e0))
- *(website)* Add workflow file for deploying the website - ([f550458](https://github.com/orhun/git-cliff/commit/f550458414f4c35dd1c23e3d05d1115424404a3c))
- *(website)* Move website to website folder - ([5644d10](https://github.com/orhun/git-cliff/commit/5644d1036508ffb420e12503adb671708e087cd9))
- *(website)* Move website to docs for GitHub pages deployment - ([b6e52e1](https://github.com/orhun/git-cliff/commit/b6e52e128e7a105682adf885850d14deefdff3ec))

## [1.1.2](https://github.com/orhun/git-cliff/compare/v1.1.1..v1.1.2) - 2023-01-20

### 🐛 Bug Fixes

- *(changelog)* Allow saving context to a file ([#138](https://github.com/orhun/git-cliff/issues/138)) - ([08ea900](https://github.com/orhun/git-cliff/commit/08ea900de09a124710d07783b2271cdbf453dd94))
- *(changelog)* Do not skip all tags when `skip_tags` is empty ([#136](https://github.com/orhun/git-cliff/issues/136)) - ([eee35ad](https://github.com/orhun/git-cliff/commit/eee35ad1eeeb89313e69086ad265ab400a6b4898))
- *(git)* Derive the tag order from commits instead of timestamp ([#139](https://github.com/orhun/git-cliff/issues/139)) - ([4df5656](https://github.com/orhun/git-cliff/commit/4df5656c1239b0252ce9e7571efa06e9d11490ea))

### 🎨 Styling

- *(fmt)* Update the derives in config module - ([336b25f](https://github.com/orhun/git-cliff/commit/336b25f3ae9da5d972304662fcdb88f4719c721d))

### ⚙️ Miscellaneous Tasks

- *(cargo)* Add metadata for cargo-binstall - ([31dac34](https://github.com/orhun/git-cliff/commit/31dac3410c04b837c7ede271faf9d862c58966af))
- *(docker)* Update versions in Dockerfile - ([353ca62](https://github.com/orhun/git-cliff/commit/353ca62260e8d71c68ec5a3a4ce1abe795f2cab5))

### ◀️ Revert

- *(git)* Use timestamp for deriving the tag order ([#139](https://github.com/orhun/git-cliff/issues/139)) - ([accfb0f](https://github.com/orhun/git-cliff/commit/accfb0fcdd06c66d3e9d98f8848cbb9ab0944d09))

## [1.1.1](https://github.com/orhun/git-cliff/compare/v1.1.0..v1.1.1) - 2023-01-09

### 🐛 Bug Fixes

- *(npm)* Fix the type casting in base NPM package - ([bc0807f](https://github.com/orhun/git-cliff/commit/bc0807fe935d1ea1b1fe81025f8f4da0aad3387c))
- *(npm)* Fix the variable declaration for NPM package OS - ([f40a565](https://github.com/orhun/git-cliff/commit/f40a56588e22de2f6ed83e9d7b333da11cc0a797))
- *(npm)* Rename the NPM binary package for Windows - ([ce1d468](https://github.com/orhun/git-cliff/commit/ce1d468f0bd045f584d2ce4c0ed5f046cdc13777))

### 📚 Documentation

- *(readme)* Update README.md about the NPM package - ([e0177c2](https://github.com/orhun/git-cliff/commit/e0177c25e13812306aab0b0991562d58b6d14767))

### ⚙️ Miscellaneous Tasks

- *(cd)* Parallelize releasing on crates.io - ([24c8e3e](https://github.com/orhun/git-cliff/commit/24c8e3e4cf092dc347f90c6621238d86f0001f2a))
- *(cd)* Add README.md to the base NPM package - ([e2e124e](https://github.com/orhun/git-cliff/commit/e2e124ed4bc9ff77413af1b5cd075f5a55ca98de))
- *(npm)* Add more keywords to the base NPM package - ([abe68a2](https://github.com/orhun/git-cliff/commit/abe68a28847ec9d444337fb1adec522fca7aac1b))
- *(npm)* Package `git-cliff` for npm ([#133](https://github.com/orhun/git-cliff/issues/133)) - ([b7dd592](https://github.com/orhun/git-cliff/commit/b7dd592653a722a764609a3eacff5e1eee58c07e))

## [1.1.0](https://github.com/orhun/git-cliff/compare/v1.0.0..v1.1.0) - 2023-01-08

### ⛰️  Features

- *(git)* Support generating changelog for multiple git repositories ([#13](https://github.com/orhun/git-cliff/issues/13)) - ([8b17a1f](https://github.com/orhun/git-cliff/commit/8b17a1f02619027bebc5df1a8938aaf76adcd631))

### 🚜 Refactor

- *(cd)* Use the git-cliff-action output for GitHub release body - ([03cf3a7](https://github.com/orhun/git-cliff/commit/03cf3a7028ec273bf23085d4bdac729422d83b42))

### 📚 Documentation

- *(readme)* Update copyright years - ([261ee4f](https://github.com/orhun/git-cliff/commit/261ee4f146ac7d68353dd052c62a510aadebfad4))
- *(readme)* Disable Liquid parsing in README.md by using raw blocks - ([6e8c7ed](https://github.com/orhun/git-cliff/commit/6e8c7edd3ab3a7efb53bb8197407c295a8252396))

### ⚙️ Miscellaneous Tasks

- *(cd)* Publish binaries for more platforms/architectures - ([ce1b7c3](https://github.com/orhun/git-cliff/commit/ce1b7c3d7b7622af03994bd7a2fbcbb00a7be8bf))
- *(cd)* Bump git-cliff-action to v2 - ([1b3cba8](https://github.com/orhun/git-cliff/commit/1b3cba8dca0d8e9da76c9d7c2444d04a3cd30528))
- *(config)* Update the description in the default config - ([0350bfd](https://github.com/orhun/git-cliff/commit/0350bfdee229ee50bb289b68190b9737d7ab572c))
- *(docker)* Add Jekyll configuration to .dockerignore - ([5dd1a15](https://github.com/orhun/git-cliff/commit/5dd1a15a6cf393de8efd1a63de5374f65e706120))
- *(github)* Add Jekyll theme configuration for GitHub pages - ([81e5720](https://github.com/orhun/git-cliff/commit/81e5720376346a2b0d3ef5a3ef4408507044f6e0))
- *(release)* Improve the release script with additional messages - ([09ab59f](https://github.com/orhun/git-cliff/commit/09ab59f12e8d295e607966f295b39d3ad2457fd0))

## [1.0.0](https://github.com/orhun/git-cliff/compare/v0.10.0..v1.0.0) - 2022-12-25

### ⛰️  Features

- *(cd)* Publish Debian package via release workflow ([#113](https://github.com/orhun/git-cliff/issues/113)) - ([efd827f](https://github.com/orhun/git-cliff/commit/efd827f59f8394dd894ebd35a5d630ff558a3ebe))
- *(cd)* Include completions and mangen in binary releases ([#115](https://github.com/orhun/git-cliff/issues/115)) - ([9a070b2](https://github.com/orhun/git-cliff/commit/9a070b248d4ae0b58c9463f0627c87ca647c3023))
- *(changelog)* [**breaking**] Use current time for `--tag` argument ([#107](https://github.com/orhun/git-cliff/issues/107)) - ([e2cd07b](https://github.com/orhun/git-cliff/commit/e2cd07bcc92a6bdd011bbbb34843f22e6c4da271))
- *(changelog)* Allow running with `--prepend` and `--output` ([#120](https://github.com/orhun/git-cliff/issues/120)) - ([7325be8](https://github.com/orhun/git-cliff/commit/7325be84045ad376e0989a111ed3c44a3e1400ea))
- *(changelog, config)* [**breaking**] Replace `--date-order` by `--topo-order` - ([77731ec](https://github.com/orhun/git-cliff/commit/77731ec7aeb279b5b7a49b5f7d17cc51009afca2))

### 🐛 Bug Fixes

- *(fixtures)* Fix test fixture failures - ([29b3dd1](https://github.com/orhun/git-cliff/commit/29b3dd15982f8645d3f75c185d7a5adbfbb2a06f))

### 🚜 Refactor

- *(clippy)* Apply clippy suggestions - ([352e884](https://github.com/orhun/git-cliff/commit/352e884877a7014f6b3fc6c6ccea9fed4c57c56c))

### 📚 Documentation

- *(readme)* Fix GitHub badges in README.md - ([acf2d52](https://github.com/orhun/git-cliff/commit/acf2d52602c008352de9ef98df7bb8d6f19b5222))

### 🎨 Styling

- *(readme)* Update README.md about the styling of footer field - ([47a7345](https://github.com/orhun/git-cliff/commit/47a7345167b78f824c80e41f9f8e2bf9df53d654))

### ⚙️ Miscellaneous Tasks

- *(cd)* Remove deprecated set-output usage - ([5187f02](https://github.com/orhun/git-cliff/commit/5187f029ec3d004a0acf7ffacec4621cce3ec1f1))
- *(ci)* Update actions/checkout to v3 - ([6c37611](https://github.com/orhun/git-cliff/commit/6c37611e162adb71d78203ad7d24d7c7f17774e3))
- *(config)* Comment out custom commit preprocessor ([#112](https://github.com/orhun/git-cliff/issues/112)) - ([8f77caf](https://github.com/orhun/git-cliff/commit/8f77caf86a1e5dd23eda1b9e9b5a7a6606642b8a))
- *(fixtures)* Run all test fixtures - ([53c1c50](https://github.com/orhun/git-cliff/commit/53c1c50a1e1a66c684bb1319c0bf48648ed01eab))

## [0.10.0](https://github.com/orhun/git-cliff/compare/v0.9.2..v0.10.0) - 2022-11-20

### ⛰️  Features

- *(args)* Add a short variant `-d` for specifying `--date-order` flag - ([5913e24](https://github.com/orhun/git-cliff/commit/5913e24596a32625ce59ca819cbcf3329e7b3b5b))
- *(changelog)* Do not skip breaking changes if configured ([#114](https://github.com/orhun/git-cliff/issues/114)) - ([1c98995](https://github.com/orhun/git-cliff/commit/1c98995454f2df1e6766d55e026c16e857aa938b))
- *(config)* Changelog for the last n commits ([#116](https://github.com/orhun/git-cliff/issues/116)) - ([0c7769b](https://github.com/orhun/git-cliff/commit/0c7769b52fe3dee6afd0321c58021cf157acb964))

### 🐛 Bug Fixes

- *(changelog)* Warn against invalid tag range for `--current` flag ([#124](https://github.com/orhun/git-cliff/issues/124)) - ([e73fd9f](https://github.com/orhun/git-cliff/commit/e73fd9f821a5f16ab2581839be17c0c5ade85dc6))
- *(docker)* Fix syntax error in Dockerfile - ([5f9b2d5](https://github.com/orhun/git-cliff/commit/5f9b2d5d02d75c49d11e930ac80657eabb882140))
- *(docker)* Use an alternative method to fetch registry - ([876b13b](https://github.com/orhun/git-cliff/commit/876b13b1deea184cb423b82bccec7d6b7bf5bde4))

### 🚜 Refactor

- *(deps)* Utilize workspace dependencies - ([f2def40](https://github.com/orhun/git-cliff/commit/f2def401ba0b2b5aa4092b7167cd334d5bd54cd8))
- *(docker)* Improve cargo-chef caching in Dockerfile - ([0f38960](https://github.com/orhun/git-cliff/commit/0f38960851ac0fd159727d5dffb36f50268eec18))

### 📚 Documentation

- *(readme)* Update badge URL for Docker builds - ([a8fa7f9](https://github.com/orhun/git-cliff/commit/a8fa7f9fb5e334ff58e9ae371cc2ffb0a873c345))
- *(readme)* Add MacPorts install info ([#111](https://github.com/orhun/git-cliff/issues/111)) - ([f9d4b88](https://github.com/orhun/git-cliff/commit/f9d4b88a3324a10b918bab8c272a60214bcdcd13))

### ⚙️ Miscellaneous Tasks

- *(docker)* Update versions in Dockerfile - ([02e2b8e](https://github.com/orhun/git-cliff/commit/02e2b8e58e0e4a518fe5318be2bec6d1360ad34e))

## [0.9.2](https://github.com/orhun/git-cliff/compare/v0.9.1..v0.9.2) - 2022-09-24

### 🐛 Bug Fixes

- *(docker)* Remove custom user creation from the Dockerfile ([#109](https://github.com/orhun/git-cliff/issues/109)) - ([5cb991d](https://github.com/orhun/git-cliff/commit/5cb991d4e3a39dd15ae22b661c23d18ccbd45004))

### ⚙️ Miscellaneous Tasks

- *(audit)* Remove cargo-audit config - ([078bdc3](https://github.com/orhun/git-cliff/commit/078bdc3f7a482e752bb983fad057a37f15528698))
- *(ci)* Switch to cargo-tarpaulin for measuring code coverage ([#110](https://github.com/orhun/git-cliff/issues/110)) - ([17f3a09](https://github.com/orhun/git-cliff/commit/17f3a0994d85022650170ff3a9fef942aa414303))

## [0.9.1](https://github.com/orhun/git-cliff/compare/v0.9.0..v0.9.1) - 2022-09-20

### 🐛 Bug Fixes

- *(docker)* Configure git safe.directory for Docker image ([#108](https://github.com/orhun/git-cliff/issues/108)) - ([4fc2217](https://github.com/orhun/git-cliff/commit/4fc2217868fceea81ab5e6aeeb63ca719a07fe91))

### 🚜 Refactor

- *(clippy)* Apply clippy suggestions - ([055a057](https://github.com/orhun/git-cliff/commit/055a05708b5b893c452a30653d4764c1d2da7327))

### 🎨 Styling

- *(readme)* Update styling for with-commit example - ([8247301](https://github.com/orhun/git-cliff/commit/82473017ca627a8d736099a928f03cfb56c895dc))

## [0.9.0](https://github.com/orhun/git-cliff/compare/v0.8.1..v0.9.0) - 2022-08-16

### ⛰️  Features

- *(changelog)* Support setting commit SHA while using `--with-commit` - ([d453d4c](https://github.com/orhun/git-cliff/commit/d453d4cbebbb607ff7dbe530542802e8ca60b585))
- *(changelog)* Support splitting commits by lines ([#101](https://github.com/orhun/git-cliff/issues/101)) - ([e3eae33](https://github.com/orhun/git-cliff/commit/e3eae33abcf212e56efcdbb873411e7d8e761b4f))
- *(commit)* Add commit author and committer to the context ([#100](https://github.com/orhun/git-cliff/issues/100)) - ([940065b](https://github.com/orhun/git-cliff/commit/940065b5754d4db4d6bb24c1ac8c7c5ddba75af0))

### 🚜 Refactor

- *(clippy)* Run clippy for tests - ([1b5dcc1](https://github.com/orhun/git-cliff/commit/1b5dcc105156ee287d5b484a135816debf1990cb))
- *(commit)* Use a more concise conversion for string - ([1b13b97](https://github.com/orhun/git-cliff/commit/1b13b97f7359987e027cfc55dc52b3ea40894d5b))

### 📚 Documentation

- *(readme)* Add test repository link to README.md - ([da484a3](https://github.com/orhun/git-cliff/commit/da484a33644acda9f93f46e48fae2186ce0397e2))

### ⚙️ Miscellaneous Tasks

- *(build)* Enable strip option for release profile - ([f70fefd](https://github.com/orhun/git-cliff/commit/f70fefd88acae2e599880af5a8bcff7710fbe6c1))
- *(docker)* Upgrade versions in Dockerfile - ([6bb4f5b](https://github.com/orhun/git-cliff/commit/6bb4f5b43e257941191a49756e3388aacb5f4978))
- *(docker)* Disable updating the description on Docker Hub - ([cd1306c](https://github.com/orhun/git-cliff/commit/cd1306cffdd01cc002c68d0e2297fa5a4589766b))
- *(docker)* Update the description on Docker Hub on push - ([9c0e7a2](https://github.com/orhun/git-cliff/commit/9c0e7a2ab8c3efdfb1be66ac55fba519b9bbb20c))
- *(docker)* Enable building arm64 docker images - ([f2968cd](https://github.com/orhun/git-cliff/commit/f2968cdf6ca1bfc9126c07af0894c0b2bd7f9a64))
- *(docker)* Use an alternative method to fetch registry - ([220d6e6](https://github.com/orhun/git-cliff/commit/220d6e64f4c786f1e4e353e506f98cb9eefb34ec))
- *(funding)* Add GitHub Sponsors option for funding - ([f3fada7](https://github.com/orhun/git-cliff/commit/f3fada723d680dab4f0cd435dc0430425a7fe995))
- *(project)* Update MSRV to 1.60.0 - ([b55e678](https://github.com/orhun/git-cliff/commit/b55e678a4ea669e195d0adae0694a340ab724c31))

## [0.8.1](https://github.com/orhun/git-cliff/compare/v0.8.0..v0.8.1) - 2022-07-12

### 🐛 Bug Fixes

- *(cd)* Set fail-fast strategy to false - ([4b2ded0](https://github.com/orhun/git-cliff/commit/4b2ded0cf17d57b76c505372bebd098256200c5a))

### ⚙️ Miscellaneous Tasks

- *(cd)* Update windows runners to windows-2022 - ([8621a59](https://github.com/orhun/git-cliff/commit/8621a59d47b9e13dd449e6c781e847086e501153))

## [0.8.0](https://github.com/orhun/git-cliff/compare/v0.7.0..v0.8.0) - 2022-07-12

### ⛰️  Features

- *(changelog)* Support external commands for commit preprocessors ([#86](https://github.com/orhun/git-cliff/issues/86)) - ([7d0786c](https://github.com/orhun/git-cliff/commit/7d0786ca55423950f0779de4e6a907fc25ae203a))
- *(commit)* [**breaking**] Pass footer token and separator to template ([#97](https://github.com/orhun/git-cliff/issues/97)) - ([0bf499e](https://github.com/orhun/git-cliff/commit/0bf499ec940a22a2bed052db21e89f7fc4b824ba))
- *(config)* Support changing commit scope with `commit_parsers` ([#94](https://github.com/orhun/git-cliff/issues/94)) - ([e220768](https://github.com/orhun/git-cliff/commit/e22076843b91be3680617db5686e070dedcfef29))

### 🐛 Bug Fixes

- *(ci)* Update lychee arguments to skip checking protonmail - ([a5aaca1](https://github.com/orhun/git-cliff/commit/a5aaca1a01e6e380c35a70bc512cb11d17e4b964))

### 🚜 Refactor

- *(clippy)* Apply clippy suggestions - ([34270c8](https://github.com/orhun/git-cliff/commit/34270c860542323ff72d005ab018436474ccbfc7))
- *(clippy)* Apply clippy suggestions - ([8d8981c](https://github.com/orhun/git-cliff/commit/8d8981c6b1900b7c04150f739b7841c1164b4d6d))

### 📚 Documentation

- *(readme)* Switch chronological and topological ([#99](https://github.com/orhun/git-cliff/issues/99)) - ([2289199](https://github.com/orhun/git-cliff/commit/22891992a2e7898238b9d4e154bfffc6d84b180f))
- *(readme)* Clarify that `--tag` argument can be an unexisting tag - ([d540f5d](https://github.com/orhun/git-cliff/commit/d540f5d8938bc84b01b4fafaa69c3290eb72cd08))

### ⚙️ Miscellaneous Tasks

- *(docker)* Disable building arm64 docker images temporarily - ([175f7d7](https://github.com/orhun/git-cliff/commit/175f7d70559c642721c0c82215224cfba2cb0221))
- *(project)* Set MSRV to 1.58.1 ([#87](https://github.com/orhun/git-cliff/issues/87)) - ([bfcd0d9](https://github.com/orhun/git-cliff/commit/bfcd0d97ba2fc2271e754f6c9ecb834edf7f1190))

## [0.7.0](https://github.com/orhun/git-cliff/compare/v0.6.1..v0.7.0) - 2022-04-24

### ⛰️  Features

- *(args)* [**breaking**] Prefix environment variables with `GIT_CLIFF_` ([#76](https://github.com/orhun/git-cliff/issues/76)) - ([84507dd](https://github.com/orhun/git-cliff/commit/84507dd361f4c1ba37bfdb7005714bb817afb6b0))
- *(args)* Add `--context` flag for outputting context ([#71](https://github.com/orhun/git-cliff/issues/71)) - ([95ad55d](https://github.com/orhun/git-cliff/commit/95ad55d542cb772881f12ae2a25f7e06202c8587))
- *(cli)* Show a message if a newer version is available ([#69](https://github.com/orhun/git-cliff/issues/69)) - ([720a7c1](https://github.com/orhun/git-cliff/commit/720a7c1ec2cde510733d119b7b3b8a938ff945ff))
- *(config)* Support placing configuration inside Cargo.toml ([#46](https://github.com/orhun/git-cliff/issues/46)) - ([f48d207](https://github.com/orhun/git-cliff/commit/f48d2077c33b878337edc2e5fe8a97be990b4773))
- *(git)* Support preprocessing commit messages using regex ([#62](https://github.com/orhun/git-cliff/issues/62)) - ([64317f2](https://github.com/orhun/git-cliff/commit/64317f21168a9e9aa088befbd2841632f72e13a3))
- *(log)* Print more debug information when `-vv` is used ([#79](https://github.com/orhun/git-cliff/issues/79)) - ([a8efffc](https://github.com/orhun/git-cliff/commit/a8efffc7d57691999583a5a13bfd5d0e48aca095))
- *(man)* Add man page generation script ([#35](https://github.com/orhun/git-cliff/issues/35)) - ([03d55c8](https://github.com/orhun/git-cliff/commit/03d55c8eb1483fa783ea724b12bdd209fc2eaca2))

### 🐛 Bug Fixes

- *(build)* Pin the Rust nightly version - ([97c3044](https://github.com/orhun/git-cliff/commit/97c30449a1b7f9a551c5c57fc7518027aaf52f3b))
- *(changelog)* Allow custom commit range while prepending ([#68](https://github.com/orhun/git-cliff/issues/68)) - ([1bacc7f](https://github.com/orhun/git-cliff/commit/1bacc7f194f70fea5378d2609dd72e64eb62bdfb))
- *(ci)* Pin the Rust nightly version - ([1b04dbf](https://github.com/orhun/git-cliff/commit/1b04dbf1a8760281babcf699c0126063c131d6fe))
- *(fixtures)* Update expected changelog date - ([2b484f0](https://github.com/orhun/git-cliff/commit/2b484f078cb0c5236aa10fbb8c375e5b368ec0a1))
- *(log)* Remove redundant logging while using `--context` ([#71](https://github.com/orhun/git-cliff/issues/71)) - ([efd40e0](https://github.com/orhun/git-cliff/commit/efd40e02b3c7be29da6e2a161423b92a74c4f46e))

### 🚜 Refactor

- *(cli)* Make update-informer opt-out via feature flag ([#69](https://github.com/orhun/git-cliff/issues/69)) - ([cddb4d4](https://github.com/orhun/git-cliff/commit/cddb4d49fb03191208bc61d00cbaafde62ad8f92))
- *(completions)* Use implicit Result type in completions script - ([fa2639a](https://github.com/orhun/git-cliff/commit/fa2639aafe6082d937534d77f8ae3268f718b722))

### 📚 Documentation

- *(readme)* Update the title of projects section - ([4f4a82c](https://github.com/orhun/git-cliff/commit/4f4a82cbe3d7e03058a77f7757cf138716353b2a))
- *(readme)* Add `cliff-jumper` to similar projects ([#83](https://github.com/orhun/git-cliff/issues/83)) - ([2a21890](https://github.com/orhun/git-cliff/commit/2a218902d3d42ea233d8cf087944b575d05399c3))
- *(readme)* Update GitHub Actions reference link in README.md - ([9801963](https://github.com/orhun/git-cliff/commit/980196357bbf41c4b7596b81237ce36051196b6f))
- *(readme)* Add more regex examples for commit_preprocessors - ([9b83518](https://github.com/orhun/git-cliff/commit/9b83518a59cb8275f58f6076bd5ff23be606df2a))

### 🎨 Styling

- *(release)* Update the changelog template for tag message - ([72544b1](https://github.com/orhun/git-cliff/commit/72544b18073295362174200189a0f4e165c6d296))

### ⚙️ Miscellaneous Tasks

- *(cd)* Include man page in the release assets - ([a5ddf75](https://github.com/orhun/git-cliff/commit/a5ddf75152764bce42b9b5484989aea227d175b2))
- *(ci)* Return to nightly builds ([#73](https://github.com/orhun/git-cliff/issues/73)) - ([312b654](https://github.com/orhun/git-cliff/commit/312b654b07f000f49a7d1a3d1b9b4649c37842fe))
- *(docker)* Strip the binaries in Docker image - ([aca4ccf](https://github.com/orhun/git-cliff/commit/aca4ccfb7ff8c47bbe3c16203ef617dde94d3ad5))
- *(docker)* Disable default features for the Docker image - ([e6fb20d](https://github.com/orhun/git-cliff/commit/e6fb20d11c50c2989abecd27b7fb325d9d3ac490))
- *(docker)* Build Docker images for arm64 - ([8475e1f](https://github.com/orhun/git-cliff/commit/8475e1fd63b89bb56c2cf68de62dbb4d9e66b4bb))
- *(docker)* Upgrade versions in Dockerfile - ([3aa9a1a](https://github.com/orhun/git-cliff/commit/3aa9a1a059f876b66ce03bc1a4a7735a2c27e146))

## [0.6.1](https://github.com/orhun/git-cliff/compare/v0.6.0..v0.6.1) - 2022-03-13

### 🐛 Bug Fixes

- *(changelog)* Use root commit when --latest and there is only one tag ([#59](https://github.com/orhun/git-cliff/issues/59)) - ([3ccec7f](https://github.com/orhun/git-cliff/commit/3ccec7f93a917a4feaa5baf17b604fe3de76b0e1))
- *(changelog)* Do not skip all tags when `skip_tags` is empty ([#63](https://github.com/orhun/git-cliff/issues/63)) - ([ff1d981](https://github.com/orhun/git-cliff/commit/ff1d981fd9a5dba26422f56582e06b3b463eb8a3))
- *(example)* Fix `keepachangelog` config example ([#66](https://github.com/orhun/git-cliff/issues/66)) - ([9b5f0bb](https://github.com/orhun/git-cliff/commit/9b5f0bb5fdadf15cccb738f1bb96937be058795e))
- *(project)* Use the correct branch for codecov ([#65](https://github.com/orhun/git-cliff/issues/65)) - ([8f3325e](https://github.com/orhun/git-cliff/commit/8f3325e758d25d814c5c9831d128907696a12536))

### 📚 Documentation

- *(core)* Document timestamp format of `Release` struct ([#67](https://github.com/orhun/git-cliff/issues/67)) - ([d68eb12](https://github.com/orhun/git-cliff/commit/d68eb120c0a0a98bc1e7264a3aede17b5f5c54be))
- *(readme)* Add another option of GitHub Actions ([#64](https://github.com/orhun/git-cliff/issues/64)) - ([db7edf5](https://github.com/orhun/git-cliff/commit/db7edf5707f2bfdf49c749026969fd1833530ed7))

## [0.6.0](https://github.com/orhun/git-cliff/compare/v0.5.0..v0.6.0) - 2022-02-12

### ⛰️  Features

- *(changelog)* [**breaking**] Use conventional commit body to check against commit parsers - ([e1da611](https://github.com/orhun/git-cliff/commit/e1da61150f07f641dfe471e240033e13cc19d089))
- *(changelog)* Add `link_parsers` for parsing/extracting links ([#42](https://github.com/orhun/git-cliff/issues/42)) - ([b88e7d3](https://github.com/orhun/git-cliff/commit/b88e7d30bee74667028602fed1337ceb24829145))
- *(changelog, config)* [**breaking**] Replace --topo-order by --date-order ([#58](https://github.com/orhun/git-cliff/issues/58)) - ([a3980f4](https://github.com/orhun/git-cliff/commit/a3980f4632cea95d939c044ef5687123f5b91546))
- *(config)* Make the `changelog` section optional ([#45](https://github.com/orhun/git-cliff/issues/45)) - ([e02ae0b](https://github.com/orhun/git-cliff/commit/e02ae0b3661b3379175a10cb273a9c7744747765))
- *(config)* Make the `git` section optional ([#45](https://github.com/orhun/git-cliff/issues/45)) - ([8202e37](https://github.com/orhun/git-cliff/commit/8202e37dbd5dd98b3f9de8470e41776b5afb1b51))

### 🐛 Bug Fixes

- *(changelog)* Set the previous release when using `--unreleased` ([#47](https://github.com/orhun/git-cliff/issues/47)) - ([2be04f8](https://github.com/orhun/git-cliff/commit/2be04f8b2214513d08d349b254a97a8c783073fb))
- *(changelog)* Only drop previous releases if skipped ([#44](https://github.com/orhun/git-cliff/issues/44)) - ([943c23f](https://github.com/orhun/git-cliff/commit/943c23fd350eea1154deb2e294257c22d2bc76e5))
- *(ci)* Update grcov download command - ([5bfb454](https://github.com/orhun/git-cliff/commit/5bfb45411da940eb7d0df874558b31b50911bb59))
- *(ci)* Use the correct tar command for extracting grcov archive - ([a3f3aa6](https://github.com/orhun/git-cliff/commit/a3f3aa6405846419fabeafab5fea204ec0e4be9b))
- *(ci)* Update the download link of latest grcov release - ([c47133a](https://github.com/orhun/git-cliff/commit/c47133ac2423f7581b711fa97f1b30094907a3c2))
- *(ci)* Run clippy from nightly toolchain - ([7d766d7](https://github.com/orhun/git-cliff/commit/7d766d7e34726faf69f91a3941f1470356e4dda5))
- *(config)* Lower the priority of global configuration file ([#51](https://github.com/orhun/git-cliff/issues/51)) - ([2595952](https://github.com/orhun/git-cliff/commit/25959529d60340caac668e0dd3e5c5b105ab4290))
- *(test)* Update tests about optional config values - ([8bb48b0](https://github.com/orhun/git-cliff/commit/8bb48b09ef4488e4fb6b03a43e1d862b4645971b))
- *(tests)* Update custom error tests - ([58165c7](https://github.com/orhun/git-cliff/commit/58165c730e4ef370a448001193d90ff29a57449d))

### 🚜 Refactor

- *(clippy)* Apply clippy suggestions - ([cd018e9](https://github.com/orhun/git-cliff/commit/cd018e90a15e9960c0998bbfdd284d65029691f2))
- *(config)* [**breaking**] Change the default value of `trim` to `true` - ([3b3ef7e](https://github.com/orhun/git-cliff/commit/3b3ef7e4d8d2cb680419e7175bc948b895c7de24))
- *(lib)* Unify serde and serde_derive using derive feature ([#57](https://github.com/orhun/git-cliff/issues/57)) - ([bedabc9](https://github.com/orhun/git-cliff/commit/bedabc93ddc30be69d27cbb42b23d3ff68e96f95))

### 📚 Documentation

- *(config)* Add minimal example - ([848d8a5](https://github.com/orhun/git-cliff/commit/848d8a587efd5f611a98b647b954c06938fac24a))
- *(readme)* Update copyright years - ([0a3c56c](https://github.com/orhun/git-cliff/commit/0a3c56c7ecdf01133d0e857269076052febadd91))
- *(readme)* Update template contexts about link_parsers - ([dce09d7](https://github.com/orhun/git-cliff/commit/dce09d71a05f5d1e1d8939d688d19c4740ba6a93))

### 🎨 Styling

- *(changelog)* Comply with MD022 and fix minor typos ([#61](https://github.com/orhun/git-cliff/issues/61)) - ([0293b28](https://github.com/orhun/git-cliff/commit/0293b281090f74a5855678acbb3dc9a259ba7126))
- *(readme)* Update the styling - ([dcb3141](https://github.com/orhun/git-cliff/commit/dcb3141ac969c52009f3f13314da65f1cf0e2604))

### ⚙️ Miscellaneous Tasks

- *(args)* Update arg parsing to clap v3 ([#49](https://github.com/orhun/git-cliff/issues/49)) - ([d961b53](https://github.com/orhun/git-cliff/commit/d961b53ba5ceb99adccfc5df3909c96cda682341))
- *(cd)* Update the runner to macos-11 - ([960cb4a](https://github.com/orhun/git-cliff/commit/960cb4ac6f4ffb0398a39c36637f53c1307d44ab))
- *(ci)* Run cargo-audit for checking vulnerabilities - ([cfe41fe](https://github.com/orhun/git-cliff/commit/cfe41fe56eddb38c109e178e02d3567d10ad78ff))
- *(docker)* Bump the Rust version in Dockerfile - ([d4cbb85](https://github.com/orhun/git-cliff/commit/d4cbb857388f8d5686715fcba62f8adaeb92230a))

## [0.5.0](https://github.com/orhun/git-cliff/compare/v0.4.2..v0.5.0) - 2021-12-15

### ⛰️  Features

- *(args)* Add `--with-commit` argument for including custom commit messages in changelog - ([e4c60b2](https://github.com/orhun/git-cliff/commit/e4c60b20be8b7f1fa19429b1b1f984dc4caf9340))
- *(args)* Add `--current` flag for processing the current tag ([#37](https://github.com/orhun/git-cliff/issues/37)) - ([02a6187](https://github.com/orhun/git-cliff/commit/02a6187a58583f27e4604d7ea518b52b4bc7a833))
- *(args)* Add `--exclude-path` argument for excluding related commits - ([25a1d49](https://github.com/orhun/git-cliff/commit/25a1d49c0993685d8bf95225e81ee7d614131115))
- *(args)* Support multiple values for `--commit-path` argument - ([edb343a](https://github.com/orhun/git-cliff/commit/edb343a10e76a33b7223aa36d37df350d4ac6df1))
- *(args)* Accept glob patterns for `--commit-path` argument - ([ad11cbf](https://github.com/orhun/git-cliff/commit/ad11cbf6c528e1cf80075d986658774be3fabff7))
- *(changelog)* Support having both conventional and unconventional commits in the changelog - ([8445313](https://github.com/orhun/git-cliff/commit/8445313b13f6f087f79ea73bc1c12b0340a87d92))
- *(changelog)* Add `--topo-order` flag for sorting tags ([#29](https://github.com/orhun/git-cliff/issues/29)) - ([cc09d63](https://github.com/orhun/git-cliff/commit/cc09d637ff4edfcba625e469dcd3eb0062ac2a4f))
- *(config)* Add `ignore_tags` option ([#40](https://github.com/orhun/git-cliff/issues/40)) - ([de11cce](https://github.com/orhun/git-cliff/commit/de11ccecac10de6069d7a1ba0a1013582fc8bd25))
- *(config)* Support specifying the sorting methods in config ([#31](https://github.com/orhun/git-cliff/issues/31)) - ([4eb334d](https://github.com/orhun/git-cliff/commit/4eb334da06c1bde3b53d95ea50d086daab07e4bb))
- *(template)* Use more explanatory error messages about templates - ([1a9c3e3](https://github.com/orhun/git-cliff/commit/1a9c3e310f1b7ae8de6f5d62bd2095afd616c463))

### 🐛 Bug Fixes

- *(args)* Override the sort related config if args are present ([#39](https://github.com/orhun/git-cliff/issues/39)) - ([ef63727](https://github.com/orhun/git-cliff/commit/ef63727b5f0a5aba7024e2afe5dc24a1b218d978))
- *(changelog)* Drop the skipped releases from 'previous' field - ([7f867ae](https://github.com/orhun/git-cliff/commit/7f867ae647ff30f54aae314596cbc7c7ce4f50c1))
- *(fixtures)* Strip the carriage return on fixtures while comparing - ([d7e8ce2](https://github.com/orhun/git-cliff/commit/d7e8ce25286a2dc1ce5d134df871cdc07f4a9211))
- *(fixtures)* Update the multi line docker command - ([c8d288c](https://github.com/orhun/git-cliff/commit/c8d288c4c8dafd011b2d324d3ba3052b0fe11794))
- *(fixtures)* Use the defined configuration file for fixtures - ([bbc58d7](https://github.com/orhun/git-cliff/commit/bbc58d7bd01091b71c38323ba71cc07b97285c19))
- *(fixtures)* Checkout the repository before running fixtures - ([cb412a9](https://github.com/orhun/git-cliff/commit/cb412a905f7121bb3277de1086fb48f34bbb7319))
- *(tests)* Update log test about exclude path - ([9d213f5](https://github.com/orhun/git-cliff/commit/9d213f5713af3a27e1026d275be1676a739d6c3a))

### 🚜 Refactor

- *(config)* Rename the config value for commit order - ([6cec37d](https://github.com/orhun/git-cliff/commit/6cec37d1ecb23507f0bf47cd7fe942368faca92d))

### 📚 Documentation

- *(readme)* Update `--with-commit` example in README.md - ([47d124a](https://github.com/orhun/git-cliff/commit/47d124add2669e541d992aba83759dc31cd9d18d))

### 🎨 Styling

- *(args)* [**breaking**] Rename `--commit-path` argument to `--include-path` - ([7b000ad](https://github.com/orhun/git-cliff/commit/7b000ad43ef5d25941057b38bb6747f9f1514b17))

### ⚙️ Miscellaneous Tasks

- *(config)* Indicate the breaking changes via default config - ([316c11b](https://github.com/orhun/git-cliff/commit/316c11b60756f8b38174433450d42f25919368b7))
- *(fixtures)* Run test fixtures on ubuntu-latest - ([dea65f2](https://github.com/orhun/git-cliff/commit/dea65f235e2091001d8de41794bf3c98a7223917))
- *(fixtures)* Improve the workflow for test fixtures - ([92a54d6](https://github.com/orhun/git-cliff/commit/92a54d67b825b53b6993a769ea9d5cf37ea2e43e))

## [0.4.2](https://github.com/orhun/git-cliff/compare/v0.4.1..v0.4.2) - 2021-10-22

### 🐛 Bug Fixes

- *(cd)* Install the Rust toolchain explicitly for crates.io releases - ([2cee3bf](https://github.com/orhun/git-cliff/commit/2cee3bf9ecc00e21b871e88a34a949fbca6b646b))

## [0.4.1](https://github.com/orhun/git-cliff/compare/v0.4.0..v0.4.1) - 2021-10-22

### 🐛 Bug Fixes

- *(changelog)* Add support for special characters in scopes ([#26](https://github.com/orhun/git-cliff/issues/26)) - ([10771f4](https://github.com/orhun/git-cliff/commit/10771f43c0f252dec9ad414b780bb22d866d00e2))

### 🚜 Refactor

- *(git)* Use a better error message for invalid repo path - ([f447cc2](https://github.com/orhun/git-cliff/commit/f447cc2e73ea707c2f4694507e9c7847fcff29e9))

### 📚 Documentation

- *(readme)* Update GitLab CI/CD section - ([2925340](https://github.com/orhun/git-cliff/commit/2925340368da5c74104f7c9befa47ee27f49c02d))
- *(readme)* Add GitLab CI/CD section to README.md ([#24](https://github.com/orhun/git-cliff/issues/24)) - ([90a87c5](https://github.com/orhun/git-cliff/commit/90a87c58741b3cdeee87a3c162cd10ddef59adaf))

### ⚙️ Miscellaneous Tasks

- *(ci)* Run CI workflows periodically - ([627d4ef](https://github.com/orhun/git-cliff/commit/627d4eff7f6e4da9ea942ac05c6743a4153f4cc2))
- *(docker)* Bump the Rust version in Dockerfile - ([fc33efd](https://github.com/orhun/git-cliff/commit/fc33efde85fb6c3a8c82c37e795b598706aea609))
- *(project)* Migrate to Rust 2021 edition - ([0000000](https://github.com/orhun/git-cliff/commit/0000000ef0e2d0710f4c1294408da2639f6f3217))
- *(project)* Remove unnecessary Cargo.lock entry from .gitignore - ([481713c](https://github.com/orhun/git-cliff/commit/481713cbb74fc2bce4a46ab6f8d4649b03d96fc2))

## [0.4.0](https://github.com/orhun/git-cliff/compare/v0.3.0..v0.4.0) - 2021-10-01

### ⛰️  Features

- *(changelog)* Add `--sort` argument for sorting commits ([#15](https://github.com/orhun/git-cliff/issues/15)) - ([2950a41](https://github.com/orhun/git-cliff/commit/2950a412c2aaa0d96609753129047cef39fd3e1c))

### 🐛 Bug Fixes

- *(ci)* Update lychee arguments to skip checking files - ([ba3f1ca](https://github.com/orhun/git-cliff/commit/ba3f1cac50338672c555581659e098e11796f466))
- *(config)* Remove only the leading "v" from tags ([#18](https://github.com/orhun/git-cliff/issues/18)) - ([e444615](https://github.com/orhun/git-cliff/commit/e444615c02749da5fc64ae3286bfde1b616e7271))
- *(docker)* Remove tags from the base image names - ([ece0481](https://github.com/orhun/git-cliff/commit/ece0481e73e63371dab87ec6a71da59999db7d47))

### 📚 Documentation

- *(config)* Add scope-sorted example ([#16](https://github.com/orhun/git-cliff/issues/16)) - ([05584b6](https://github.com/orhun/git-cliff/commit/05584b614aa593558674243cfbf14dafe7b6b8db))
- *(readme)* Add "build from source" section to README.md - ([b193f42](https://github.com/orhun/git-cliff/commit/b193f42e258e8fdee8b1b645d5a614d606f7e079))
- *(readme)* Mention the signing key for binary releases ([#17](https://github.com/orhun/git-cliff/issues/17)) - ([9022af5](https://github.com/orhun/git-cliff/commit/9022af533d3d967a09352a1bdf542f8ba97e9930))
- *(readme)* Add packaging status badge to installation section - ([5409e06](https://github.com/orhun/git-cliff/commit/5409e06e4cf3b6833ba3a9b1eb224014280069dd))
- *(readme)* Add raw/rendered output for scoped-sorted example - ([f64459d](https://github.com/orhun/git-cliff/commit/f64459dd9f6030697e51903f91136fa857332425))

### 🎨 Styling

- *(config)* Fix the newline issues in scoped-sorted example - ([428d407](https://github.com/orhun/git-cliff/commit/428d407df581dd00e26b320c3872eb21cfc8c803))

### ⚙️ Miscellaneous Tasks

- *(docker)* Use docker.yml workflow for CI/CD - ([7756266](https://github.com/orhun/git-cliff/commit/7756266b17f3c3ba55f5d00f8e55aea0a3a68ef2))
- *(docker)* Use explicit image name for docker automated builds - ([777375f](https://github.com/orhun/git-cliff/commit/777375f67703b2833509700273cb0bec9a659525))
- *(docker)* Specify the latest tag explicitly - ([6bafc5d](https://github.com/orhun/git-cliff/commit/6bafc5d1a124dea5423338afd75a5136944bebb5))
- *(docker)* Rename the GHCR package due to legacy reasons - ([71b8846](https://github.com/orhun/git-cliff/commit/71b88466ebd37b2bf5f0489afed117417ad20b24))
- *(docker)* Extend the tags for docker meta - ([dff2e62](https://github.com/orhun/git-cliff/commit/dff2e62edc3886346375a2451faabe5e8cf679f9))
- *(docker)* Use docker meta for tagging for GHCR - ([081b2d2](https://github.com/orhun/git-cliff/commit/081b2d257ba853c1d31d35ebfeae1cb92a641746))
- *(docker)* Use cache for docker builds - ([d3140ed](https://github.com/orhun/git-cliff/commit/d3140ed9882b6df8c07196c8f68bae67ab8da684))
- *(workflow)* Update the runner to ubuntu-20.04 - ([5069594](https://github.com/orhun/git-cliff/commit/5069594f1800e409665609224995b25dcb9df438))
- *(workflow)* Set a version for the checkout action - ([b323e60](https://github.com/orhun/git-cliff/commit/b323e60996595976fbe8261b5f8c4a9f67d2a8f8))

## [0.3.0](https://github.com/orhun/git-cliff/compare/v0.2.6..v0.3.0) - 2021-09-10

### ⛰️  Features

- *(changelog)* Support generating a changelog scoped to a directory ([#11](https://github.com/orhun/git-cliff/issues/11)) - ([0bb7c91](https://github.com/orhun/git-cliff/commit/0bb7c910b45436cbf69b444ccb29a530ede4aea0))
- *(changelog)* Support parsing the missing scopes with `default_scope` ([#8](https://github.com/orhun/git-cliff/issues/8)) - ([b5df656](https://github.com/orhun/git-cliff/commit/b5df656e61035f6230f2613f3dba6a92f88708cd))

### 🐛 Bug Fixes

- *(config)* Fix default regexes and references in docs ([#7](https://github.com/orhun/git-cliff/issues/7)) - ([8a18e4d](https://github.com/orhun/git-cliff/commit/8a18e4d48debbe522cefa9acd662bcc9a825c74e))

### 📚 Documentation

- *(config)* Update the default regex in scoped config example - ([0d793ad](https://github.com/orhun/git-cliff/commit/0d793ad9db43f0290ff6286f6aa1a618feb714ea))
- *(readme)* Update example regexes - ([f420a5a](https://github.com/orhun/git-cliff/commit/f420a5ac6cf56f3167cac23d40b43ec9aa370005))
- *(readme)* Add badge for joining the Matrix chat - ([b5edfc2](https://github.com/orhun/git-cliff/commit/b5edfc279d0290fecaacab469ecccdadf63eb3ab))
- *(readme)* Update installation instructions for Arch Linux - ([8fb18b7](https://github.com/orhun/git-cliff/commit/8fb18b784808222fdf3c4328ac9c871b93524fee))

## [0.2.6](https://github.com/orhun/git-cliff/compare/v0.2.5..v0.2.6) - 2021-09-04

### 🐛 Bug Fixes

- *(docker)* Pin the cargo-chef version in Dockerfile - ([af1851c](https://github.com/orhun/git-cliff/commit/af1851c0111d66ec3dd190baf7a456bcf44fdcdc))

### 📚 Documentation

- *(readme)* Update docker commands to only mount the .git directory - ([4398828](https://github.com/orhun/git-cliff/commit/4398828df7b3710550adee8ec09a34a59783265b))

### ⚙️ Miscellaneous Tasks

- *(docker)* Bump cargo-chef version in Dockerfile - ([612192b](https://github.com/orhun/git-cliff/commit/612192b3aa638be9ccd38ecda27bdee6b6ff6655))

## [0.2.5](https://github.com/orhun/git-cliff/compare/v0.2.4..v0.2.5) - 2021-08-20

### ⛰️  Features

- *(template)* Add `breaking_description` to the template context ([#4](https://github.com/orhun/git-cliff/issues/4)) - ([e0f6ca1](https://github.com/orhun/git-cliff/commit/e0f6ca151af1b0561cfcc4b757e430923f3d81b5))

### 📚 Documentation

- *(readme)* Update template examples to mention how to contribute - ([4c6e64b](https://github.com/orhun/git-cliff/commit/4c6e64bd13badf6bff55d608fcff4a06585f226c))
- *(readme)* Mention breaking changes for templating - ([6fa5d28](https://github.com/orhun/git-cliff/commit/6fa5d288d35c9f4218e33631b942bdd0cc381d3c))

### ⚙️ Miscellaneous Tasks

- *(release)* Show the committed changes before creating a tag - ([59ffe53](https://github.com/orhun/git-cliff/commit/59ffe53a7cb4791e4877a74f2e14d15139d2aca9))

## [0.2.4](https://github.com/orhun/git-cliff/compare/v0.2.3..v0.2.4) - 2021-08-20

### 🐛 Bug Fixes

- *(cd)* Change the config file location for crates.io release - ([a9b286c](https://github.com/orhun/git-cliff/commit/a9b286cf023148da0800c2a0408d87571c239847))

## [0.2.3](https://github.com/orhun/git-cliff/compare/v0.2.2..v0.2.3) - 2021-08-18

### 🐛 Bug Fixes

- *(cd)* Fetch the dependencies before copying the file to embed - ([9e29c95](https://github.com/orhun/git-cliff/commit/9e29c95319abd1747fbf6fd1e205d414617b0447))

## [0.2.2](https://github.com/orhun/git-cliff/compare/v0.2.1..v0.2.2) - 2021-08-18

### 🐛 Bug Fixes

- *(cd)* Copy the config file into registry to resolve it for embed - ([48ea157](https://github.com/orhun/git-cliff/commit/48ea1578b5ecc17f5cc9a4249fb7b38610028fc6))

## [0.2.1](https://github.com/orhun/git-cliff/compare/v0.2.0..v0.2.1) - 2021-08-18

### 🐛 Bug Fixes

- *(cd)* Copy the configuration file to embed into package - ([68dda36](https://github.com/orhun/git-cliff/commit/68dda364278870df84891495a3ff546ddbcae6a1))

## [0.2.0](https://github.com/orhun/git-cliff/compare/v0.1.2..v0.2.0) - 2021-08-18

### ⛰️  Features

- *(config)* Support a global location for configuration file ([#2](https://github.com/orhun/git-cliff/issues/2)) - ([210b634](https://github.com/orhun/git-cliff/commit/210b6341137016b902b20736f76c358e47d53c97))
- *(config)* Add `--init` flag for creating the default config - ([183481b](https://github.com/orhun/git-cliff/commit/183481bac374707fbb7c579e2df83296e27f7251))
- *(config)* Embed the default configuration file into the binary - ([e5148e3](https://github.com/orhun/git-cliff/commit/e5148e3ae1f6d459c0faeba6b6a78cf221f6f7ff))

### 🐛 Bug Fixes

- *(config)* Use custom error type for UTF-8 errors - ([45889c0](https://github.com/orhun/git-cliff/commit/45889c0a05ad64598d4e6b053bc0d90ff5449b51))

### 🚜 Refactor

- *(lib)* Update the log message for unprocessed tags - ([6f154ce](https://github.com/orhun/git-cliff/commit/6f154ce4eef33e5d36e07a6c78878eb01ebc024f))
- *(lib)* Create a constant for default configuration file - ([c478f2c](https://github.com/orhun/git-cliff/commit/c478f2c6167c53a814810eb65ab96084f020d928))

### 📚 Documentation

- *(changelog)* Update the doc comment of `prepend` - ([e7ae5a0](https://github.com/orhun/git-cliff/commit/e7ae5a062d3c446c6a0bc7d2784c52b63485259f))

### 🎨 Styling

- *(args)* Update the message of `--init` flag - ([927776b](https://github.com/orhun/git-cliff/commit/927776b9db06ff2de8555b57c862c4a9e4991469))

### ⚙️ Miscellaneous Tasks

- *(config)* Move `cliff.toml` to config/ - ([acda195](https://github.com/orhun/git-cliff/commit/acda1954dc192a3b706c21a48821f75e5a8e0d22))

## [0.1.2](https://github.com/orhun/git-cliff/compare/v0.1.1..v0.1.2) - 2021-08-14

### 🐛 Bug Fixes

- *(cd)* Use the correct name of completions binary - ([3ae64f4](https://github.com/orhun/git-cliff/commit/3ae64f4dd01f05a7896821c55642b8cac6e59bec))

### 📚 Documentation

- *(completions)* Update the example completion command - ([f1fd88a](https://github.com/orhun/git-cliff/commit/f1fd88af8f7fdfb021109ed9a24bd3d43045d534))

## [0.1.1](https://github.com/orhun/git-cliff/compare/v0.1.0..v0.1.1) - 2021-08-14

### 🐛 Bug Fixes

- *(changelog)* Set the previous release when using `--latest` ([#3](https://github.com/orhun/git-cliff/issues/3)) - ([29db41a](https://github.com/orhun/git-cliff/commit/29db41aa12332f14a421109fe8d6d09b549abd61))

### 📚 Documentation

- *(readme)* Add installation instructions for the AUR - ([f1b495d](https://github.com/orhun/git-cliff/commit/f1b495d7b1aeb016911150faa0d49f847cc7b17c))

### ⚡ Performance

- *(changelog)* Optimize the release vector size - ([7e84797](https://github.com/orhun/git-cliff/commit/7e84797900f1b6f61e16d8a4766b8209673a7acb))
- *(changelog)* Process only the last 'previous' release - ([f859747](https://github.com/orhun/git-cliff/commit/f85974761be11e0ecc85575bc4b6d5a02e438fd2))

### ⚙️ Miscellaneous Tasks

- *(project)* Rename the shell completions binary - ([718f535](https://github.com/orhun/git-cliff/commit/718f53573b9f48a60ad1930cd9555063414f8b96))

## [0.1.0] - 2021-08-12

### ⛰️  Features

- *(args)* Add `--output` argument - ([e0cda23](https://github.com/orhun/git-cliff/commit/e0cda238105e0ce22ac71409d6e3ee0e32a6bad7))
- *(args)* Add `--workdir` argument - ([de439be](https://github.com/orhun/git-cliff/commit/de439bef01e0e1209b8517ce7b241bb0db2cb530))
- *(changelog)* Support setting the body template via args - ([9fc08f4](https://github.com/orhun/git-cliff/commit/9fc08f4c50160374298bf77701ee5e299fa435a0))
- *(logs)* Show the processsed commit message - ([45dccf7](https://github.com/orhun/git-cliff/commit/45dccf74a4a5449d92d453d21b6566acd8f30ebe))

### 🐛 Bug Fixes

- *(cd)* Wait for core library to update on crates.io before publish - ([e795460](https://github.com/orhun/git-cliff/commit/e795460c9bb7275294d1fa53a9d73258fb51eb10))
- *(cd)* Wait between publishing crates - ([777b3e5](https://github.com/orhun/git-cliff/commit/777b3e573f20e913a68293eb155076d52914b4d4))
- *(cd)* Generate changelog on a dedicated/different job - ([2f16dd5](https://github.com/orhun/git-cliff/commit/2f16dd5ae7f4bbff7b9944db039192d8ce148655))
- *(cd)* Fix the syntax of publish step arguments - ([6414789](https://github.com/orhun/git-cliff/commit/6414789067780d0551292c004a8aaff04483906d))
- *(cd)* Use a separate step for setting the changelog body - ([f038054](https://github.com/orhun/git-cliff/commit/f038054417f608b5792b89f60b4d384b74b317dd))
- *(cd)* Publish the cargo workspace members seperately - ([acc1d34](https://github.com/orhun/git-cliff/commit/acc1d3453ca865ddbf3e8e786fcc157c8e31eae6))
- *(cd)* Strip the changelog header before escaping - ([6b97c2d](https://github.com/orhun/git-cliff/commit/6b97c2d18c05fb9cf27088a39b24d12ab1b0c556))
- *(cd)* Use printf to prevent field splitting the variable - ([92a4b0d](https://github.com/orhun/git-cliff/commit/92a4b0d5bfb8c27627886005d0e6d823a3cc6476))
- *(cd)* Double quote the environment variable - ([85aa1cd](https://github.com/orhun/git-cliff/commit/85aa1cdc688b74d1d2df46dc61e6aa5561fc6ace))
- *(changelog)* Return error if there is not a latest tag to process - ([8232111](https://github.com/orhun/git-cliff/commit/8232111aa1fab0089fdb3b025202729b59dcf906))
- *(changelog)* Use footers field as an array for the context - ([3e5c23d](https://github.com/orhun/git-cliff/commit/3e5c23d342ef96bececf41254cb609025a012fb3))
- *(ci)* Update lychee arguments to exclude invalid links - ([e1d604c](https://github.com/orhun/git-cliff/commit/e1d604cbdf049f30c33380c591bfedf0baa3f0a7))
- *(config)* Update the environment variable parsing settings - ([9984abf](https://github.com/orhun/git-cliff/commit/9984abfe04e6bed0745b52701ce3f45dd92529f6))
- *(config)* Update config to skip release commits - ([2e5f30a](https://github.com/orhun/git-cliff/commit/2e5f30a5c3b920dc2b52fe12774fccb2fb3fd124))
- *(config)* Update commit parsers to match the commit type - ([24b9068](https://github.com/orhun/git-cliff/commit/24b9068d308b9818fab8fd631120a79e4069521d))
- *(example)* Remove symbolic link - ([ed010be](https://github.com/orhun/git-cliff/commit/ed010be74fb745a9147315bcbbe0f9ac0aae85fb))
- *(example)* Update symbolic link to the default config - ([ee377cc](https://github.com/orhun/git-cliff/commit/ee377cc42d2d8771d67b0907151b1bb2ee012d69))
- *(git)* Sort the commits in topological order - ([a1b4b5b](https://github.com/orhun/git-cliff/commit/a1b4b5b18a6061392fc27b66a5726824212c114c))
- *(git)* Return tags by their creation order - ([000a67c](https://github.com/orhun/git-cliff/commit/000a67cd8aae7ae20848baa04cd6212376dcde12))
- *(release)* Fix the character escape in release script - ([5d616ee](https://github.com/orhun/git-cliff/commit/5d616ee249aaee9f38d6d9b7a9e14170b9b7405a))
- *(release)* Specify the committer email in release script - ([381c941](https://github.com/orhun/git-cliff/commit/381c941a94188fb40bcce1129c29e6a5379ac7a7))
- *(release)* Strip the unreleased title from tag message - ([c7f08fe](https://github.com/orhun/git-cliff/commit/c7f08fe4a3113f656bb45a29141aa127d4e453e5))
- *(template)* Use 7 digits for short SHA - ([1f85263](https://github.com/orhun/git-cliff/commit/1f85263f84bf15285fd3fd75af00fa21b12e9738))
- *(test)* Use default tag_pattern for tests - ([e6fb8de](https://github.com/orhun/git-cliff/commit/e6fb8de5d834eb5fe5ff3b5fab1986fcf5e720b6))

### 🚜 Refactor

- *(args)* Rename changelog argument to prepend - ([5a5a042](https://github.com/orhun/git-cliff/commit/5a5a042134c4829d98271607f697f77701f80860))
- *(args)* Update value names and description - ([c697b17](https://github.com/orhun/git-cliff/commit/c697b17f67c9438a1fb159db74b1e225aeb28853))
- *(args)* Update the value name for `--strip` - ([e8a3c35](https://github.com/orhun/git-cliff/commit/e8a3c35fa1f2a9a1a99ad8c60b82899c2af212eb))
- *(config)* Make tag_pattern optional - ([3a27a3e](https://github.com/orhun/git-cliff/commit/3a27a3e1a2d3813def0405ddf89914c5f10f7b36))
- *(error)* Use custom error message for GroupError - ([1be66ee](https://github.com/orhun/git-cliff/commit/1be66eebebaa3a5849399433ad5823345668ffb1))
- *(logs)* Improve logging - ([5333453](https://github.com/orhun/git-cliff/commit/53334533eb6399dc4b931fa3f61c32b5e28bd9e7))

### 📚 Documentation

- *(bin)* Update the doc comment for completions script - ([c056196](https://github.com/orhun/git-cliff/commit/c056196af9ee3f7109876639a0fb3b230661e1e4))
- *(contributing)* Add CONTRIBUTING.md - ([0fe28b3](https://github.com/orhun/git-cliff/commit/0fe28b322e4dd83bef1ba39d1028922065aa8aee))
- *(readme)* Add preview image to README.md - ([6e6bdde](https://github.com/orhun/git-cliff/commit/6e6bddeadd3ee43ac6bb626ce5ebd582ffd1f7cb))
- *(readme)* Update detailed template example - ([441ed4d](https://github.com/orhun/git-cliff/commit/441ed4d8b499a46ce22800da3d969ea6165d6ddf))
- *(readme)* Add examples for templating - ([a406158](https://github.com/orhun/git-cliff/commit/a40615860871d4bbb351ae1571192a624b1b539d))
- *(readme)* Add examples for CLI usage - ([32f837e](https://github.com/orhun/git-cliff/commit/32f837e3f0d642f5dc16f1e144ef5040652173ec))
- *(readme)* Update README.md about template and examples - ([2f2b2fc](https://github.com/orhun/git-cliff/commit/2f2b2fc50a5aa4cdd7842448b9fde7f73564f5d7))
- *(readme)* Update README.md about usage - ([7f4a9c2](https://github.com/orhun/git-cliff/commit/7f4a9c20128af75a8972e58130ac0ed4ce52d995))
- *(readme)* Add usage section - ([c87fbbe](https://github.com/orhun/git-cliff/commit/c87fbbe7cf869dd343269f6f23d9e4776a7d952b))
- *(release)* Add RELEASE.md - ([ce2246b](https://github.com/orhun/git-cliff/commit/ce2246bcfdcbce2e2ef30ec44fbfae26d3914139))
- *(release)* Add link to the signer key of the tag - ([59b7c6e](https://github.com/orhun/git-cliff/commit/59b7c6ef79c6377b3de26247a232412c765ab6a8))

### 🎨 Styling

- *(config)* Update the order of entries in config - ([9f84fe5](https://github.com/orhun/git-cliff/commit/9f84fe58a76c4aed97844e648bf42ad0c1d46303))
- *(readme)* Remove quotes from rendered output - ([dfab978](https://github.com/orhun/git-cliff/commit/dfab97842fe78fb8f217a5185fb6ec92682a53ca))
- *(readme)* Wrap table of contents into summary - ([fa6a38b](https://github.com/orhun/git-cliff/commit/fa6a38b339d7a56e976820b363f48d5c13d5cdf5))
- *(readme)* Remove comments from template context - ([1cf1ac7](https://github.com/orhun/git-cliff/commit/1cf1ac73fa26d015f7316c7c27639c1c7f4a7995))
- *(readme)* Update the comments in template context - ([3749490](https://github.com/orhun/git-cliff/commit/37494901ef4826b57f82946a6cd6c7cd21f5ca2c))
- *(readme)* Center the badges - ([f08ff0c](https://github.com/orhun/git-cliff/commit/f08ff0c4136a2257bfe7e586c03bcf7a520f06bd))

### 🧪 Testing

- *(config)* Add tests - ([11a7a7e](https://github.com/orhun/git-cliff/commit/11a7a7eb3eec4e801b6ab6576db2849bc771987f))
- *(git)* Update repository tests about getting the latest tag - ([9cf9ac1](https://github.com/orhun/git-cliff/commit/9cf9ac1586cf0f582b9a48bc5ce6f351d8350721))

### ⚙️ Miscellaneous Tasks

- *(cargo)* Update project details - ([22f0a7e](https://github.com/orhun/git-cliff/commit/22f0a7ef73cb1649d9ed59e43ee0e410b456233d))
- *(cd)* Enable crates.io releases - ([31ecfd8](https://github.com/orhun/git-cliff/commit/31ecfd8ad041e36090575e5851ff00d491ccebca))
- *(cd)* Use only one step for uploading releases - ([42a714f](https://github.com/orhun/git-cliff/commit/42a714f31cf5b6f924fe68d966189e2c278a11a6))
- *(cd)* Use seperate steps for uploading releases - ([0182533](https://github.com/orhun/git-cliff/commit/01825330d90a9399c9285b5a286b1d69aa1494e7))
- *(cd)* Remove the custom changelog template - ([d826b9d](https://github.com/orhun/git-cliff/commit/d826b9d2e5bdd30b132731ff6d1dc87748543ccb))
- *(cd)* Override the changelog template - ([41053fb](https://github.com/orhun/git-cliff/commit/41053fbe90a08648b70e5cca6a2504e94202bd06))
- *(cd)* Set the release body on linux - ([7623977](https://github.com/orhun/git-cliff/commit/76239771662bf342d742c12907619eece1bf946d))
- *(cd)* Fix setting the release body - ([62403be](https://github.com/orhun/git-cliff/commit/62403be828ac75f5689f27ad5d5e0421b514be62))
- *(cd)* Set the changelog as release body - ([66dfbf4](https://github.com/orhun/git-cliff/commit/66dfbf40ec0ef91dc2016c1bdf2044a07e4580d5))
- *(cd)* Set the release name explicitly - ([0e5a7b1](https://github.com/orhun/git-cliff/commit/0e5a7b1dd54afff0085930bf8c058803cfe3ea03))
- *(config)* Update template to include commit ids - ([e3d3482](https://github.com/orhun/git-cliff/commit/e3d34821911ffb44e4c6e61e8d0c6ecd2d31a341))
- *(config)* Update the skip_tags regex - ([28171f0](https://github.com/orhun/git-cliff/commit/28171f0f98f0980f9b0a6d3ca89ed3d6c70eb0ef))
- *(docker)* Bump the rust version - ([a2df26f](https://github.com/orhun/git-cliff/commit/a2df26ff8602095707ebcfcfc4c278d12a9463f4))
- *(docker)* Rename the docker automated builds action - ([ad3126e](https://github.com/orhun/git-cliff/commit/ad3126e15144f98b5845c31149214cdce5ffb3d5))
- *(docker)* Remove user directive from Dockerfile - ([076fc85](https://github.com/orhun/git-cliff/commit/076fc8580aee74755003d99d902b3e64abcad535))
- *(git)* Remove etc directory from .gitignore - ([8881083](https://github.com/orhun/git-cliff/commit/8881083520e5fcdbdfa27520f0a1aa29752067cc))
- *(project)* Update .editorconfig about shell scripts - ([c898912](https://github.com/orhun/git-cliff/commit/c8989122d06af2abb70edf5d42d7297411d0b093))
- *(project)* Update the release script about arguments - ([098c6ad](https://github.com/orhun/git-cliff/commit/098c6ad3803d728f4cd25320cb557d2a0bb22bd1))
- *(project)* Add release script - ([d76bb9b](https://github.com/orhun/git-cliff/commit/d76bb9b3e5ff41df96b27c4fb202a2dd2344d6b2))
- *(release)* Indicate which versions are managed by the script - ([f481081](https://github.com/orhun/git-cliff/commit/f48108109e84ea6d869c75e98143be9b7ed5d069))
- *(release)* Verify the created tag after creation - ([99f2f07](https://github.com/orhun/git-cliff/commit/99f2f0701cbf92c5b4ec235e307549af53336db7))
- *(release)* Set the new version in release script - ([6619c38](https://github.com/orhun/git-cliff/commit/6619c385ed5220293b4a9af1c405a364f0085bc9))
- *(release)* Include the commit id in the custom template - ([5677281](https://github.com/orhun/git-cliff/commit/5677281b2ca594789a32c53e7f304cae734c089f))
- *(release)* Set a custom changelog for the tag message - ([c4df0fa](https://github.com/orhun/git-cliff/commit/c4df0fa892568f1491b62c47d5757fb70b7f4316))
- *(release)* Add release title to the tag message - ([6099b2d](https://github.com/orhun/git-cliff/commit/6099b2d0c7c7d51a44a0e05a53908a9a370a7e83))
- *(release)* Strip the markdown format from tag message - ([3cb6761](https://github.com/orhun/git-cliff/commit/3cb67618ef475bf0eb144c5b84cd68af0758f564))

### ◀️ Revert

- Chore(config): update template to include commit ids - ([f95fca9](https://github.com/orhun/git-cliff/commit/f95fca966bacb520e958fe783e239f98dfe026bc))

<!-- generated by git-cliff -->
