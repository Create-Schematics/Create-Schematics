<script>
    // TODO: Firebase Integration
</script>


<div class="header">
    <img alt="logo" src="/logo.jpg" class="logo" width="100px"/>

    <div id="title">
        Create Schematics
        <hr/>
    </div>

    <div id="header-buttons">
        <!-- TODO: -->
    </div>
</div>
<nav class="navigation">
    <form>
        <input type="text" placeholder='Search...'/>
        <input id="submit" type="submit" value="Submit"/>
    </form>
</nav>

<style>
    .header {
        background-color: var(--primary-dark);
        height: 100px;
        font-family: 'Minecraftia';
        display: flex;
        -webkit-box-shadow: 0px -10px 5px 0px #000;
        -moz-box-shadow: 0px -10px 5px 0px #000;
        box-shadow: 0px -3px 10px 0px #000;
        z-index: 0;
    }

    .header>#title {
        color: aliceblue;
        font-size: xx-large;
    }

    .navigation {
        background-color: var(--primary);
        height: 30px;
        position: sticky;
        -webkit-box-shadow: 0px -10px 5px 0px #000;
        -moz-box-shadow: 0px -10px 5px 0px #000;
        box-shadow: 0px -3px 10px 0px #000;
    }

</style>