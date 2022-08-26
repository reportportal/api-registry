---
stoplight-id: q1ws5ttd85q56
tags: [architecture]
---

# Principles

We'll start with a few concepts that will be useful throughout the rest of the API Guidelines.

## RESTful API

We use the RESTful API design style for our APIs:

- We prefer REST-based APIs with JSON payloads over RPC-based APIs with XML payloads.
- We prefer services to be truly RESTful, rather than "REST-like".

An important principle for API design and usage is Postel’s Law, aka
[The Robustness Principle](http://en.wikipedia.org/wiki/Robustness_principle)
(see also RFC [1122](https://tools.ietf.org/html/rfc1122)):

> Be liberal in what you accept, be conservative in what you send.

## API first

We decided to follow the API-First principle because we want to provide a better experience.

We believe that approach will catalyze our community to create new plugins and services.

There are two aspects:

- define APIs first, before coding its implementation, using a standard specification language.
- get early review feedback from peers and developers.

## API design

We use
[Zalando RESTful API and Event Scheme Guidelines](https://opensource.zalando.com/restful-api-guidelines)
as a base for our API Guidelines.

For this reason, we use the similar rules and conventions for API design.

Also, we use [RFC 2119](https://tools.ietf.org/html/rfc2119) to define the levels of conformance.

### General Guidelines

**MUST** follow the API-First principle:

- Define APIs first, before coding its implementation, using a standard specification language.
- Design APIs using our [API Guidelines](./guidelines.md).
- Use [Spectral](https://github.com/stoplightio/spectral) to validate your API design.
- Call for early review feedback from peers and client developers.

**MUST** provide API specification using [OpenAPI Specification 3.x](
https://spec.openapis.org/oas/v3.1.0.html).

*SHOULD* use the [Stoplight Studio](https://stoplight.io/studio/) to design and document our APIs.

**MUST** write APIs using U.S. English

*SHOULD* don't repeat yourself (The DRY principle).

>There's no need to specify the same property in different places in the     specification.
>
>Instead, move it to the `components` section and reference it from other places using `$ref.`

*SHOULD* split the document into several files.

>A good rule of thumb is to use the natural hierarchy present in URLs to build your file structure.
>
>For example, put all routes starting with /users (like /users and /users/{id}) in the same file
>(think of it as a “sub-API”).