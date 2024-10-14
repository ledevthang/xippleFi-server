export const ORACLE_ABI = [
	{
		inputs: [
			{
				internalType: "uint8",
				name: "decimals_",
				type: "uint8"
			},
			{
				internalType: "string",
				name: "description_",
				type: "string"
			}
		],
		stateMutability: "nonpayable",
		type: "constructor"
	},
	{
		inputs: [
			{
				internalType: "address",
				name: "owner",
				type: "address"
			}
		],
		name: "OwnableInvalidOwner",
		type: "error"
	},
	{
		inputs: [
			{
				internalType: "address",
				name: "account",
				type: "address"
			}
		],
		name: "OwnableUnauthorizedAccount",
		type: "error"
	},
	{
		anonymous: false,
		inputs: [
			{
				indexed: true,
				internalType: "address",
				name: "previousOwner",
				type: "address"
			},
			{
				indexed: true,
				internalType: "address",
				name: "newOwner",
				type: "address"
			}
		],
		name: "OwnershipTransferred",
		type: "event"
	},
	{
		anonymous: false,
		inputs: [
			{
				indexed: true,
				internalType: "address",
				name: "requester",
				type: "address"
			},
			{
				indexed: false,
				internalType: "uint32",
				name: "epoch",
				type: "uint32"
			},
			{
				indexed: false,
				internalType: "uint8",
				name: "round",
				type: "uint8"
			}
		],
		name: "RoundRequested",
		type: "event"
	},
	{
		inputs: [],
		name: "decimals",
		outputs: [
			{
				internalType: "uint8",
				name: "",
				type: "uint8"
			}
		],
		stateMutability: "view",
		type: "function"
	},
	{
		inputs: [],
		name: "description",
		outputs: [
			{
				internalType: "string",
				name: "",
				type: "string"
			}
		],
		stateMutability: "view",
		type: "function"
	},
	{
		inputs: [
			{
				internalType: "uint80",
				name: "roundId",
				type: "uint80"
			}
		],
		name: "getRoundData",
		outputs: [
			{
				internalType: "uint80",
				name: "roundId_",
				type: "uint80"
			},
			{
				internalType: "int256",
				name: "answer",
				type: "int256"
			},
			{
				internalType: "uint256",
				name: "startedAt",
				type: "uint256"
			},
			{
				internalType: "uint256",
				name: "updatedAt",
				type: "uint256"
			},
			{
				internalType: "uint80",
				name: "answeredInRound",
				type: "uint80"
			}
		],
		stateMutability: "view",
		type: "function"
	},
	{
		inputs: [],
		name: "latestRoundData",
		outputs: [
			{
				internalType: "uint80",
				name: "roundId_",
				type: "uint80"
			},
			{
				internalType: "int256",
				name: "answer",
				type: "int256"
			},
			{
				internalType: "uint256",
				name: "startedAt",
				type: "uint256"
			},
			{
				internalType: "uint256",
				name: "updatedAt",
				type: "uint256"
			},
			{
				internalType: "uint80",
				name: "answeredInRound",
				type: "uint80"
			}
		],
		stateMutability: "view",
		type: "function"
	},
	{
		inputs: [],
		name: "owner",
		outputs: [
			{
				internalType: "address",
				name: "",
				type: "address"
			}
		],
		stateMutability: "view",
		type: "function"
	},
	{
		inputs: [],
		name: "renounceOwnership",
		outputs: [],
		stateMutability: "nonpayable",
		type: "function"
	},
	{
		inputs: [],
		name: "requestNewRound",
		outputs: [
			{
				internalType: "uint80",
				name: "",
				type: "uint80"
			}
		],
		stateMutability: "nonpayable",
		type: "function"
	},
	{
		inputs: [
			{
				internalType: "address",
				name: "newOwner",
				type: "address"
			}
		],
		name: "transferOwnership",
		outputs: [],
		stateMutability: "nonpayable",
		type: "function"
	},
	{
		inputs: [
			{
				internalType: "bytes32",
				name: "_epochAndRound",
				type: "bytes32"
			},
			{
				internalType: "bytes",
				name: "report",
				type: "bytes"
			}
		],
		name: "transmit",
		outputs: [],
		stateMutability: "nonpayable",
		type: "function"
	}
] as const

export const ORACLE_CONTRACT = {
	BTC: "0x8B84bdC3Ac0db29003625dc2FdF9902b80e2484F",
	ETH: "0xC6a1f4925676E0f81f871C53d2C5A7Cff7B773c6",
	XRP: "0x50E67748dBdb608bE5b85d97b0Da72313f7Faf4f",
	TRX: "0x7D2e4B489a9058E728Bd9B63b23251A29f0Ed246",
	BNB: "0x02f6C887a1C0857bF7106c02FAeF05d46Ba6aBEf",
	USDT: "0x4f3110350D0F6510F3bA7792d9E1be68D5937c9A"
} as const
