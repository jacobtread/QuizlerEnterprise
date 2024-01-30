<script lang="ts">
	import GoogleAuthButton from "$lib/components/GoogleAuthButton.svelte";
	import {
		type OIDData,
		type OIDConfirmResponse,
		AuthProvider,
		openIdConfirm
	} from "$lib/api/auth";
	import Loader from "$lib/components/Loader.svelte";
	import FinishAccountSetup from "$lib/components/FinishAccountSetup.svelte";
	import Captcha from "$lib/components/Captcha.svelte";
	import { goto } from "$app/navigation";
	import { getErrorMessage } from "$lib/error";

	function onFormSubmit() {}

	let openIDData: (OIDData & { verified: boolean }) | null = null;

	let defaultUsername: string;

	let loading: boolean = false;
	let error: string | null = null;

	// reCaptcha token
	let captchaToken: string | null = null;

	/**
	 * Handles the completion of authentication with Google OpenID
	 */
	async function onGoogleIdentify(response: google.accounts.id.CredentialResponse) {
		const token = response.credential;
		console.debug("Authenticated with Google OpenID", token);

		openIDData = {
			token,
			provider: AuthProvider.Google,
			verified: false
		};

		await verifyOpenId();
	}

	async function verifyOpenId() {
		if (openIDData == null) return;

		error = null;
		loading = true;

		try {
			const response: OIDConfirmResponse = await openIdConfirm({
				token: openIDData.token,
				provider: openIDData.provider
			});
			switch (response.type) {
				// Finish account creation
				case "Success":
					defaultUsername = response.default_username ?? "";
					openIDData.verified = true;
					break;
				// Existing account automatic login
				case "Existing":
					// Account already exists
					goto("/login");
					break;
			}
		} catch (e) {
			error = getErrorMessage(e);
		} finally {
			loading = false;
		}
	}
</script>

{#if openIDData == null}
	<div>
		<form on:submit|preventDefault={onFormSubmit}>
			<h1>Register</h1>
			<p>Enter your details below to create a new account</p>

			{#if error}
				<p class="input-error">{error}</p>
			{/if}

			<!-- <Captcha bind:captchaToken /> -->
		</form>
		<div>
			<p>Or create an account with an alternative method below</p>
			<ul>
				<li>
					<GoogleAuthButton {onGoogleIdentify} />
				</li>
			</ul>
		</div>
	</div>
{:else if openIDData.verified}
	<FinishAccountSetup {openIDData} {defaultUsername} />
{/if}

{#if loading}
	<Loader />
{/if}
