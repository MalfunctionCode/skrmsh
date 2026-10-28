<script lang="ts">
	import defaultPfp from '$lib/assets/placeholderPfp.jpg';
	import discord from '$lib/assets/discord-icon.svg';
	import twitch from '$lib/assets/twitch-icon.svg';

	let { data } = $props();
	const created = new Date(data.player.created_at);
	const month = created.toLocaleString('default', { month: 'long' });
</script>

<div id="topRow">
	<div id="playerInfo">
		<img src={defaultPfp} class="pfp" alt="Profile" />
		<div>
			<h1>{data.player.username}</h1>
			<p>Member since {month} {created.getFullYear()}</p>
			<div id="linkBox">
				<img class="links" src={twitch} alt="Twitch Link" />
				<img class="links" src={discord} alt="Discord Tag" />
			</div>
		</div>
	</div>
	<h1>Team here</h1>
</div>

<div id="bottomRow">
    <div id="tierlistBox">
        <h1>Heroes</h1>
        <div id="tierlist">
            <div class="tierHead" id="mainTier">
                <p>Main</p>
            </div>
            <div class="tierContent">
                <p>lalalalal</p>
            </div>
            <div class="tierHead" id="comfortable">
                <p>Comfortable</p>
            </div>
            <div class="tierContent">
                <p>lalalalal</p>
            </div>
            <div class="tierHead" id="learning">
                <p>Learning</p>
            </div>
            <div class="tierContent">
                <p>lalalalal</p>
            </div>
        </div>
    </div>
    <div id="historyBox">
        <h1>History</h1>
        <hr>
    </div>
</div>

<style>
	#topRow {
		width: 100%;
		height: 50%;
		display: flex;
		flex-direction: row;
	}

	#bottomRow{
	    width:100%;
		height:50%;
		display:flex;
		flex-direction:row;
	}

	#playerInfo {
		box-sizing: border-box;
		height: 100%;
		width: 60%;
		padding: 3rem;
		display: flex;
		flex-direction: row;
		gap: 1.5rem;
		background-image: url('$lib/assets/cubes.svg');
		background-repeat: repeat;
	}

	.pfp {
		height: 60%;
		aspect-ratio: 1;
		border-radius: 50%;
		border: 3px solid #fff;
		flex-shrink: 0;
	}

	#linkBox {
		display: flex;
		flex-direction: row;
		width: 100%;
		height: 20%;
		margin-top: 4%;
		gap: 1.5rem;
	}

	.links {
		height: 100%;
		aspect-ratio: 1;
	}

	#tierlistBox{
    	box-sizing: border-box;
        margin-left: 3rem;
        width: 40%;
        height: 80%;
        display: flex;
        flex-direction: column;
        gap: 1rem;
	}

	#tierlist{
	    flex: 1;
		min-height: 0;
	    display:grid;
		gap:1px;
		background-color:#fff;
		grid-template-columns: 20% 80%;
		grid-template-rows: repeat(3, minmax(0, 1fr));
		border:1px solid #fff;
		border-radius:24px 0 0 24px;
	}

	.tierHead{
		text-align: center;
	}

	.tierHead p{
	    color:#000;
	}

	#mainTier{
	    background-color: #39CFBA;
		border-radius: 24px 0 0 0;
	}

	#comfortable{
	    background-color: #8BFF33;
	}

	#learning{
	    background-color: #FF8833;
		border-radius: 0 0 0 24px;
	}

	.tierContent{
	    background-color:#000;
	}

	#historyBox{
	    margin-left:10%;
		width:40%;
	}

	p {
		font-family: 'Roboto', sans-serif;
		font-size: 20px;
		color: #fff;
	}

	h1 {
		margin: 5% 0 0 0;
		font-weight: 200;
	}
</style>
