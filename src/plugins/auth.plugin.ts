import type { FastifyPluginAsync } from "fastify"
import fastifyPlugin from "fastify-plugin"

const auth: FastifyPluginAsync<{
	options?: "require" | "optional"
}> = async (self, { options = "require" }) => {
	self.addHook("onRequest", async (request, reply) => {
		try {
			await request.jwtVerify()
		} catch {
			if (options === "optional") return
			throw reply.unauthorized()
		}
	})
}

export default fastifyPlugin(auth)
