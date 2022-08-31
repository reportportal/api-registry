# Principles

- [Principles](#principles)
  - [RESTful API](#restful-api)
  - [API first](#api-first)
  - [API design](#api-design)

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
