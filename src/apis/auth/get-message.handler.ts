import { userSignMessageKey } from "@root/helpers/redis.js"
import { redis } from "@root/infrastrutures/redis.js"
import { address } from "@root/shared/parsers.js"
import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import randomstring from "randomstring"
import { hashMessage } from "viem"
import z from "zod"

const querystring = z.object({
	address: address()
})

const handler: FastifyPluginAsyncZod = async self => {
	self.get(
		"/message",
		{
			schema: {
				tags: ["User"],
				querystring
			}
		},
		async ({ query }) => {
			const message = hashMessage(
				randomstring.generate({
					charset: "alphanumeric",
					length: 8
				})
			)

			await redis.set(userSignMessageKey(query.address), message, "EX", 300) // 5mins

			return {
				message
			}
		}
	)
}

export default handler
