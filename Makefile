bundle:
	for file in api/openapi/*.yaml; do \
		npx @redocly/cli@latest bundle "$$file" -d -o "api/openapi/bundles/$$(basename $$file)"; \
	done