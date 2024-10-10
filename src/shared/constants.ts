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
		threshold: 0.5
	},
	Eth: {
		id: "ethereum",
		symbol: "ETH",
		heartbeat: 3600, //1h
		threshold: 0.5
	},
	Xrp: {
		id: "xrp",
		symbol: "XRP",
		heartbeat: 3600 * 24, //24h
		threshold: 0.5
	},
	Tron: {
		id: "tron",
		symbol: "TRX",
		heartbeat: 600, //10mins
		threshold: 0.2
	},
	Bnb: {
		id: "binance-coin",
		symbol: "BNB",
		heartbeat: 3600 * 24, //24h
		threshold: 0.5
	},
	Usdt: {
		id: "tether",
		symbol: "USDT",
		heartbeat: 3600 * 24, //24h
		threshold: 0.25
	}
} as const
