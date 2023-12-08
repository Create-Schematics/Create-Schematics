import { getCurrentUser } from "$lib/requests";

export const prerender = true;

export async function load(){
    const request = getCurrentUser({})
    const resp = await request.result
    return {user: resp}
}