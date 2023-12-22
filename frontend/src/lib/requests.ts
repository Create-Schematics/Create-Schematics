import type { paths } from '$lib/openapiSchema'
import createClient from 'openapi-fetch';

export const apiBaseUrl = import.meta.env.PROD ? "https://createschematics.com/" : "http://localhost:3000"


export const { GET, POST } = createClient<paths>({ baseUrl: apiBaseUrl });