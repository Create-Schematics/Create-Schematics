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

export interface Comment {
    id: string;
    author: string;
    text: string;
}

export interface Schematic {
    id: string;
    tags: string[];
    uploadDate: Date;
    title: string;
    images: string[];
    downloads: number;
    likes: number;
    dislikes: number;
    views: number;
    author: string;
}

export interface SchematicDetails extends Schematic {
    description: string;
    comments: Comment[];
    mods: string[];
    file: string;
}