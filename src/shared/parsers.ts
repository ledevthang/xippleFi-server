import { isAddress } from "viem"
import z from "zod"

export const address = () =>
	z.string().refine(isAddress, { message: "expected ethereum address" })
