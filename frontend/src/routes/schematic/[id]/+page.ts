import { GET } from '$lib/requests.js'
import { error } from '@sveltejs/kit'

export const load = async ({ params }) => {

    const schematic = await GET("/api/v1/schematics/{schematic_id}", {
        params: {
            path: {
                schematic_id: params.id
            }
        }
    })
    const tags = await GET("/api/v1/schematics/{schematic_id}/tags", {
        params: {
            path: {
                schematic_id: params.id
            }
        }
    })
    const comments = await GET("/api/v1/schematics/{schematic_id}/comments", {
        params: {
            path: {
                schematic_id: params.id
            },
            query: {query: {}}
        }
    })
    if(!schematic.data) throw error(404, "Schematic not found")
    if(!tags.data) throw error(500)
    if(!comments.data) throw error(500)
    
    return {schematic: schematic.data, tags: tags.data, comments: comments.data}
}