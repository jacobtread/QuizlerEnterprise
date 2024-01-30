<script lang="ts">
	import { PUBLIC_GOOGLE_OPENID_CLIENT_ID } from "$env/static/public";
	import { AuthProvider } from "$lib/api/auth";
	import GoogleIcon from "$lib/components/icons/GoogleIcon.svelte";

	/**
	 * Callback to provide the OpenID identity token and provider
	 * type to the outer component
	 */
	export let onIdentify: (token: string, provider: AuthProvider) => Promise<void>;

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
			callback: (response: google.accounts.id.CredentialResponse) => {
				console.debug("Authenticated with Google");
				onIdentify(response.credential, AuthProvider.Google);
			}
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
	<GoogleIcon class="button__icon" />
	Sign-in with Google
</button>

<style lang="scss">
	.button {
		display: flex;
		gap: 1rem;
		padding: 0.75rem 1rem;
		border: 1px solid #ccc;
		background-color: #f7f7f7;
		border-radius: 0.2rem;
		font-size: 1rem;
		text-align: left;
		width: 100%;
		cursor: pointer;
	}

	.button__icon {
		width: 4rem;
	}
</style>
