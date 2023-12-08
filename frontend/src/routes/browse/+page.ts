import { getSchematics } from '$lib/requests.js'
import { error } from '@sveltejs/kit'


export const load = async ({url}) => {
    const q = url.searchParams.get('q')
    const schematics = await getSchematics({
        query: {
            term: q,
            limit: 50
        }
    }).result

    if(!schematics.ok) throw error(500)
    return {schematics: schematics.data}
}