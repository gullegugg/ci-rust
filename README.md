# CI-Rust

A webserver to handle github webhooks on push and then build and test this project.
So on webhook: Grab the repo, checkout commit, build projekt, run tests, collect result and publish.

Publish on github pr/commit check. Store result with build logs in sqlite, axum with templating to display info.
