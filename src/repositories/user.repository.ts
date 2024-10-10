import { prisma } from "@root/infrastrutures/database.js"
import type { Address } from "viem"

const createIfNotExist = (address: Address) =>
	prisma.user.upsert({
		where: {
			address
		},
		create: {
			address
		},
		update: {}
	})

export const UserRepository = { createIfNotExist }
