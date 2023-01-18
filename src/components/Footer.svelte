<script>
    import FOOTER from "../data/footer_data";
</script>

<div class="footer">
    {#each FOOTER as footerColumn}
        <div class="footer-column">
            <h3>{footerColumn.heading}</h3> <hr/>
            {@html footerColumn.body}
        </div>
    {/each}
</div>

<style lang="scss">

    .footer {
        font-size: large;
        display: flex;
        flex-direction: row;

        > div {
            margin: 20px;
            width: 33%;
        }
    }

    @media only screen and (max-width: 800px) { .footer {
        font-size: large;
        display: flex;
        flex-direction: column;

        > div {
            width: 100%;
            border-radius: 20px;
            margin: 20px;
        }
    }}

</style>