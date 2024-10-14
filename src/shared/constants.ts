export type Asset =
	| "bitcoin"
	| "ethereum"
	| "xrp"
	| "tron"
	| "binance-coin"
	| "tether"

export const ASSET = {
	bitcoin: {
		id: "bitcoin",
		symbol: "BTC",
		heartbeat: 3600, //1h
		threshold: 0.5,
		apy: 0.01,
		oracleContract: "0x8B84bdC3Ac0db29003625dc2FdF9902b80e2484F",
		address: "0x06d7354F56209fa461B27A173316013EcC4a4c99"
	},
	ethereum: {
		id: "ethereum",
		symbol: "ETH",
		heartbeat: 3600, //1h
		threshold: 0.5,
		apy: 0.01,
		oracleContract: "0xC6a1f4925676E0f81f871C53d2C5A7Cff7B773c6",
		address: "0x78AE63017E18520cf63CbA0a5CF190d7f04Cb3f6"
	},
	xrp: {
		id: "xrp",
		symbol: "XRP",
		heartbeat: 3600 * 24, //24h
		threshold: 0.5,
		apy: 0.5,
		oracleContract: "0x50E67748dBdb608bE5b85d97b0Da72313f7Faf4f",
		address: "0x21fa8610CBD3a1a45bCB1DbE933052EBF9e3dd52"
	},
	tron: {
		id: "tron",
		symbol: "TRX",
		heartbeat: 600, //10mins
		threshold: 0.2,
		apy: 0.5,
		oracleContract: "0x7D2e4B489a9058E728Bd9B63b23251A29f0Ed246",
		address: "0x7D2e4B489a9058E728Bd9B63b23251A29f0Ed246"
	},
	"binance-coin": {
		id: "binance-coin",
		symbol: "BNB",
		heartbeat: 3600 * 24, //24h
		threshold: 0.5,
		apy: 0.6,
		oracleContract: "0x02f6C887a1C0857bF7106c02FAeF05d46Ba6aBEf",
		address: "0xe2C179BB9e31Cd6f16142D1C8d2dDB7458b371Ca"
	},
	tether: {
		id: "tether",
		symbol: "USDT",
		heartbeat: 3600 * 24, //24h
		threshold: 0.25,
		apy: 1,
		oracleContract: "0x4f3110350D0F6510F3bA7792d9E1be68D5937c9A",
		address: "0xcD84fcd2964612D1585F1494B8Ed4F1Ae29D32AC"
	}
} as const
