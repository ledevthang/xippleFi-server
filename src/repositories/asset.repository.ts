import type { Decimal } from "@prisma/client/runtime/library"
import { prisma } from "@root/infrastrutures/database.js"
import type { Asset } from "@root/shared/constants.js"
import { DateTime } from "luxon"

type CreateAssetParams = {
	id: Asset
	price: Decimal
	realTimePrice: Decimal
	symbol: string
	timeline: DateTime
	apy: number
}

const findAll = () => prisma.asset.findMany()

const findAllTimelineReached = () =>
	prisma.asset.findMany({
		where: {
			timeline: {
				lte: DateTime.now().toJSDate()
			}
		}
	})

const findById = (id: Asset) =>
	prisma.asset.findUnique({
		where: {
			id
		}
	})

const create = (params: CreateAssetParams) =>
	prisma.asset.create({
		data: {
			apy: params.apy,
			id: params.id,
			price: params.price,
			symbol: params.symbol,
			timeline: params.timeline.toJSDate(),
			realTimePrice: params.realTimePrice
		}
	})

const updatePriceAndTimeline = (
	id: Asset,
	price: Decimal,
	timeline: DateTime
) =>
	prisma.asset.update({
		where: {
			id
		},
		data: {
			price,
			timeline: timeline.toJSDate()
		}
	})

export const AssetRepository = {
	create,
	findById,
	findAll,
	findAllTimelineReached,
	updatePriceAndTimeline
}
