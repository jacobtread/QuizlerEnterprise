<!-- Screen for completing the account creation process of an OpenID account -->
<script lang="ts">
	import { goto } from "$app/navigation";
	import { openIdCreate, type TokenResponse, type OIDData } from "$lib/api/auth";
	import { getErrorMessage } from "$lib/error";
	import { setTokenData } from "$lib/stores/auth";
	import Loader from "./Loader.svelte";

	export let openIDData: OIDData;
	export let defaultUsername: string;

	let loading: boolean = false;
	let error: string | null = null;

	let username: string = defaultUsername;
	let password: string;

	async function onSubmit() {
		loading = true;
		error = null;

		try {
			const result: TokenResponse = await openIdCreate(openIDData, username, password);

			setTokenData(result);
			goto("/dashboard");
		} catch (e) {
			error = getErrorMessage(e);
			return;
		} finally {
			loading = false;
		}
	}
</script>

<form on:submit|preventDefault={onSubmit}>
	<h1>Finish Account Setup</h1>
	<p>Please enter a username and password to complete the account creation process</p>

	{#if error}
		<p class="input-error">{error}</p>
	{/if}

	<div class="input-list">
		<label class="input">
			<span class="input__name">Username</span>
			<input
				class="input__value"
				type="text"
				required
				bind:value={username}
				minlength="3"
				maxlength="99"
				autocomplete="off"
			/>
		</label>
		<label class="input">
			<span class="input__name">Password</span>
			<input
				class="input__value"
				type="password"
				required
				bind:value={password}
				minlength="4"
				maxlength="100"
				autocomplete="current-password"
			/>
		</label>
	</div>
	<button type="submit" class="submit">Create</button>
</form>

{#if loading}
	<Loader />
{/if}
