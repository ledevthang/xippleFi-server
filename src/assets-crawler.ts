import { Decimal } from "@prisma/client/runtime/library"
import { DateTime } from "luxon"
import { parseEther } from "viem"
import { WebSocket } from "ws"
import { writeAssetPrice } from "./contracts/services.js"
import {
	findStaticAssetById,
	findStaticAssetOracleContract
} from "./helpers/find-static.js"
import { AssetRepository } from "./repositories/asset.repository.js"
import { ASSET, type Asset } from "./shared/constants.js"

async function main() {
	const assets = Object.values(ASSET)
		.map(asset => asset.id)
		.join(",")

	const socket = new WebSocket(`wss://ws.coincap.io/prices?assets=${assets}`)

	socket.onmessage = async msg => {
		const payload = JSON.parse(msg.data as string)

		for (const [asset, priceStr] of Object.entries<string>(payload)) {
			const price = new Decimal(parseEther(priceStr).toString())

			await handleAsset(asset as Asset, price)
		}
	}

	socket.onerror = error => {
		console.error("error occur from handshake", error)
		main()
	}

	console.log("🦀 assets crawler is running")
}

const handleAsset = async (id: Asset, price: Decimal) => {
	const asset = await AssetRepository.findById(id)

	const { symbol, heartbeat, threshold, apy } = findStaticAssetById(id)

	const address = findStaticAssetOracleContract(id)

	if (!asset) {
		const timeline = DateTime.now().plus({ seconds: heartbeat })

		await AssetRepository.create({
			id,
			price,
			symbol,
			timeline,
			apy,
			realTimePrice: price,
			address
		})

		return
	}

	const change = Decimal.abs(price.minus(asset.price)).div(price)

	if (change.lessThan(threshold)) {
		await AssetRepository.updateRealtimePrice(id, price)
		return
	}

	//need update to contract here

	await writeAssetPrice(address, price)

	await AssetRepository.updateAllPriceAndTimeline(
		id,
		price,
		DateTime.now().plus({ seconds: heartbeat })
	)

	console.log("done update price asset: ", asset.symbol)
}

main()
