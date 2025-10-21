# Repository Guidelines

## Project Structure & Module Organization

Schemas live under `api/openapi` (OpenAPI YAML) and `api/proto` (protobuf definitions grouped by domain and version). Generated stubs land in `src/build` and are ignored from reviews unless explicitly updated. Gradle wrapper files sit in `gradle/`, Buf templates in `templates/`, and contributor docs in `docs/`. Keep shared types in `api/proto/reportportal/common`; create new domains below `api/proto/reportportal/<area>/vN`.

## Build, Test, and Development Commands

Use `buf lint` to enforce protobuf conventions, and `buf breaking --against '.git#branch=main'` before pushing to guard compatibility. Run `buf build --exclude-source-info -o -#format=json | jq '.file[] | .package'` to confirm packages compile. Generate Java stubs with `./gradlew generateProto`; clean artifacts via `./gradlew clean`. For OpenAPI work, `make lint` checks a single spec, `make lint-all` covers `api/openapi/apis/*.yaml`, and `make bundle` assembles distributable bundles into `build/openapi`.

## Coding Style & Naming Conventions

Follow the official Protocol Buffers style: `UpperCamelCase` for messages, `snake_case` for fields, and prefix enums with a shared namespace (`LOG_TYPE_*`). YAML specs use two-space indentation and kebab-case file names in `api/openapi/apis`. Keep component IDs stable and reuse existing schemas when possible. Run Redocly bundling commands after touching shared fragments to keep generated headers in sync.

## Testing Guidelines

Treat `buf lint`, `buf breaking`, and `buf build` as the pre-commit test suite for protobuf changes. OpenAPI updates must pass `make lint` (or `lint-all` when multiple files move) and, if you touch the combined spec, re-run `make join` followed by `make bundle`. Place any manual validation notes alongside the edited spec in a PR comment. No automated coverage targets exist, but keep regenerated outputs checked in only when relevant to the change.

## Commit & Pull Request Guidelines

Commit messages in this repo are short, imperative statements without trailing periods (e.g., `Update log type schemas`). Group related schema edits together and avoid mixing proto and OpenAPI changes without context. PRs should describe the affected service or endpoint, list generated artifacts, and mention the commands you ran (`buf breaking`, `make lint`, etc.). Link to tracking issues when applicable and include screenshots only when documenting docs-site changes.
