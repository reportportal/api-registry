file ?= reportportal-api.yaml

lint:
	npx @redocly/cli@latest lint --lint-config=error api/openapi/$(file)

lint-all:
	npx @redocly/cli@latest lint api/openapi/apis/*.yaml

bundle:
	npx @redocly/cli@latest bundle api/openapi/$(file) \
		-o "build/openapi/$(file)" \
		--remove-unused-components

bundle-all:
	npx @redocly/cli@latest bundle api/openapi/apis/*.yaml \
		-o "build/openapi/" \
		--dereferenced \
		--remove-unused-components

join:
	npx @redocly/cli@latest join \
		api/openapi/apis/organizations.yaml \
		api/openapi/apis/users.yaml \
		-o api/openapi/reportportal-api.yaml \
		--without-x-tag-groups 