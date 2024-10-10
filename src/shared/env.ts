import z from "zod"

const schema = z.object({
	DATABASE_URL: z.string().min(1)
})

export const { DATABASE_URL } = schema.parse(process.env)
