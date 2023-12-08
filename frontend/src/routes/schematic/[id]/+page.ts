import { getSchematic, getSchematicComments, getSchematicTags } from '$lib/requests.js'
import { error } from '@sveltejs/kit'

export const load = async ({ params }) => {

    const schematic = await getSchematic({schematic_id: params.id}).result
    const tags = await getSchematicTags({schematic_id: params.id}).result
    const comments = await getSchematicComments({schematic_id: params.id, query: {}}).result
    if(!schematic.ok) throw error(404, "Schematic not found")
    if(!tags.ok) throw error(500)
    if(!comments.ok) throw error(500)
    
    return {schematic: schematic.data, tags: tags.data, comments: comments.data}
}