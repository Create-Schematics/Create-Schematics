import { getSchematic, getSchematicComments, getSchematicTags } from '$lib/requests.js'
import { error } from '@sveltejs/kit'

export const load = async ({ params }) => {
    //TODO: remove this comment when the schema is updated
    //@ts-ignore this is a temporary bug in the schema
    const schematic = await getSchematic(params).result
    const tags = await getSchematicTags(params).result
    const comments = await getSchematicComments({...params, query: {}}).result
    if(!schematic.ok) throw error(404, "Schematic not found")
    if(!tags.ok) throw error(500)
    if(!comments.ok) throw error(500)
    
    return {schematic: schematic.data, tags: tags.data, comments: comments.data}
}