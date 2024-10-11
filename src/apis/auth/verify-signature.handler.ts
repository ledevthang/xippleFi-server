import type { FastifyPluginAsyncZod } from "fastify-type-provider-zod"
import { type Address, verifyMessage } from "viem"
import z from "zod"

import { userSignMessageKey } from "@root/helpers/redis.js"
import { redis } from "@root/infrastrutures/redis.js"
import { UserRepository } from "@root/repositories/user.repository.js"
import { address } from "@root/shared/parsers.js"

const handler: FastifyPluginAsyncZod = async self => {
	self.post(
		"/verify-signature",
		{
			schema: {
				tags: ["Auth"],
				body: z.object({
					address: address(),
					message: z.string().min(1),
					signature: z.string().min(1)
				})
			}
		},
		async ({ body }, reply) => {
			const { address, message, signature } = body

			const key = userSignMessageKey(address)

			const msg = await redis.get(key)

			if (msg !== message) throw reply.unauthorized()

			const isValid = await verifyMessage({
				address,
				message,
				signature: signature as Address
			})

			if (!isValid) throw reply.unauthorized()

			await UserRepository.createIfNotExist(address)

			const payload = {
				address
			}

			const accessToken = await reply.jwtSign(payload, {
				sign: {
					expiresIn: "3d"
				}
			})

			return {
				accessToken
			}
		}
	)
}

export default handler
