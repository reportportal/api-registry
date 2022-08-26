---
stoplight-id: egurpy4cg3bk5
tags: [guideline]
---

# API Guidelines

- [API Guidelines](#api-guidelines)
  - [General Guidelines](#general-guidelines)
  - [API Design](#api-design)
  - [Meta information](#meta-information)
  - [Security](#security)
  - [Data formats](#data-formats)
  - [URLs](#urls)
  - [JSON payload](#json-payload)

Our API Guidelines are based on the Zalando
[RESTful API and Event Scheme Guidelines](https://opensource.zalando.com/restful-api-guidelines).

If you are not familiar with the Zalando API Guidelines, we recommend you to read them first.

Here you can find the main rules and differences between our API Guidelines and the
Zalando API Guidelines.

> **Note**: The Zalando API Guidelines contains x-extensions that are not supported our API Guidlines.
> Please, do not use them.

## General Guidelines

The first read our [Principles](./principles.md).

There you find all the general rules and conventions for API design.

## API Design

## Meta information

> **Note**: Avoid use others rules not covered in our API Guidelines for this part.

**MUST** contain API meta information
[218](https://opensource.zalando.com/restful-api-guidelines/#218)

- #/info/title as (unique) identifying, functional descriptive name of the API
- #/info/version to distinguish API specifications versions following semantic rules
- #/info/description containing a proper description of the API
- #/info/contact/{name,url,email} containing the responsible team/developer

**MUST** use semantic versioning
[116](https://opensource.zalando.com/restful-api-guidelines/#116).

## Security

**MUST** secure endpoints
[104](https://opensource.zalando.com/restful-api-guidelines/#104).

## Data formats

**MUST** use standard data formats
[238](https://opensource.zalando.com/restful-api-guidelines/#238).

**MUST** define a format for number and integer types
[171](https://opensource.zalando.com/restful-api-guidelines/#171).

**MUST** use standard formats for date and time properties
[169](https://opensource.zalando.com/restful-api-guidelines/#169).

**MUST** use standard formats for country, language and currency properties
[170](https://opensource.zalando.com/restful-api-guidelines/#170).

*SHOULD* only use UUIDs if necessary
[144](https://opensource.zalando.com/restful-api-guidelines/#144).

>Generating IDs can be a scaling problem in high frequency and near real time use cases.
>UUIDs solve this problem, as they can be generated without collisions in a distributed,
>non-coordinated way and without additional server round trips.

## URLs

*SHOULD* not use /api as base path
[135](https://opensource.zalando.com/restful-api-guidelines/#135).

**MUST** pluralize resource names
[134](https://opensource.zalando.com/restful-api-guidelines/#134).

**MUST** use URL-friendly resource identifiers
[228](https://opensource.zalando.com/restful-api-guidelines/#228).

**MUST** use kebab-case for path segments
[129](https://opensource.zalando.com/restful-api-guidelines/#129).

> Hint: kebab-case applies to concrete path segments and not necessarily the names of path parameters.

**MUST** use snake_case (never camelCase) for query parameters
[130](https://opensource.zalando.com/restful-api-guidelines/#130).

**MUST** use normalized paths without empty path segments and trailing slashes
[136](https://opensource.zalando.com/restful-api-guidelines/#136).

MUST stick to conventional query parameters
[137](https://opensource.zalando.com/restful-api-guidelines/#137).

**MUST** keep URLs verb-free
[141](https://opensource.zalando.com/restful-api-guidelines/#141).

**MUST** avoid actions — think about resources
[138](https://opensource.zalando.com/restful-api-guidelines/#138).

> PUT /article-locks/{article-id}

*SHOULD* define useful resources
[140](https://opensource.zalando.com/restful-api-guidelines/#140).

**MUST** use domain-specific resource names
[142](https://opensource.zalando.com/restful-api-guidelines/#142).

>"sales-order-items" is superior to "order-items"

**MUST** identify resources and sub-resources via path segments
[143](https://opensource.zalando.com/restful-api-guidelines/#143).

>/resources/{resource-id}/sub-resources/{sub-resource-id}

*SHOULD* limit number of resource types
[146](https://opensource.zalando.com/restful-api-guidelines/#146).

## JSON payload
