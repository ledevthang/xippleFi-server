import { ORACLE_CONTRACT } from "@root/contracts/oracle.contract.js"
import { ASSET, type Asset } from "@root/shared/constants.js"

export const findStaticAssetById = (id: Asset) => {
	const asset = Object.values(ASSET).find(asset => asset.id === id)

	if (!asset) throw new Error(`Not found asset with id: ${id}`)

	return asset
}

export const findStaticAssetOracleContract = (id: Asset) => {
	switch (id) {
		case "bitcoin":
			return ORACLE_CONTRACT.BTC

		case "binance-coin":
			return ORACLE_CONTRACT.BNB

		case "ethereum":
			return ORACLE_CONTRACT.ETH

		case "tether":
			return ORACLE_CONTRACT.USDT

		case "tron":
			return ORACLE_CONTRACT.TRX

		case "xrp":
			return ORACLE_CONTRACT.XRP

		default:
			throw new Error(`Not found oracle contract address with id: ${id}`)
	}
}
