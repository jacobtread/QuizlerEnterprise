<script lang="ts">
	import GoogleAuthButton from "$lib/components/GoogleAuthButton.svelte";
	import { type OIDData, AuthProvider, openIdLogin, type TokenResponse } from "$lib/api/auth";
	import Loader from "$lib/components/Loader.svelte";
	import FinishAccountSetup from "$lib/components/FinishAccountSetup.svelte";
	import Captcha from "$lib/components/Captcha.svelte";
	import { goto } from "$app/navigation";
	import { getErrorMessage } from "$lib/error";
	import { setTokenData } from "$lib/stores/auth";
	import MicrosoftAuthButton from "$lib/components/MicrosoftAuthButton.svelte";

	function onFormSubmit() {}

	let loading: boolean = false;
	let error: string | null = null;

	// reCaptcha token
	let captchaToken: string | null = null;

	async function onIdentify(token: string, provider: AuthProvider) {
		error = null;
		loading = true;

		try {
			const response: TokenResponse = await openIdLogin({
				token,
				provider
			});
			setTokenData(response);
			goto("/dashboard");
		} catch (e) {
			error = getErrorMessage(e);
		} finally {
			loading = false;
		}
	}
</script>

<div>
	<form on:submit|preventDefault={onFormSubmit}>
		<h1>Login</h1>
		<p>Enter your details below</p>

		{#if error}
			<p class="input-error">{error}</p>
		{/if}

		<!-- <Captcha bind:captchaToken /> -->
	</form>
	<div>
		<p>Or login with an alternative method below</p>
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

{#if loading}
	<Loader />
{/if}
