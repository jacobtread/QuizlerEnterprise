<script lang="ts">
	import { AuthProvider, openIdLogin, type TokenResponse } from "$lib/api/auth";
	import Loader from "$lib/components/Loader.svelte";
	import Captcha from "$lib/components/Captcha.svelte";
	import { goto } from "$app/navigation";
	import { getErrorMessage } from "$lib/error";
	import { setTokenData } from "$lib/stores/auth";
	import MicrosoftAuthButton from "$lib/components/auth/button/MicrosoftAuthButton.svelte";
	import GoogleAuthButton from "$lib/components/auth/button/GoogleAuthButton.svelte";
	import Logo from "$lib/components/icons/Logo.svelte";

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

<main class="main">
	<div class="content">
		<div class="panel">
			<form on:submit|preventDefault={onFormSubmit} class="form">
				<h1 class="title">Login</h1>
				<p class="text">Enter your details below</p>

				{#if error}
					<p class="input-error">{error}</p>
				{/if}

				<label for="email">Email <span class="required">*</span></label>
				<input type="text" id="email" autocomplete="email" required />
				<label for="password">Password <span class="required">*</span></label>
				<input type="password" id="password" autocomplete="new-password" required />

				<a href="/" class="forgot">Forgot password?</a>

				<Captcha bind:captchaToken />

				<button class="button">Login</button>
			</form>
			<div>
				<p class="text">Or login with an alternative method below</p>
				<ul class="methods">
					<li>
						<GoogleAuthButton {onIdentify} />
					</li>
					<li>
						<MicrosoftAuthButton {onIdentify} />
					</li>
				</ul>
			</div>
			{#if loading}
				<Loader />
			{/if}
		</div>
		<div class="logo-wrapper">
			<Logo fill="#999" />
			<p class="tagline">Powerful Quizzes without the extra hassle</p>
		</div>
	</div>
</main>

<style lang="scss">
	.main {
		background: no-repeat url("/background-waves.svg");
		background-position: center;
		background-size: cover;
		width: 100vw;
		height: 100vh;
	}

	.button {
		display: block;
		padding: 0.75rem 0.5rem;
		background-color: #0464e8;
		border: none;
		border-radius: 0.2rem;
		color: #fff;
		font-weight: bold;
		font-size: 1.2rem;
	}

	.forgot {
		margin-bottom: 0.5rem;
		font-size: 0.9rem;
		color: #426391;
	}

	label {
		font-weight: bold;
		display: block;
		margin-bottom: 0.5rem;
		margin-top: 0.5rem;
		color: #666;
		font-size: 1rem;
	}

	.required {
		color: #e06363;
	}

	.form {
		display: flex;
		flex-flow: column;
		gap: 0.25rem;
	}

	input {
		padding: 0.75rem 1rem;
		display: block;
		width: 100%;
		margin-bottom: 0.5rem;
		border: 1px solid #aaa;
		border-radius: 0.2rem;
		font-size: 1rem;
	}

	.content {
		width: 100%;
		height: 100%;

		max-width: 84rem;
		margin: 0 auto;

		display: flex;
		flex-flow: row;

		justify-content: center;
		align-items: center;
	}

	.tagline {
		margin-top: 1rem;
		color: #999;
	}

	.logo-wrapper {
		display: flex;
		align-items: center;
		justify-content: center;
		flex: auto;
		flex-flow: column;
	}

	.panel {
		max-width: 28rem;
		width: 100%;
		background: #fff;

		border: 1px solid #ccc;

		padding: 2rem;
		display: flex;
		flex-flow: column;
		justify-content: center;

		border-radius: 0.2rem;

		gap: 1rem;
	}

	.title {
		margin-bottom: 1rem;
		font-size: 2rem;
		font-weight: 600;
		color: #444;
	}

	.text {
		color: #555;
		margin-bottom: 0.5rem;
	}

	.methods {
		list-style: none;
		display: flex;
		gap: 1rem;
		flex-flow: column;
		margin-top: 1rem;
	}
</style>
