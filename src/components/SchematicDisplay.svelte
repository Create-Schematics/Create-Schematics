<script lang="ts">
    import { getContext } from 'svelte';

    import FaStar from 'svelte-icons/fa/FaStar.svelte'
    import MdFileDownload from 'svelte-icons/md/MdFileDownload.svelte'
    import Tags from './Tags.svelte';
    
    // Post id
    export let postid: string = "";

    // Image reference on server
    export let src: string = "none";
    // Schematic Body
    export let title: string = "Loading...";
    export let rating: number = 5;
    export let description: string = "";
    // Schematic versions
    export let gameVersion: string = "1.18.2";
    export let createVersion: string = "0.5.0.g";
    // Schematic info
    export let author: string = "Author";
    export let downloads: number = 0;
    export let uploaded: string = "1/1/1970"

    let width: number = 0;

</script>

<div class='schematic-card' bind:clientWidth={width}>
    <img class='schematic-image' alt="Schematic"/>

    <div class='schematic-body'>
        <div class='schematic-header'>    
            <h1 id='title'>
                {title}
            </h1>
            <a id='version'>
                {gameVersion} - {createVersion}
            </a>
        </div>
        <div class='rating-container' style:width="{rating*18}px">
            {#each Array(rating) as star}
                <FaStar/>
            {/each}
        </div>

        <Tags 
            tags={['Steampunk', 'Decorative']}
        />

        <p id='description'>
           {description}
        </p>
    </div> 
    {#if width > 800}
        <div class='schematic-info'>
            <b>Author</b>
            <em>{author}</em>
            <hr/>
            <b>Downloads</b>
            <em>
            {downloads} <div id='download-icon'><MdFileDownload/></div>
            </em>
            <hr/>
            <b>Uploaded</b>
            <em>{uploaded}</em>
        </div>
    {/if}
</div>

<style lang="scss">
    .schematic-card {
        font-family: 'Minecraftia';
        display: flex;
        flex-direction: row;
        background-color: var(--white);
        -webkit-box-shadow: 0px -10px 5px 0px #000;
        -moz-box-shadow: 0px -10px 5px 0px #000;
        box-shadow: 0px -3px 10px 0px #000;
        height: 15em;
        border-radius: 10px;
        width: 100%;

        transition: all 0.5s;

        &:hover {
            -webkit-box-shadow: 20px 0px 10px 5px #000;
            -moz-box-shadow:20px 0px 10px 5px #000;
            box-shadow:20px 0px 10px 5px #000;
        }
    }

    .schematic-column {
        display: flex;
        flex-direction: column;
    }

    .schematic-image {
        border-radius: 10px;
        margin: 1em;
        background-color: var(--gray);
        min-width: 14em;
        height: 13em;
    }

    .schematic-body {
        @extend .schematic-column;
        flex-grow: 10;

        > #description {
            background-color: var(--gray);
            border-radius: 10px;
            padding: 0.5em;
            flex-grow: 10;
            overflow-y: hidden;
            text-overflow: ellipsis;
        }
    }

    .schematic-body>.schematic-header {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        margin-top: 1em;

        > h1 {
            font-weight: 600;
            text-shadow: 2px 2px  var(--gray);
            margin: 0;
            color: black;
        }

        > a {
            font-size: large
        }
    }

    .schematic-info {
        @extend .schematic-column;
        justify-content: space-between;
        margin: 1em;
        
        > hr {
            width: 90%;
        }

        > b {
            font-size: large;
            text-shadow: 2px 2px var(--gray);
        }

        > em {
            flex-direction: row;
            font-size: medium;
        }
    }

    #download-icon {
        width: 20px;
        margin: 0;
        padding: 0;
    }

    .rating-container {
        display: flex;
        flex-direction: row;
        align-items: start;
        color: var(--primary);
        height: 16px;
        margin-bottom: 10px;
    }



</style>
