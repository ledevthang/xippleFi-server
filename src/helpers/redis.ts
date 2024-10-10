import type { Address } from "viem"

export const userSignMessageKey = (address: Address) => `message_${address}`
