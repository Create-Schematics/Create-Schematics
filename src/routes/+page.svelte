<script>

</script>

<home>
    <div class="banner">
        <h1>Create <br/> Schematics</h1>
        <p>Scroll</p>
    </div>

    <div class="step-one">
        <h2>Step: 1</h2>
        <h3>Build something cool</h3>
    </div>


    <div class="step-two">
        <h2>Step: 2</h2>
        <h3>Schematic-ify</h3>
    </div>


    <div class="step-three">
        <h2>Step: 3</h2>
        <h3>Show us!</h3>
    </div>
</home>

<style lang="scss">
    home {
        overflow-y: scroll;
        
        div:nth-child(odd) {
            background-color: var(--primary);
            color: var(--primary-accent);
        }
        div:nth-child(even) {
            background-color: var(--primary-accent);
            color: var(--primary);
        }

        > div {
            height: 50em;
        }
    }

    .banner {
        h1, h3 {
            font-weight: 200;
            color: aliceblue;
            font-size: 5em;
            text-align: center;
            text-decoration: underline;
        }

        p {
            text-align: center;
            width: 100%;
            margin: auto;
            color: var(--white);
        }
    }
</style>