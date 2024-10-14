import { DateTime } from "luxon"
import type { Address } from "viem"
import { writeAssetPrice } from "./contracts/services.js"
import { findStaticAssetById } from "./helpers/find-static.js"
import { AssetRepository } from "./repositories/asset.repository.js"
import type { Asset } from "./shared/constants.js"
import { sleep } from "./utils/sleep.js"

async function main() {
	console.log("🦀 assets heart beat is running")

	for (;;) {
		const assets = await AssetRepository.findAllTimelineReached()

		for (const asset of assets) {
			const { heartbeat } = findStaticAssetById(asset.id as Asset)

			await writeAssetPrice(asset.address as Address, asset.realTimePrice)

			await AssetRepository.updateAllPriceAndTimeline(
				asset.id as Asset,
				asset.realTimePrice,
				DateTime.now().plus({ seconds: heartbeat })
			)

			console.log("done heartbeat on asset: ", asset.symbol)
		}

		await sleep(1000)
	}
}

main()
