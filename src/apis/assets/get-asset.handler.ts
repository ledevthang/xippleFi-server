import { AssetRepository } from "@root/repositories/asset.repository.js"
import { ASSET, type Asset } from "@root/shared/constants.js"
import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { z } from "zod"

const handler: FastifyPluginAsyncZod = async self => {
	self.get(
		"/:id",
		{
			schema: {
				tags: ["Assets"],
				params: z.object({
					id: z.enum(Object.keys(ASSET) as [Asset])
				})
			}
		},
		async ({ params }) => {
			const asset = await AssetRepository.findById(params.id)

			return {
				asset
			}
		}
	)
}

export default handler
