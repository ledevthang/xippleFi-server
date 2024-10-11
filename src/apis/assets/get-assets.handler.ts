import { AssetRepository } from "@root/repositories/asset.repository.js"
import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"

const handler: FastifyPluginAsyncZod = async self => {
	self.get(
		"/",
		{
			schema: {
				tags: ["Assets"]
			}
		},
		async () => {
			const assets = await AssetRepository.findAll()

			return {
				assets
			}
		}
	)
}

export default handler
