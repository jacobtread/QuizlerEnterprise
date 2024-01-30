<script lang="ts">
	import { PUBLIC_APP_BASE_URL, PUBLIC_MICROSOFT_OPENID_CLIENT_ID } from "$env/static/public";
	import { AuthProvider } from "$lib/api/auth";
	import {
		PublicClientApplication,
		type AuthenticationResult,
		type PopupRequest
	} from "@azure/msal-browser";
	import Icon from "@iconify/svelte";

	/**
	 * Callback to provide the OpenID identity token and provider
	 * type to the outer component
	 */
	export let onIdentify: (token: string, provider: AuthProvider) => Promise<void>;

	// Microsoft auth authority
	const AUTHORITY: string = "https://login.microsoftonline.com/common/";

	// Create the instance of MSAL
	const msalInstance = new PublicClientApplication({
		auth: {
			clientId: PUBLIC_MICROSOFT_OPENID_CLIENT_ID,
			redirectUri: PUBLIC_APP_BASE_URL,
			authority: AUTHORITY
		}
	});

	// Scopes to request from Microsoft when authenticating
	const LOGIN_SCOPES: string[] = ["user.read", "email"];
	// Login request properties
	const LOGIN_REQUEST: PopupRequest = {
		scopes: LOGIN_SCOPES,
		redirectUri: PUBLIC_APP_BASE_URL
	};

	async function doLogin() {
		try {
			await msalInstance.initialize();
			const response: AuthenticationResult = await msalInstance.loginPopup(LOGIN_REQUEST);
			console.debug("Authenticated with Microsoft");

			await onIdentify(response.idToken, AuthProvider.Microsoft);
		} catch (e) {
			console.error("Error authenticating with Microsoft", e);
		}
	}
</script>

<button on:click={doLogin} class="button">
	<Icon icon="logos:microsoft-icon" class="button__icon" />
	Sign-in with
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
