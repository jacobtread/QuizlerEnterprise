<script lang="ts">
	import {
		openIdProviders,
		type OIDProvidersResponse,
		AuthProvider,
		registerBasic
	} from "$lib/api/auth";
	import Loader from "$lib/components/Loader.svelte";
	import AuthProviderButton from "$lib/components/auth/AuthProviderButton.svelte";
	import Logo from "$lib/components/icons/Logo.svelte";
	import { onMount, type ComponentType } from "svelte";
	import { base } from "$app/paths";
	import z from "zod";
	import GoogleIcon from "$lib/components/icons/GoogleIcon.svelte";
	import MicrosoftIcon from "$lib/components/icons/MicrosoftIcon.svelte";
	import CaptchaContext, { getCaptchaToken } from "$lib/components/CaptchaContext.svelte";
	import { goto } from "$app/navigation";
	import { setTokenData } from "$lib/stores/auth";
	import { createForm } from "$lib/stores/form";
	import TextInput from "$lib/components/input/TextInput.svelte";

	interface ProviderButtonData {
		icon: ComponentType;
		text: string;
	}

	const PROVIDER_BUTTON_DATA: Record<AuthProvider, ProviderButtonData> = {
		[AuthProvider.Google]: { icon: GoogleIcon, text: "Sign-up with Google" },
		[AuthProvider.Microsoft]: { icon: MicrosoftIcon, text: "Sign-up with Microsoft" }
	};

	type ProviderData = { url: string } & ProviderButtonData;

	let providers: ProviderData[] = [];

	async function loadProviders() {
		providers = [];

		try {
			const response: OIDProvidersResponse = await openIdProviders();

			for (const [key, value] of Object.entries(response.providers)) {
				let buttonData = PROVIDER_BUTTON_DATA[key as AuthProvider];

				providers.push({
					icon: buttonData.icon,
					text: buttonData.text,
					url: value.auth_url
				});
			}

			providers = providers;
		} catch (e) {
			console.error("Failed to load auth providers");
		}
	}

	let username: string = "";
	let email: string = "";
	let password: string = "";

	const { errors, loading, submit } = createForm(onFormSubmit);

	// Schema for validating the request data
	const schema = z.object({
		username: z
			.string()
			.trim()
			.toLowerCase()
			.min(4)
			.max(100)
			// Username alphanumeric validation
			.regex(new RegExp("^([a-zA-Z0-9]+)$"), {
				message: "Username must only contain letters and numbers"
			}),
		email: z.string().trim().toLowerCase().email(),
		password: z.string().trim().min(4).max(100)
	});

	async function onFormSubmit() {
		const body = schema.parse({ username, email, password });
		const captchaToken = await getCaptchaToken();
		const response = await registerBasic(body, captchaToken);

		setTokenData({
			token: response.token,
			refresh_token: response.refresh_token,
			expiry: response.expiry
		});

		goto(`${base}/dashboard`);
	}

	onMount(loadProviders);

	$: {
		username = username.toLowerCase();
		email = email.toLowerCase();
	}
</script>

<CaptchaContext />

<main class="main">
	<div class="content">
		<div class="panel">
			<form on:submit|preventDefault={submit} class="form">
				<h1 class="title">Register</h1>
				<p class="text">Enter your details below to register an account</p>

				{#if $errors["base"]}
					<p class="input-error">{$errors["base"]}</p>
				{/if}

				<TextInput
					label="Username"
					type="text"
					id="username"
					autocomplete="username"
					required
					error={$errors["username"]}
					minlength="4"
					maxlength="100"
					bind:value={username}
				/>

				<p class="text text--small">
					Username must be all lowercase containing only letters and numbers
				</p>

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
					autocomplete="new-password"
					required
					error={$errors["password"]}
					minlength="4"
					maxlength="100"
					bind:value={password}
				/>

				<p class="text text--small">Password must be between 4 and 100 characters</p>

				<button class="button" type="submit">Register</button>

				<a href="{base}/auth/login" class="switch">Already have an account? Login</a>
			</form>
			<div>
				<p class="text">Or register with an alternative method below</p>
				<ul class="methods">
					{#each providers as provider}
						<li>
							<AuthProviderButton icon={provider.icon} text={provider.text} url={provider.url} />
						</li>
					{/each}
				</ul>
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

		&--small {
			font-size: 0.8rem;
		}
	}

	.methods {
		list-style: none;
		display: flex;
		gap: 1rem;
		flex-flow: column;
		margin-top: 1rem;
	}
</style>
