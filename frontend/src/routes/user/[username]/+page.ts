import { GET } from '$lib/requests.js'
import { error } from '@sveltejs/kit'

export const load = async ({params})=>{
    const [user, collections, schematics] = await Promise.all([
        GET("/v1/users/{user_id}", {
            params: {
                path: {user_id: params.username}
            }
        }),
        GET("/v1/users/{user_id}/collections", {
            params: {
                path: {user_id: params.username}
            }
        }),
        GET("/v1/users/{user_id}/schematics", {
            params: {
                path: {user_id: params.username}
            }
        })
    ])

    if(!user.data) throw error(404, "User not found")
    if(!collections.data || !schematics.data) throw error(500, "Internal error")

    return {user: user.data, collections: collections.data, schematics: schematics.data}
}