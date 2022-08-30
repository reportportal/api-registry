---
stoplight-id: egurpy4cg3bk5
tags: [guideline]
---

# API Guidelines

Our API Guidelines are based on the Zalando
[RESTful API and Event Scheme Guidelines](https://opensource.zalando.com/restful-api-guidelines).

If you are not familiar with the Zalando API Guidelines, we recommend you to read them first.

Here you can find the main rules and differences between our API Guidelines and the
Zalando API Guidelines.

## General Guidelines

The first read our [Principles](./principles.md).

There you find all the general rules and conventions for API design.

## API Design

## Meta information

**MUST** contain API meta information
[218](https://opensource.zalando.com/restful-api-guidelines/#218).

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

**MUST** encode binary data in base64url

**MUST** use standard formats for date and time properties
[169](https://opensource.zalando.com/restful-api-guidelines/#169).

**MUST** use standard formats for country, language and currency properties
[170](https://opensource.zalando.com/restful-api-guidelines/#170).

*SHOULD* only use UUIDs if necessary
[144](https://opensource.zalando.com/restful-api-guidelines/#144).

## URLs

SHOULD not use /api as base path
[135](https://opensource.zalando.com/restful-api-guidelines/#135).

MUST pluralize resource names
[134](https://opensource.zalando.com/restful-api-guidelines/#134).

MUST use URL-friendly resource identifiers
[228](https://opensource.zalando.com/restful-api-guidelines/#228).

MUST use kebab-case for path segments 
[129](https://opensource.zalando.com/restful-api-guidelines/#129).

MUST use normalized paths without empty path segments and trailing slashes
[136](https://opensource.zalando.com/restful-api-guidelines/#136).

MUST keep URLs verb-free
[141](https://opensource.zalando.com/restful-api-guidelines/#141).

MUST avoid actions — think about resources
[138](https://opensource.zalando.com/restful-api-guidelines/#138).

> PUT /article-locks/{article-id}

SHOULD define useful resources
[140](https://opensource.zalando.com/restful-api-guidelines/#140).

**MUST** use domain-specific resource names
[142](https://opensource.zalando.com/restful-api-guidelines/#142).

> For example: "sales-order-items" is superior to "order-items" in that it clearly
> indicates which business object it represents.

**MUST** identify resources and sub-resources via path segments
[143](https://opensource.zalando.com/restful-api-guidelines/#143).

**MUST** use snake_case (never camelCase) for query parameters
[130](https://opensource.zalando.com/restful-api-guidelines/#130).

MUST stick to conventional query parameters
[137](https://opensource.zalando.com/restful-api-guidelines/#137).

## JSON payload

