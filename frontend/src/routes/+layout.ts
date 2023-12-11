import { GET } from "$lib/requests";


export const prerender = true;

export async function load(){
    const request = GET("/api/v1/users")
    const resp = (await request).data
    return {user: resp}
}