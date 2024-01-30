<script lang="ts">
	import { PUBLIC_GOOGLE_OPENID_CLIENT_ID } from "$env/static/public";
	import Icon from "@iconify/svelte";

	export let onGoogleIdentify: (response: google.accounts.id.CredentialResponse) => Promise<void>;

	// HTML parent for the google button to render within
	let googleButton: HTMLDivElement;

	/**
	 * Handles creating the Google login button when the client
	 * script is loaded
	 */
	function onScriptLoaded() {
		// Initialize Google ID context
		google.accounts.id.initialize({
			client_id: PUBLIC_GOOGLE_OPENID_CLIENT_ID,
			callback: onGoogleIdentify
		});

		// Render google auth button
		google.accounts.id.renderButton(googleButton, {
			type: "standard",
			text: "signin",
			size: "large",
			width: 400,
			theme: "outline"
		});
	}

	function doLogin() {
		googleButton.click();
	}
</script>

<!-- Include Google Client script -->
<svelte:head>
	<script
		src="https://accounts.google.com/gsi/client"
		async
		defer
		nonce="google-client"
		on:load={onScriptLoaded}
	></script>
</svelte:head>

<div class="hidden" bind:this={googleButton} />

<button on:click={doLogin} class="button">
	<Icon icon="logos:google-icon" class="button__icon" />
	Login With Google
</button>

<style lang="scss">
	.button {
		display: flex;
		gap: 1rem;
		padding: 1rem;
		border: 1px solid #ccc;
		background-color: #f7f7f7;
		border-radius: 0.2rem;
		font-size: 1rem;
		text-align: left;
		cursor: pointer;
	}

	.button__icon {
		width: 4rem;
	}

	.hidden {
		display: none;
	}
</style>
