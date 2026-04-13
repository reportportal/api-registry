# Standard REST API Errors

This page describes the standard REST API errors that you can use in your API documentation.

## Standard REST API Errors definitions

| HTTP Status Code | Description |
| - | - |
| 400 Bad Request | Use when an endpoint contains parameters or request body. They could be missing, invalid, or not conforming to the expected format. |
| 401 Unauthorized | Use for private endpoints when authentication is required. |
| 402 Payment Required | Use if API endpoint provides a payment feature. The user has not made the required payment to access the resource. |
| 403 Forbidden | Use when the endpoint defines `permissions`. |
| 404 Not Found | Use when the endpoint contains a path parameter identifying a resource. The resource could not be found. |
| 409 Conflict | Use when an endpoint creates a resource, can change a unique parameter of the resource like email, name, etc., or changes the state of the resource. |
| 429 Too Many Requests | Use when the API endpoint has a rate limit defined. The user has exceeded the number of allowed requests in a given time frame. |

## Example of using standard REST API errors in OpenAPI specification

```yaml
responses:
  '400':
    $ref: '#/components/responses/Error'
  '401':
    $ref: '#/components/responses/Error'
  '402':
    $ref: '#/components/responses/Error'
  '403':
    $ref: '#/components/responses/Error'
  '404':
    $ref: '#/components/responses/Error'
  '409':
    $ref: '#/components/responses/Error'
  '429':
    $ref: '#/components/responses/Error'
components:
  responses:
    Error:
      description: Error response
      content:
        application/json:
          schema:
            $ref: '#/components/schemas/Error'
```
