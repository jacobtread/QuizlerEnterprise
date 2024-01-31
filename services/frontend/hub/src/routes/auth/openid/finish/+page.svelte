<!-- Page for completing account creation through OpenID -->
<script lang="ts">
	import { AuthProvider, openIdCreate, type TokenResponse } from "$lib/api/auth";
	import Loader from "$lib/components/Loader.svelte";
	import { setTokenData } from "$lib/stores/auth";
	import { goto } from "$app/navigation";
	import { getErrorMessage } from "$lib/error";
	import { onMount } from "svelte";

	let loading = true;
	let error: string | null = null;

	let token: string | null = null;
	let authProvider: AuthProvider | null = null;

	let username: string = "";
	let password: string = "";

	/**
	 * Loads the OpenID response properties from the query
	 * parameters
	 */
	async function loadProperties() {
		loading = true;

		// Load the query parameters
		const searchParams: URLSearchParams = new URLSearchParams(window.location.search);

		token = searchParams.get("token");

		// Auth provider type
		const authProviderRaw: string | null = searchParams.get("provider");

		if (token == null || authProviderRaw == null) {
			return;
		}

		authProvider = authProviderRaw as AuthProvider;

		// Ensure its a valid auth provider
		if (!Object.values(AuthProvider).includes(authProvider)) {
			console.error("Unknown auth provider");
			return;
		}

		const defaultUsername: string | null = searchParams.get("defaultUsername");
		if (defaultUsername !== null) {
			username = defaultUsername;
		}

		loading = false;
	}

	onMount(() => {
		loadProperties();
	});

	async function onSubmit() {
		if (token == null || authProvider == null) return;

		loading = true;
		error = null;

		try {
			const result: TokenResponse = await openIdCreate(
				{
					token,
					provider: authProvider
				},
				username,
				password
			);

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
