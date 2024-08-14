lint:
	npx @redocly/cli@latest lint api/openapi/$(file)

lint-all:
	npx @redocly/cli@latest lint api/openapi/endpoints/*.yaml

bundle:
	npx @redocly/cli@latest bundle api/openapi/$(file) \
		-o "api/openapi/bundles/$(file)" \
		--remove-unused-components

bundle-all:
	npx @redocly/cli@latest bundle api/openapi/endpoints/*.yaml \
		-o "build/openapi/" \
		--dereferenced \
		--remove-unused-components

join:
	npx @redocly/cli@latest join \
		api/openapi/endpoints/organizations.yaml \
		api/openapi/endpoints/users.yaml \
		-o api/openapi/reportportal-api.yaml \
		--without-x-tag-groups 