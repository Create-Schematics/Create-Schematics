import { GET } from '$lib/requests.js'
import { error } from '@sveltejs/kit'


export const load = async ({url}) => {
    const term = url.searchParams.get('term')
    const page = url.searchParams.get('page')
    const schematics = await GET("/v1/schematics", {
        params: {
            query: {
                term: term??undefined,
                limit: 50,
                offset: page ? parseInt(page) * 50 : 0
            }
        }
    })

    console.log(schematics);

    if(!schematics.data) throw error(500)
    return {schematics: schematics.data}
}