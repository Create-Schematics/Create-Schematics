import { GET } from "$lib/requests";

export const prerender = true;
export const trailingSlash = 'always';
export const ssr = true;

export async function load(){
    const request = GET("/v1/users", {
        credentials: "include"
    })
    const resp = (await request).data
    return {currentUser: resp}
}