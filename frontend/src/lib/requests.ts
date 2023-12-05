import type { Schematic, Collection } from "$lib/types";

export async function getCollection(collectionId: string): Promise<Collection> {
    const res = await fetch(`/api/v1/schematics/collection/${collectionId}`, {
        method: 'GET'
    })
    
    const json = await res.json()
    return JSON.parse(json);
}

export async function getSchematic(schematicId: string): Promise<Schematic> {
    const res = await fetch(`/api/v1/schematic/${schematicId}`, {
        method: 'GET'
    })
    
    const json = await res.json()
    return JSON.parse(json);
}