help:
	@echo
	@echo dev: Builds the dev Docker image.
	@echo shell: Runs a container from the dev Docker image.
	@echo

dev: Dockerfile-dev
	docker build $(opts) --tag local/code/game-of-life --file Dockerfile-dev .

shell: dev
	docker run --env TERM --interactive --tty --rm --volume $$(pwd):/workdir --workdir /workdir --env USER=$$USER local/code/game-of-life
