<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import defaultPfp from '$lib/assets/placeholderPfp.jpg'



	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<nav>
    <h1>SKRMSH</h1>
    <div id="navLinks">
        <div class="subgroup">
        <h2>Teams</h2>
        <i class="fa fa-chevron-right"></i>
        </div>
        <div class="subgroup">
        <h2>Players</h2>
        <i class="fa fa-chevron-right"></i>
        </div>
        <div class="subgroup">
        <h2>Scrims</h2>
        <i class="fa fa-chevron-right"></i>
        </div>
    </div>
    <div id="userElements">
        <div class="subgroup">
            <img src={defaultPfp} alt="Team"/>
            <h2>Team</h2>
        </div>
        <div class="subgroup">
            <img src={defaultPfp} alt="Profile"/>
            <h2>Profile</h2>
        </div>
    </div>
</nav>
<hr/>

{@render children()}

<style>
    nav{
        display:flex;
        flex-direction: row;
        width:100%;
        height:9%;
        padding: 1% 2%;
        margin:0;
        align-items: center;
    }

    h1{
        border-right: 2px solid #fff;
        padding-right:2%;
        margin: 0;
    }

    h2{
        color:#fff;
        font-family: "Play", sans-serif;
        font-weight:400;
        font-size:30px;
        margin:0;
    }

    i{
        color:#fff;
        margin-left:10%;
        flex-shrink: 0;
        font-size:10px;

    }

    img{
        height:80%;
        width:20%;
        clip-path: circle(50%);
        margin-right:8%;
    }

    hr{
        width:100%;
        background: linear-gradient(90deg,rgba(29, 255, 251, 1) 0%, rgba(255, 136, 51, 1) 100%);
        height:4px;
        border:0;
    }

    .subgroup{
        display:flex;
        flex-direction:row;
        align-items:center;
    }

    #navLinks{
        display:flex;
        flex-direction:row;
        justify-content:space-between;
        width:30%;
        padding: 0 3%;
    }

    #userElements{
        width:30%;
        display:flex;
        flex-direction: row;
        align-items:center;
        margin-left:20%;
    }

    :global(body){
        margin: 0;
        background-color: #000;
    }
    :global(h1){
        font-family: "Play", sans-serif;
        font-weight: 600;
        font-size: 50px;
        letter-spacing: 10%;
        font-style:normal;
        color:#fff;
    }
</style>
