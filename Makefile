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
		api/openapi/apis/_info.yaml \
		api/openapi/apis/*.yaml \
		-o api/openapi/reportportal-api.yaml \
		--without-x-tag-groups
	echo '# !!!DO NOT EDIT IT DIRECTLY!!!\n# This is an autogenerated file.\n# Any changes made to this file will be overwritten.\n# Please make changes to the source files and regenerate this file.' | cat - api/openapi/reportportal-api.yaml > temp && mv temp api/openapi/reportportal-api.yaml