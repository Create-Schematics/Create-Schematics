import type { components } from "./openapiSchema";

export interface User {
    id: string;
    username: string;
    dateJoined: Date;
    avatar: string;
    links : {
        url: string;
        name: string;
    }[];
    totalDownloads: number;
    description: string;
}

export interface Collection {
    id: string;
    tags: string[];
    creationDate: Date;
    title: string;
    views: number;
    likes: number;
    dislikes: number;
    author: string;
    schematicIds: string[];
}