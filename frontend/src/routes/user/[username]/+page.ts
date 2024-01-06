import { GET } from '$lib/requests.js'
import { error } from '@sveltejs/kit'

export const load = async ({params})=>{
    const [user, schematics] = await Promise.all([
        GET("/v1/users/{username}", {
            params: {
                path: {username: params.username}
            }
        }),
        GET("/v1/users/{username}/schematics", {
            params: {
                path: {username: params.username}
            }
        })
    ])



    if(!user.data) throw error(404, "User not found")

    const collections = await GET("/v1/users/{user_id}/collections", {
        params: {
            path: {
                user_id: user.data.user_id
            }
        }
    })

    if(!collections.data || !schematics.data) throw error(500, "Internal error")

    return {user: user.data, collections: collections.data, schematics: schematics.data}
}