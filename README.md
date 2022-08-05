# ReportPortal APIs

This repository contains gRPC protocol definitions for ReportPortal APIs and provides knowledge
about its own services. You can use Protocol Buffers (Protobuf) descriptions to generate your
client's libraries or take pre-generated.

## Contents

- [ReportPortal APIs](#reportportal-apis)
  - [Contents](#contents)
  - [Overview](#overview)
  - [How to use](#how-to-use)
    - [Buf CLI](#buf-cli)
    - [Verify](#verify)
    - [Lint API](#lint-api)
    - [Detect breaking changes](#detect-breaking-changes)
    - [Generate code](#generate-code)
  - [Repository Structure](#repository-structure)
  - [Third-party libraries](#third-party-libraries)
    - [Google types](#google-types)
      - [Additional types](#additional-types)
  - [Contribution](#contribution)

## Overview

ReportPortal APIs are deployed as the API Service.

Some ReportPortal APIs use Protobuf version 3 (proto3) as their Interface Definition
Language (IDL) to define the API interface and the structure of the payload messages.

## How to use

### Buf CLI

We use `buf` CLI to create consistent Protobuf APIs that preserve compatibility and comply with best practices.

To work with this repository, [install buf](https://docs.buf.build/installation).

### Verify

The `buf build` command is used to verify that an input compiles.

```bash
buf build --exclude-source-info -o -#format=json | jq '.file[] | .package'
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

### Generate code

Buf doesn't have a built-in compiler. Before, you need to
[install The protocol buffer compiler](https://grpc.io/docs/protoc-installation/) or use
[remote generation for Go and JS/TS](https://docs.buf.build/bsr/remote-generation/overview).

```bash
buf generate
```

## Repository Structure

```tree
├── apis
│   ├── buf.md
│   ├── buf.yaml
│   ├── common
│   │    └── types
│   │       ├── v1
│   │       │   ├── some_types.proto
│   │       │   └──...
│   │       └── ...
│   └── reporting
│       ├── services
│       │   ├── v1
│       │   │   ├── some_service.proto
│       │   │   └── ...
│       │   └── ...
│       └── types
│           ├── v1
│           │   ├── some_types.proto
│           │   └──...
│           └── ...
└── vendors
    └── vendor_name

```

- apis - root catalog for Report Portal Interface Definitions
- common - contains common types for all Report Portal services
- domain - contains definitions for particular Report Portal services and types
- vendors - catalog for third-party local definitions. Use to extend Report Portal APIs
- v`N` - catalog for grouping definitions by versions
- services - contains service definitions calls
- types - contains definitions for messages and enumerations
- [buf.md](https://docs.buf.build/bsr/documentation) - it's analogous to a GitHub repository's
README.md and supports the CommonMark syntax
- [buf.yaml](https://docs.buf.build/configuration/v1/buf-yaml) - defines a module, and is placed at
the root of the Protobuf source files it defines

## Third-party libraries

### Google types

We use additional types provided by
[Google Type Library](https://buf.build/googleapis/googleapis/docs/main:google.type).

You can use them in your proto files like:

```proto
import "google/type/<type_name>.proto";
```

#### Additional types

- CalendarPeriod (enum)
- Color (message)
- Date (message)
- DateTime (message)
- DayOfWeek (enum)
- Decimal (message)
- Expr (message)
- Fraction (message)
- Interval (message)
- LatLng (message)
- LocalizedText (message)
- Money (message)
- Month (enum)
- PhoneNumber (message)
- ShortCode (message)
- PostalAddress (message)
- Quaternion (message)
- TimeOfDay (message)
- TimeZone (message)

## Contribution

- Use official Protocol Buffers [Style Guide](https://developers.google.com/protocol-buffers/docs/style)
- Read the [Contribution details](https://github.com/reportportal/reportportal/wiki/Contribution)
- Use `buf lint` for check style guid requirements and `buf breaking --against '.git#branch=main'`
for checking forwards and backward compatibility
