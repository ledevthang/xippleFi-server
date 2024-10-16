import type { Decimal } from "@prisma/client/runtime/library"
import { rpcClient, walletClient, xrpChain } from "@root/infrastrutures/viem.js"
import { DateTime } from "luxon"
import { type Address, decodeEventLog, encodeAbiParameters } from "viem"
import { ORACLE_ABI } from "./oracle.contract.js"

export const writeAssetPrice = async (contract: Address, price: Decimal) => {
	const hash = await walletClient.writeContract({
		abi: ORACLE_ABI,
		address: contract,
		functionName: "requestNewRound",
		chain: xrpChain
	})

	const {
		logs: [log]
	} = await rpcClient.waitForTransactionReceipt({ hash })

	if (!log) throw new Error(`Can not find log event round from hash ${hash}`)

	const event = decodeEventLog({
		abi: ORACLE_ABI,
		data: log.data,
		topics: log.topics
	})

	if (event.eventName === "OwnershipTransferred")
		throw new Error("Unepected event")

	const nowInSeconds = Math.round(DateTime.now().toSeconds())

	const lastestEpoch = getLatestEpochAndRoundAsBytes32(
		nowInSeconds,
		event.args.round + 1
	)

	const hashData = encodeAbiParameters(
		[{ type: "uint32" }, { type: "int192" }],
		[nowInSeconds, BigInt(price.toHex())]
	)

	return walletClient.writeContract({
		abi: ORACLE_ABI,
		address: contract,
		chain: xrpChain,
		functionName: "transmit",
		args: [lastestEpoch, hashData]
	})
}

function getLatestEpochAndRoundAsBytes32(
	epoch: number,
	round: number
): `0x${string}` {
	// Combine epoch and round into a single uint40 value
	const uint40Value = (epoch << 8) | round // Shift epoch left by 8 bits and OR with round

	// Convert to a hexadecimal string and pad to 10 hex characters (40 bits)
	const hexString = uint40Value.toString(16)

	// Pad the hexadecimal string to ensure it is 64 characters long for a full bytes32 representation
	const paddedHexString = hexString.padStart(64, "0")

	// Return as bytes32 representation with '0x' prefix
	return `0x${paddedHexString}`
}
