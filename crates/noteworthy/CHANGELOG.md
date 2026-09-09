# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.12](https://github.com/esmevane/oneiros/compare/noteworthy-v0.0.11...noteworthy-v0.0.12) - 2026-09-09

### 🐛 Fixes

- [patch] Noteworthy with defaults. (#306)

I didn't think about how annoying it was going to be to work with
structs as annotations without defaults, to be honest. But it got
tiring quick! This commit introduces the concept of defaults to
annotation structs, so that we don't have to get ultra verbose and,
in some cases, they look and feel like derives.

