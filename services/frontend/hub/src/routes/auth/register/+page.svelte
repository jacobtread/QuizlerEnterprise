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
	import MicrosoftAuthButton from "$lib/components/MicrosoftAuthButton.svelte";

	function onFormSubmit() {}

	let openIDData: (OIDData & { verified: boolean }) | null = null;

	let defaultUsername: string;

	let loading: boolean = false;
	let error: string | null = null;

	// reCaptcha token
	let captchaToken: string | null = null;

	async function onIdentify(token: string, provider: AuthProvider) {
		openIDData = {
			token,
			provider,
			verified: false
		};

		error = null;
		loading = true;

		try {
			const response: OIDConfirmResponse = await openIdConfirm({
				token,
				provider
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
					goto("/auth/login");
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
					<GoogleAuthButton {onIdentify} />
				</li>
				<li>
					<MicrosoftAuthButton {onIdentify} />
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
