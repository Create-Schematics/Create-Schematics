interface InformationData {
    title: string;
    body: string;
}

const TITLE = "Create Schematics"

const BANNER_DATA = {
    subtitle: "Share, Discover, Download Schematics",
    body: "<p>Easily find and share schematics for Create through our schematics platform built for ease of use and speed. With full ingame integration with our companion modWIP for Forge & Fabric</p>"
}

const INFORMATION_DATA: InformationData[] = [
    {
        title: "Companion Mod",
        body: "<p>With our companion mod, coming soon for version 1.18.2 and 1.19.2 and both forge and fabric, quickly share, search for, and download schematics without ever needing to leave the game for a seamless and connected experience.<p>"
    },
    {
        title: "How To Use Schematics",
        body: "<p><details><summary>Creating Schematics</summary><hr/><p>Using a schematic & quill select the bounding areas of your creation then right click to save it, make sure to give it a memorable and descriptive name so you can easily find it later</p><a href='https://www.youtube.com/watch?v=vFEkajTVCg8&t=80s'>Video Tutorial</a></details><details><summary>Sharing Schematics</summary><hr/></details><details><summary>Using Schematics Ingame</summary><hr/><p>Once saved or downloaded, schematics can be loaded at a schematic table and then positioned as a hologram projection ingame. In creative mode this projection can be placed instantly or in survival it can be assembled similar to mods such as litematica using the projection as a guide or using the schemati-cannon fueled with gunpowder which will automatically pull the required resources from nearby inventories</p></details></p>"
    },
    {
        title: "Related Projects",
        body: "<p><img class='network-logo' alt='discord' src='https://imgs.search.brave.com/mbonb_vlURUFhHAburC1eb5XVz7fClX3RU7BZd3qU-I/rs:fit:1200:1200:1/g:ce/aHR0cHM6Ly9sb2dv/ZG93bmxvYWQub3Jn/L3dwLWNvbnRlbnQv/dXBsb2Fkcy8yMDE4/LzAyL3JlZGRpdC1s/b2dvLTE2LnBuZw'/><b>Reddit</b> <br/>- <a href='https://www.reddit.com/r/CreateSchematics/'>r/CreateSchematics</a> <br/>- <a href='https://www.reddit.com/r/Createmod/'>r/Createmod</a></p><p><img class='network-logo' alt='discord' src='https://imgs.search.brave.com/-x3KIcNLMqF5IZP6pLQeEhkerEvSJdFlf_zsKX-Ht-4/rs:fit:1200:1200:1/g:ce/aHR0cHM6Ly9jbGlw/Z3JvdW5kLmNvbS9p/bWFnZXMvZGlzY29y/ZC1sb2dvLXBuZy0z/LnBuZw'/><b>Discord</b> <br/>- <a href='https://www.reddit.com/r/CreateSchematics/'>Create Discord</a> <br/>- <a href='https://www.reddit.com/r/CreateSchematics/'>Create Addon Hub</a></p>"
    }
]

const DATA = {
    TITLE,
    BANNER_DATA,
    INFORMATION_DATA
}

export default DATA;