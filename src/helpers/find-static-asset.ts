import { Asset } from "@root/shared/constants.js"

export const findStaticAssetById = (id: Asset) => {
	const asset = Object.values(Asset).find(asset => asset.id === id)

	if (!asset) throw new Error(`Not found asset with id: ${id}`)

	return asset
}
