export type Asset =
	| "bitcoin"
	| "ethereum"
	| "xrp"
	| "tron"
	| "binance-coin"
	| "tether"

export const Asset = {
	Btc: {
		id: "bitcoin",
		symbol: "BTC",
		heartbeat: 3600, //1h
		threshold: 0.5,
		apy: 0.01
	},
	Eth: {
		id: "ethereum",
		symbol: "ETH",
		heartbeat: 3600, //1h
		threshold: 0.5,
		apy: 0.01
	},
	Xrp: {
		id: "xrp",
		symbol: "XRP",
		heartbeat: 3600 * 24, //24h
		threshold: 0.5,
		apy: 0.5
	},
	Tron: {
		id: "tron",
		symbol: "TRX",
		heartbeat: 600, //10mins
		threshold: 0.2,
		apy: 0.5
	},
	Bnb: {
		id: "binance-coin",
		symbol: "BNB",
		heartbeat: 3600 * 24, //24h
		threshold: 0.5,
		apy: 0.6
	},
	Usdt: {
		id: "tether",
		symbol: "USDT",
		heartbeat: 3600 * 24, //24h
		threshold: 0.25,
		apy: 1
	}
} as const
