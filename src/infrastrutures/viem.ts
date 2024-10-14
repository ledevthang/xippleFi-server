import { PRIVATE_KEY } from "@root/shared/env.js"
import { http, createPublicClient, createWalletClient, defineChain } from "viem"
import { privateKeyToAccount } from "viem/accounts"

const transport = http("https://rpc-evm-sidechain.xrpl.org/")

export const superUser = privateKeyToAccount(`0x${PRIVATE_KEY}`)

export const xrpChain = defineChain({
	id: Number(1440002),
	name: "ripple",
	nativeCurrency: {
		decimals: 18,
		name: "xrp",
		symbol: "XRP"
	},
	rpcUrls: {
		default: { http: ["https://rpc-evm-sidechain.xrpl.org/"] }
	}
})

export const rpcClient = createPublicClient({
	transport,
	chain: xrpChain
})

export const walletClient = createWalletClient({
	account: superUser,
	transport
})
