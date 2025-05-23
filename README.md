# ReportPortal API Definitions

https://reportportal.io/api-registry

This repository contains gRPC protocol definitions and OpenAPI Specification for ReportPortal APIs
and provides knowledge about its own services.
You can use Protocol Buffers (Protobuf) descriptions or OpenAPI specifications to generate your
client's libraries or take pre-generated.

## Contents

- [ReportPortal API Definitions](#reportportal-api-definitions)
  - [Contents](#contents)
  - [Overview](#overview)
  - [Repository Structure](#repository-structure)
  - [How to work with Protobuf](#how-to-work-with-protobuf)
    - [Buf CLI](#buf-cli)
    - [Verify](#verify)
    - [Lint API](#lint-api)
    - [Detect breaking changes](#detect-breaking-changes)
  - [Generate code](#generate-code)
    - [Java](#java)
    - [Other languages](#other-languages)
      - [Remote generating](#remote-generating)
      - [Local generating](#local-generating)
  - [Third-party libraries](#third-party-libraries)
    - [Google types](#google-types)
      - [Additional types](#additional-types)
  - [Contribution](#contribution)

## Overview

ReportPortal APIs are deployed as the
[API Service (ext. link)](https://github.com/reportportal/service-api).

Main ReportPortal APIs use OpenAPI Specification (OAS) to describe their services.

Some ReportPortal APIs use Protobuf version 3 (proto3) as their Interface Definition
Language (IDL) to define the API interface and the structure of the payload messages.

Currently, we're migrating from a **Code-first** to an **API-first** approach.
That means that you can't find all specifications here. You can find other specifications
in ReportPortal UI Settings. Little by little, we will provide the specifications here.

## Repository Structure

```tree
.
├── api
│   ├── openapi
│   └── proto
├── docs
├── gradle
├── src
└── templates
```

- [api](/api/) - root catalog for Report Portal Interface Definitions
- [openapi](/api/openapi/) - contains OpenAPI Specification for Report Portal APIs
- [proto](/api/proto/) - contains gRPC definitions for Report Portal services and types
  - [common](/api/proto/reportportal/common/) - contains common types for all Report Portal gRPC services
  - [reporting](/api/proto/reportportal/reporting/) - contains gRPC definitions for domain specific
  - v`N` - catalog for grouping definitions by versions
  - [buf.md (ext. link)](https://docs.buf.build/bsr/documentation) - it's analogous to a GitHub repository's
README.md and supports the CommonMark syntax
  - [buf.yaml (ext. link)](https://docs.buf.build/configuration/v1/buf-yaml) - defines a module, and is placed at
the root of the Protobuf source files it defines
- [docs](/docs/) - contains Guidelines for Report Portal APIs
- [gradle](/gradle/) - contains Gradle wrapper
- [src](/src/) - contains source code for building language-specific packages
  - [build](/src/build/) - catalog for generated stubs for clients or servers
- [templates](/templates/) - contains templates for Buf CLI generating code

## How to work with Protobuf

### Buf CLI

We use `buf` CLI to create consistent Protobuf APIs that preserve compatibility
and comply with best practices.

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

## Generate code

### Java

For generating Java code we use
[Protobuf Plugin for Gradle](https://github.com/google/protobuf-gradle-plugin).

Perform for generate code:

```shell
./gradlew generateProto
```

### Other languages

For other programming languages we use [buf](https://docs.buf.build/).

#### Remote generating

Buf doesn't have a built-in compiler. We use [remote plugins](https://docs.buf.build/bsr/remote-generation/overview)
for generating code for the following languages:

- Python
- C#
- JS Node
- TS

Perform for generate code:

```shell
buf generate
```

Optional languages:

- JS Web Connect Protocol
- Ruby
- Go
- Kotlin
- Scala

Perform for generate code:

```shell
buf generate --template templates/buf.gen.opt.yaml
```

#### Local generating

Before, you need to install [protocol buffer compiler](https://grpc.io/docs/protoc-installation/)
with standard plugins.

Config [buf.gen.local.yaml](./buf.gen.local.yaml) for your needs or use default `protoc` command.

Perform for generate code:

```shell
buf generate --template templates/buf.gen.local.yaml
```

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
