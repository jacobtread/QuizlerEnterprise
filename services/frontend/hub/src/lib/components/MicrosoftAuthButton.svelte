<script lang="ts">
	import { PUBLIC_APP_BASE_URL, PUBLIC_MICROSOFT_OPENID_CLIENT_ID } from "$env/static/public";
	import { PublicClientApplication, type AuthenticationResult } from "@azure/msal-browser";
	import Icon from "@iconify/svelte";

	const msalInstance = new PublicClientApplication({
		auth: {
			clientId: PUBLIC_MICROSOFT_OPENID_CLIENT_ID,
			redirectUri: PUBLIC_APP_BASE_URL,
			authority: "https://login.microsoftonline.com/common/"
		}
	});

	export let onMicrosoftIdentify: (response: AuthenticationResult) => Promise<void>;

	async function doLogin() {
		await msalInstance.initialize();

		msalInstance
			.loginPopup({
				scopes: ["user.read", "email"],
				redirectUri: PUBLIC_APP_BASE_URL
			})
			.then(onMicrosoftIdentify)
			.catch((error) => {
				console.error(error);
			});
	}
</script>

<button on:click={doLogin} class="button">
	<Icon icon="logos:microsoft-icon" class="button__icon" />
	Login With Microsoft
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
