<script lang="ts">
	import { loginBasic } from "$lib/api/auth";
	import Loader from "$lib/components/Loader.svelte";
	import CaptchaContext, { getCaptchaToken } from "$lib/components/CaptchaContext.svelte";
	import Logo from "$lib/components/icons/Logo.svelte";
	import { base } from "$app/paths";
	import { setTokenData } from "$lib/stores/auth";
	import { goto } from "$app/navigation";
	import { createForm } from "$lib/stores/form";
	import TextInput from "$lib/components/input/TextInput.svelte";
	import z from "zod";
	import AuthProviders from "$lib/components/auth/AuthProviders.svelte";

	let email: string = "";
	let password: string = "";

	const { errors, loading, submit } = createForm(onFormSubmit);

	// Schema for validating the request data
	const schema = z.object({
		email: z.string().trim().toLowerCase().email(),
		password: z.string().trim().min(4).max(100)
	});

	async function onFormSubmit() {
		const body = schema.parse({ email, password });
		const captchaToken = await getCaptchaToken();
		const response = await loginBasic(body, captchaToken);

		setTokenData({
			token: response.token,
			refresh_token: response.refresh_token,
			expiry: response.expiry
		});
		goto(`${base}/`);
	}
</script>

<CaptchaContext />

<main class="main">
	<div class="content">
		<div class="panel">
			<form on:submit|preventDefault={submit} class="form">
				<h1 class="title">Login</h1>
				<p class="text">Enter your details below</p>

				{#if $errors["base"]}
					<p class="input-error">{$errors["base"]}</p>
				{/if}

				<TextInput
					label="Email"
					type="text"
					id="email"
					autocomplete="email"
					required
					error={$errors["email"]}
					bind:value={email}
				/>

				<TextInput
					label="Password"
					type="password"
					id="password"
					autocomplete="password"
					required
					error={$errors["password"]}
					bind:value={password}
				/>

				<a href="/" class="forgot">Forgot password?</a>

				<button class="button">Login</button>

				<a href="{base}/auth/register" class="switch">Don't have an account? Register</a>
			</form>
			<div>
				<p class="text">Or login with an alternative method below</p>

				<AuthProviders buttonPrefix="Sign-in" />
			</div>
		</div>
		<div class="logo-wrapper">
			<Logo textFill="#ffffff" bgFill="#666" class="logo" />
			<p class="tagline">Powerful Quizzes without the extra hassle</p>
		</div>
	</div>
</main>

{#if $loading}
	<Loader />
{/if}

<style lang="scss">
	.main {
		background: no-repeat url("/background-waves.svg");
		background-position: center;
		background-size: cover;
		width: 100vw;
		height: 100vh;
	}

	.logo-wrapper :global(.logo) {
		width: 320px;
		height: auto;
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
		cursor: pointer;
	}

	.forgot,
	.switch {
		margin-bottom: 0.5rem;
		font-size: 0.9rem;
		color: #426391;
	}

	.switch {
		margin-top: 0.5rem;
	}

	.form {
		display: flex;
		flex-flow: column;
		gap: 0.25rem;
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
		padding: 1rem;
		border-radius: 0.5rem;
		background-color: #ffffff;
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
</style>
