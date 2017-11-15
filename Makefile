dev: Dockerfile-dev
	docker build --tag local/code/game-of-life --file Dockerfile-dev .

shell: dev
	docker run --interactive --tty --rm --volume $$(pwd):/workdir --workdir /workdir --env USER=mason local/code/game-of-life
