module.exports = {
	apps: [
		{
			name: "server",
			script: "node --env-file=./.env ./dist/server.js"
		},
		{
			name: "assets-crawler",
			script: "node --env-file=./.env ./dist/assets-crawler.js"
		},
		{
			name: "assets-heart-beat",
			script: "node --env-file=./.env ./dist/assets-heart-beat.js"
		}
	]
}
