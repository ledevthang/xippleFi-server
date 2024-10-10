import { Decimal } from "@prisma/client/runtime/library"
import { DateTime } from "luxon"
import { parseEther } from "viem"
import { WebSocket } from "ws"
import { prisma } from "./infrastrutures/database.js"
import { Asset } from "./shared/constants.js"

async function main() {
	const assets = Object.values(Asset)
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
	const asset = await prisma.asset.findUnique({
		where: {
			id
		}
	})

	const { symbol, heartbeat, threshold } = Object.values(Asset).find(
		asset => asset.id === id
	)!

	if (!asset) {
		const timeline = DateTime.now().plus({ seconds: heartbeat }).toJSDate()

		await prisma.asset.create({
			data: {
				id,
				price,
				symbol,
				timeline
			}
		})

		return
	}

	const change = Decimal.abs(price.minus(asset.price)).div(price)

	if (change.lessThan(threshold)) return

	//need update to contract here
	await prisma.asset.update({
		where: {
			id
		},
		data: {
			price,
			timeline: DateTime.now().plus({ seconds: heartbeat }).toJSDate()
		}
	})
}

main()
