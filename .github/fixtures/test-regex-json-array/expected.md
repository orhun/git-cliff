# Changelog

All notable changes to this project will be documented in this file.

## [3.0.0] - 2025-01-25

### <!-- 5 -->🌀 Miscellaneous

- Drop support for .NET 6.0.
- Merge pull request #27 from mta-solutions/drop-support-for-dotnet-6

Drop Support for .NET 6.0

## [2.2.0] - 2024-12-13

### <!-- 1 -->🛠️ Enhancements

- Merge pull request #24 from mta-solutions/additional-vctx-functions

Add additional VCtx functions

### <!-- 5 -->🌀 Miscellaneous

- Add combine function for VCtx
- Add async versions of combine, bindAndCombine, and map for VCtx
- Update README files
- Address PR comments
- Create additional versions of merging and binding functions
- Update documentation and unit tests
- Add unit tests for FSharp.Data.Validation.Async
- Address package vulnerabilities
- Update CHANGELOG
- Merge pull request #25 from mta-solutions/prep-v2.2.0-release

Update CHANGELOG for version 2.2.0
- Update CHANGELOG
- Fix CHANGELOG revision history
- Merge pull request #26 from mta-solutions/prep-giraffe-v2.0.1-release

Prep FSharp.Data.Validation.Giraffe v2.0.1 Release

## [2.1.0] - 2024-12-10

### <!-- 1 -->🛠️ Enhancements

- Merge pull request #22 from mta-solutions/async-extensions

Remove internal scoping and add Async extension methods for validation contexts

### <!-- 4 -->📝 Documentation

- Merge pull request #23 from mta-solutions/prep-v2.1.0-release

Prep v2.1.0 Release

### <!-- 5 -->🌀 Miscellaneous

- Remove internal scoping and add Async extension methods for validation contexts
- Add asynchronous validation section to the README
- Add FSharp.Data.Validation.Async to release workflow
- Update CHANGELOG

## [2.0.0] - 2024-10-08

### <!-- 1 -->🛠️ Enhancements

- Merge pull request #20 from mta-solutions/update-to-use-fsharp-plus

Use FSharpPlus

### <!-- 4 -->📝 Documentation

- Merge pull request #21 from mta-solutions/prep-for-realease-v2.0.0

Prep for Release v2.0.0

### <!-- 5 -->🌀 Miscellaneous

- Use FSharpPlus.
- Prep for v2.0.0 release.
- Update pipelines.

## [giraffe-v1.0.0] - 2023-11-17

### <!-- 5 -->🌀 Miscellaneous

- Fix pipeline.
- Merge pull request #19 from alasconnect/fix-giraffe-pipeline

Fix Giraffe Pipeline

## [1.0.1] - 2023-11-17

### <!-- 1 -->🛠️ Enhancements

- Merge pull request #17 from alasconnect/fsharp-model-validation-giraffe

Add Giraffe model validation library and examples
- Merge pull request #18 from alasconnect/convert-to-nuget-cpm

Convert to NuGet

### <!-- 5 -->🌀 Miscellaneous

- Add Giraffe model validation library and examples
- Convert to NuGet and fix package references.
- Fix build props.
- Update changelogs.
- Remove broken package.

## [1.0.0] - 2022-03-14

### <!-- 1 -->🛠️ Enhancements

- Merge pull request #14 from alasconnect/fsdv-nonemptylist-utils

Add utility functions to help with using NonEmptyLists [MBI-53]

### <!-- 5 -->🌀 Miscellaneous

- Initial commit.
- Added initial unit tests, prototyping FsCheck
Simplified implementation of catOptions
Linting cleanup
- Added testing github action
- Removed package references that are handled by paket
- Merge pull request #1 from alasconnect/initial-unit-tests

[MBI-33] Initial unit tests
- Restructured modules for Proof, ValueCtx, VCtx
Finished Proof tests
- Merge pull request #2 from alasconnect/restructure-base-modules

[MBI-33] Restructured modules for Proof, ValueCtx, VCtx
- Add custom serializer for Proof type.
- Merge pull request #3 from alasconnect/proof-serialization

[MBI-33]: Add Custom Serializer for Proof Type
- Broke projects up into src/tests/samples
Renamed Proof.bind to Proof.map
Added Proof.mapInvalid
Fixed bug in greater/less than operators
- Added tests for ValueCtx
Added tests for VCTx
Added tests for Library (incomplete)
Added integration/fixture tests
- Fix optional.
- Uncommented Delay and Run
Added more fixture/integraiton tests
- Cleaned up comments
- Merge pull request #4 from alasconnect/unit-testing-mvp

[MBI-33] Broke out samples/src/tests and expanded unit testing
- Add raiseIfInvalid function.
- Merge pull request #5 from alasconnect/add-raise-if-invalid

Add raiseIfInvalid Function
- Finished Proof and Library test coverage
Ensured expect values always come first in unit tests
Fixed bugs and simplified maps in Library
Made InvalidProofException parameters explicit
Automated formatting cleanup of whitespace
Added FsUnit for experimentation
- Merge pull request #6 from alasconnect/finish-library-tests

[MBI-33] Finished Proof and Library test coverage
- Renamed root namespace to FSharp.Data.Validation
Added README stub
- Merge pull request #7 from alasconnect/rename-fsharp-data-validation

[MBI-33] Renamed root namespace to FSharp.Data.Validation
- Added tests for the interesting parts of VCtxBuilder
Simplified VCtxBuilder.Bind implementation
Added editorconfig
- Merge pull request #8 from alasconnect/vctxbuilder-tests

[MBI-33] Added tests for the most interesting parts of VCtxBuilder
- Fixed random test failures
When random fields names were the same, test case of Map.fromList was dumping values
- Merge pull request #9 from alasconnect/fix-random-test-failures

Fixed random test failures
- Rename solution file.
- Merge pull request #10 from alasconnect/rename-solution

Rename Solution File
- Add primitive type validation documentation.
- Add more documentation and fix collection validation functions.
- Refactor builder methods.
- Refactor primitive builder functions.
- Update readme.
- Fix function signiture.
- Refactor complex builders to match primitive builders.
- Fix grammar and table of contents.
- Merge pull request #11 from alasconnect/documentation

Add Documentation
- Add more documentation and NonEmptyList type.
- Minor documentation adjustments.
- Fix documentation issues.
- Merge pull request #12 from alasconnect/more-documentation

[MBI-47]: Add Documentation and NonEmptyList Type
- Add tests for DisputeWith, DisputeWithFact, RefuteWith, and RefuteWithProof
- Merge pull request #13 from alasconnect/fsdv-ml-first-tests

Add tests for DisputeWith, DisputeWithFact, RefuteWith, and RefuteWithProof [MBI-46]
- Adjust serialization.
- Wire up release action.
- Merge pull request #15 from alasconnect/wire-up-release-actions

[MBI-48]: Wire Up Release Actions
- Add utility functions to help with using NonEmptyLists
- Update change log and readme.

<!-- generated by git-cliff -->