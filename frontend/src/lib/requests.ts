import { Apity } from '@cocreators-ee/apity'
import type { paths } from '$lib/openapiSchema'

const apity = Apity.for<paths>()

// apity.configure({
//     baseUrl: ''
// })

export const getSchematics = apity.path('/api/v1/schematics')
    .method('get')
    .create()

export const getSchematic = apity.path('/api/v1/schematics/{id}')
.method('get')
.create()
