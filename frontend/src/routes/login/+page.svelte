<script lang="ts">
	import { goto } from '$app/navigation';
	import { player } from '$lib/stores/auth';
	import type { CreatePlayerRequest } from '$lib/api/schema';

	let unameRegex = /^[a-zA-Z0-9_-]{3,16}$/;

	let is_register = $state(false);

	let uname = $state('');
	let pass = $state('');

	let mode = $derived.by(() => {
		if (is_register) {
			return 'Register';
		} else {
			return 'Login';
		}
	});

	let button_disabled = $derived.by(() => {
		return unameRegex.test(uname) && pass.length > 3 && pass.length < 15 ? false : true;
	});

	async function handleLogin() {
		let req: CreatePlayerRequest = {
			username: uname,
			password: pass
		};

		if (is_register) {
			const res = await fetch('/api/players', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(req)
			});

			if (res.ok) {
				const data = await res.json();
				player.set(data);
				goto('/');
			}
		} else {
			const res = await fetch('/api/players/verify', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(req)
			});

			if (res.ok) {
				const data = await res.json();
				player.set(data);
				goto('/');
			}
		}
	}
</script>

<div id="main">
	<h1>{mode}</h1>
	<input type="text" id="uname" placeholder="Username..." bind:value={uname} />
	<input type="password" id="pass" placeholder="Password..." bind:value={pass} />
	<button onclick={handleLogin} disabled={button_disabled}>{mode}</button>
	<div style="display:flex; flex-direction: row;">
		<p>New here?</p>
		<p id="register" onclick={() => (is_register = !is_register)}>Register</p>
	</div>
</div>

<style>
	#main {
		height: 40vh;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: center;
	}

	input {
		width: 20vw;
		height: 3vh;
		font-family: 'Roboto', sans-serif;
		color: #fff;
		background-color: #000;
		border: 2px solid #fff;
		border-radius: 4px;
	}

	input:focus {
		outline: none;
		border: 3px solid rgba(29, 255, 251, 1);
	}

	p {
		color: #fff;
		font-size: 14px;
	}

	#register {
		color: rgba(255, 136, 51, 1);
		font-size: 14px;
		cursor: pointer;
	}
</style>
