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
