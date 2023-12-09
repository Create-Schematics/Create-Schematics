import type { paths } from '$lib/openapiSchema'
import createClient from 'openapi-fetch';


export const { GET, PUT } = createClient<paths>({ baseUrl: import.meta.env.PROD ? "https://createschematics.com/" : "http://localhost:3000" });