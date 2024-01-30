<script lang="ts">
	import { PUBLIC_GOOGLE_OPENID_CLIENT_ID } from "$env/static/public";

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

<div class="google" bind:this={googleButton} />

<style lang="scss">
	.google {
		margin-top: 0.5rem;
		overflow: hidden;
	}
</style>
