# Copilot Instructions

This repository favors simple, explicit code over unnecessary abstractions.

## Architecture

- Organize backend code by feature first, not by technical layer (vertical slicing)
- Keep feature modules aligned with product concepts such as `login`, `training`, `scores`, `logs`, and `users`.
- Keep `main.rs` thin. Put startup and routing there, but keep feature behavior in feature modules.
- Keep shared code small and justified. Add shared modules only for clearly reused concerns
  such as config, state, logging, validation, or error handling.

## Rust Style

- Prefer plain Rust and the standard library when they are sufficient.
- Only add dependencies when they provide clear value.
- Avoid tiny abstractions that hide one or two lines of logic without improving readability.
- Prefer direct code over helper functions when the helper does not capture meaningful domain logic.
- Prefer concrete, explicit code over generic frameworks or clever patterns.
- Keep error handling simple. Do not introduce many special error variants unless the software really handles them differently.
- Validate filesystem path segments before using user-controlled values in paths.

## Tests

- Prefer lightweight integration tests over heavy mocking.
- For backend tests, prefer building the Axum router directly and testing it without network sockets.
- Use temporary directories and files for backend tests.
- Add smoke tests for core workflows before switching infrastructure or backend implementations.
- Focus tests on real behavior: login, dictionary/training retrieval, score updates, logs, and path validation.

## Change Discipline

- Keep changes small and focused.
- Do not refactor unrelated code while working on a feature.
- If a naming cleanup or architecture improvement would change the public contract, stop and verify that the change is actually desired.
- Prefer compatibility and clarity over elegance.
