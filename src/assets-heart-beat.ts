import { DateTime } from "luxon"
import { writeAssetPrice } from "./contracts/services.js"
import { AssetRepository } from "./repositories/asset.repository.js"
import { ASSET, type Asset } from "./shared/constants.js"
import { sleep } from "./utils/sleep.js"

async function main() {
	console.log("🦀 assets heart beat is running")

	for (;;) {
		const assets = await AssetRepository.findAllTimelineReached()

		for (const asset of assets) {
			const { heartbeat, oracleContract, id } = ASSET[asset.id as Asset]

			await writeAssetPrice(oracleContract, asset.realTimePrice)

			await AssetRepository.updateAllPriceAndTimeline(
				id,
				asset.realTimePrice,
				DateTime.now().plus({ seconds: heartbeat })
			)

			console.log("done heartbeat on asset: ", asset.symbol)
		}

		await sleep(1000)
	}
}

main()
