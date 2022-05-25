package main

import (
	"dagger.io/dagger"
	"universe.dagger.io/bash"
	"universe.dagger.io/docker"
)

dagger.#Plan & {
	client: filesystem: "./": read: {
		contents: dagger.#FS
		exclude: [
			"README.md",
			"ci.cue",
			"Vagrantfile",
			"target/",
			".vagrant/",
		]
	}

	actions: {
		deps: docker.#Build & {
			steps: [
				docker.#Pull & {
					source: "rust:1.61.0"
				},
				docker.#Copy & {
					contents: client.filesystem."./".read.contents
					dest:     "/src"
				},
			]
		}
		test: bash.#Run & {
			input:   deps.output
			workdir: "/src"
			script: contents: #"""
				cargo test
				"""#
		}
	}
}
