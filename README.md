# Audacity

Audacity is a SigmaOS component providing a functional domain model for `audacity2` with validated state, deterministic storage, and typed errors.

## Description

Audacity audio editor compatibility layer for SigmaOS

## Domain model

- `AudioEditor` — domain entity

## API

- `new()`

## Testing

```sh
cargo test
```

Tests are deterministic, isolated, and self-validating.
