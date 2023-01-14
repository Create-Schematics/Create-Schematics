<script>
    import { browser } from "$app/environment";
    import DATA from "../data/home_page_text";
</script>

<div class='home-screen'>
    <!-- Banner -->
    <div class='home-banner'>
        <div class='title'>
            <img src='/logo.jpg' alt='logo'/>
            <div>
                Create Schematics
                <hr/>
            </div>
        </div>
        <div id='info'>
            <h3>{DATA.BANNER_DATA.subtitle}</h3>
            <hr/>
            <p>{@html DATA.BANNER_DATA.body}</p>
            <!--Search bar-->
            <input type="text" placeholder="Search..."/>
        </div>
    </div>

    <!-- Information -->
    <div>
        {#each DATA.INFORMATION_DATA as card} 
            <div class="home-information-card">
                <h3>{card.title}</h3><hr/>
                <div class='home-information-card-body'>
                    {@html card.body}
                </div>
            </div>
        {/each}
    </div>
</div>


<style lang="scss">


    .home-screen {
        width: 50%;
        max-width: 120ex;
        min-width: 50ex;
        font-family: 'Minecraftia';

        margin: 0;
        background: url("/spruce-planks.png");
        background-position: center top;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        overflow-x: hidden;
        background-size: 50%;
        -ms-interpolation-mode: nearest-neighbor;
        /* Firefox */
        image-rendering: crisp-edges;
        /* Chromium + Safari */
        image-rendering: pixelated;
    }

    .home-banner {
        color: var(--white);
        width: 100%;
        height: auto;
        background: linear-gradient(to top, rgba(77, 124, 165, 0.1), rgba(77, 124, 165, 0.1)), url('/home/banner-image.jpeg') no-repeat center;
        background-position: center;
        background-size: cover;
        text-align: left;

        transition: background-size 0.5s;

        &:hover {
            background-size: 150%;
        }
    }

    .home-banner > .title {
        padding: 15px;
        font-size: xx-large;
        text-shadow: 2px 2px var(--gray-dark);
        display: flex;

        > img {
            margin-right: 10px;
            border-radius: 10px;
            width: 1.5em;
            height: 1.5em;
        }

        div > hr {
            margin: 0;
        }

    }

    .home-banner > #info {
        padding: 15px;
        padding-top: 0;
        font-size: large;

        > * {
            margin: 0;
        }

        > input[type="text"] {
            margin-top: 0.3em;
            font-family: 'Minecraftia';
            color: aliceblue;
            font-size: large;
            padding: 0.45em;
            border-radius: 10px;
            width: 95%;
            border: none;
            background-color: var(--primary);

            &::placeholder {
                color: aliceblue;
            }
        }

        > hr {
            margin-bottom: 10px;
        }

        > h3 {
            text-shadow: 2px 2px #000;
            color: #588acb;
            padding-bottom: 3px;
        }

        > p {
            font-size: 0.95em;
            text-shadow: 1px 1px #808089;
        }
    }

    .home-information-card {
        border-radius: 10px;
        background-color: aliceblue;

        > .home-information-card-body {
            margin: 5px;
            border-radius: 10px;
            background-color: var(--gray);
        }

        > hr {
            margin: 0;
        }
        
        > h3 {
            margin: 0;
            text-shadow: 2px 2px #000;
            color: #588acb;
        }
    }

</style>