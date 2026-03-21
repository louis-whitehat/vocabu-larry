# Copilot Instructions

This repository favors simple, explicit code over unnecessary abstractions.

## Architecture

- Organize backend code by feature first, not by technical layer (vertical slicing)
- Keep feature modules aligned with product concepts such as `login`, `training`, `scores`, `logs`, and `users`.
- Keep `main.rs` thin. Put startup and routing there, but keep feature behavior in feature modules.
- Keep shared code small and justified. Add shared modules only for clearly reused concerns
  such as config, state, logging, validation, or error handling.

## Frontend Architecture

- Keep Yew views thin. Prefer moving behavior and state transitions into view-model modules so the view mostly renders and wires events.
- It is acceptable for a view-model to own endpoint paths, request shapes, and backend-call logic when that removes logic from the view and improves testability.
- Do not add small wrapper helpers that only hide a single obvious call or value. Favor direct, explicit code.
- Prefer sharing logic through focused view-model modules instead of duplicating behavior between UI code and acceptance tests.

## Frontend HTTP And Errors

- Prefer one HTTP client strategy across browser and native test code when practical. Avoid maintaining parallel transport implementations without a clear need.
- When the backend returns plain-text error bodies, surface that message directly. Do not report misleading JSON decode errors for non-JSON error responses.
- Be careful with browser-restored form state. When a screen should start unselected, represent that explicitly and ensure the rendered controls reflect it.

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
- Do not add unit tests inside product source files under `src/`. Keep backend test coverage in `src/WebApi/tests/` unless a different location is explicitly requested.
- Use temporary directories and files for backend tests.
- Add smoke tests for core workflows before switching infrastructure or backend implementations.
- Focus tests on real behavior: login, dictionary/training retrieval, score updates, logs, and path validation.
- For acceptance coverage, prefer browser-free Rust tests that exercise the real backend over HTTP and reuse frontend view-model logic where useful.
- Avoid browser automation for acceptance tests unless the behavior is specifically browser-dependent.

## Change Discipline

- Keep changes small and focused.
- Do not refactor unrelated code while working on a feature.
- If a naming cleanup or architecture improvement would change the public contract, stop and verify that the change is actually desired.
- Prefer compatibility and clarity over elegance.
