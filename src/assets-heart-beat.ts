import { DateTime } from "luxon"
import { findStaticAssetById } from "./helpers/find-static-asset.js"
import { AssetRepository } from "./repositories/asset.repository.js"
import type { Asset } from "./shared/constants.js"
import { sleep } from "./utils/sleep.js"

async function main() {
	console.log("🦀 assets heart beat is running")

	for (;;) {
		const assets = await AssetRepository.findAllTimelineReached()

		for (const asset of assets) {
			const { heartbeat } = findStaticAssetById(asset.id as Asset)

			// need upadate price to blockchain here
			await AssetRepository.updatePriceAndTimeline(
				asset.id as Asset,
				asset.realTimePrice,
				DateTime.now().plus({ seconds: heartbeat })
			)
		}

		await sleep(1000)
	}
}

main()
