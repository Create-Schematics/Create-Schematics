import { getSchematics } from '$lib/requests.js'
import { error } from '@sveltejs/kit'


export const load = async ({url}) => {
    const q = url.searchParams.get('q')
    const page = url.searchParams.get('page')
    const schematics = await getSchematics({
        query: {
            term: q,
            limit: 50,
            offset: page ? parseInt(page) * 50 : 0
        }
    }).result

    if(!schematics.ok) throw error(500)
    return {schematics: schematics.data}
}