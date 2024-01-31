<!-- Redirection callback page from a completed OAuth -->
<script lang="ts">
	import { AuthProvider, type OIDAuthenticateResponse, openIdAuthenticate } from "$lib/api/auth";
	import { GenericError } from "$lib/api/api";
	import Loader from "$lib/components/Loader.svelte";
	import { setTokenData } from "$lib/stores/auth";
	import { goto } from "$app/navigation";
	import { getErrorMessage, gotoError } from "$lib/error";
	import { onMount } from "svelte";
	import { base } from "$app/paths";

	let loading = true;

	/**
	 * Loads the OpenID response properties from the query
	 * parameters
	 */
	async function loadProperties() {
		// Load the query parameters
		const searchParams: URLSearchParams = new URLSearchParams(window.location.search);

		const error = searchParams.get("error");
		const errorDescription = searchParams.get("error_description");

		if (error !== null) {
			console.log(error, errorDescription);
			gotoError(error, errorDescription ?? "Unknown error occurred", `${base}/auth/login`);
			return;
		}

		// OpenID code
		const code = searchParams.get("code");
		// Auth provider type
		const authProviderRaw: string | null = searchParams.get("state");

		if (code == null || authProviderRaw == null) {
			return;
		}

		const authProvider: AuthProvider = authProviderRaw as AuthProvider;

		// Ensure its a valid auth provider
		if (!Object.values(AuthProvider).includes(authProvider)) {
			console.error("Unknown auth provider");
			return;
		}

		try {
			const response: OIDAuthenticateResponse = await openIdAuthenticate(code, authProvider);

			switch (response.type) {
				// Continue creating account
				case "CreateAccount": {
					const params = new URLSearchParams();
					params.append("token", response.token);
					params.append("provider", authProvider);
					if (response.default_username !== null) {
						params.append("defaultUsername", response.default_username);
					}

					const query = params.toString();

					goto(`${base}/auth/openid/finish?${query}`);

					break;
				}

				// Account link already existed, login
				case "ExistingLinked":
					setTokenData({
						token: response.token,
						refresh_token: response.refresh_token,
						expiry: response.expiry
					});
					goto(`${base}/dashboard`);
					break;
			}
		} catch (e) {
			console.error(e);

			const errorName = e instanceof GenericError ? e.name : "generic";
			gotoError(errorName, getErrorMessage(e), `${base}/auth/login`);
		}

		loading = false;
	}

	onMount(() => {
		loadProperties();
	});
</script>

{#if loading}
	<Loader />
{/if}
