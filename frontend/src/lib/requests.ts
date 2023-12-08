import { Apity } from '@cocreators-ee/apity'
import type { paths } from '$lib/openapiSchema'

const apity = Apity.for<paths>()

console.log(import.meta.env.BASE_URL)

apity.configure({
    //I'm not sure what the production api url is going to be
    baseUrl: import.meta.env.PROD ? "https://api.createschematics.com/" : "http://localhost:3000",
})

//Schematics

export const getSchematics = apity.path('/api/v1/schematics')
    .method('get')
    .create()

export const getSchematic = apity.path('/api/v1/schematics/{id}')
    .method('get')
    .create()

export const getSchematicTags = apity.path('/api/v1/schematics/{id}/tags')
    .method('get')
    .create()

export const getSchematicComments = apity.path('/api/v1/schematics/{id}/comments')
    .method('get')
    .create()

//Users

export const getCurrentUser = apity.path('/api/v1/users')
    .method('get')
    .create()