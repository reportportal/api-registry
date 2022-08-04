# ReportPortal APIs

This repository contains gRPC protocol definitions for ReportPortal APIs and provides knowledge
about its own services. You can use Protocol Buffers descriptions to generate your client's
libraries or take pre-generated.

## Overview

ReportPortal APIs are deployed as the API Service.

Some ReportPortal APIs use Protocol Buffers version 3 (proto3) as their Interface Definition
Language (IDL) to define the API interface and the structure of the payload messages.

## Usage

### Buf CLI

We use `buf` CLI to create consistent Protobuf APIs that preserve compatibility and comply with best practices.

To work with this repository, [install buf](https://docs.buf.build/installation).

### Verify

The `buf build` command is used to verify that an input compiles.

```bash
buf build --exclude-source-info -o -#format=json | jq '.file[] | .package'
```

### Generate code

Buf doesn't have built-in compiler. Before, you need to [install The protocol buffer compiler](https://grpc.io/docs/protoc-installation/).

```bash
buf generate
```

### Lint API

Just use

```bash
buf lint
```

### Detect breaking changes

```bash
buf breaking --against '.git#branch=main'
```

## Repository Structure

```tree
├── apis
│   └── reportportal
│       ├── common
│       │    └── types
│       │       ├── v1
│       │       │   ├── some_types.proto
│       │       │   └──...
│       │       └── ...
│       └── reporting
│           ├── services
│           │   ├── v1
│           │   │   ├── some_service.proto
│           │   │   └── ...
│           │   └── ...
│           └── types
│               ├── v1
│               │   ├── some_types.proto
│               │   └──...
│               └── ...
└── vendors
    └── google
        └── type
            └── google_types.proto
```

* apis - root catalog for Report Portal services
* common - contains common types for all Report Portal services
* domain - contains definitions for particular Report Portal services and types
* vendor - catalog for third-party definitions. Use to extend Report Portal APIs
* v`N` - catalog for grouping definitions by versions
* services - contains service definitions calls
* types - contains definitions for messages and enumerations

## Contribution

* Use official Protocol Buffers [Style Guide](https://developers.google.com/protocol-buffers/docs/style)
* Read the [Contribution details](https://github.com/reportportal/reportportal/wiki/Contribution)
* Use `buf lint` for check style guid requirements and `buf breaking --against '.git#branch=main'`
for checking forwards and backward compatibility
