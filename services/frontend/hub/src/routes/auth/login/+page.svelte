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

	const { data, errors, loading, submit } = createForm({
		// The form submission handler
		submitAction: async (data) => {
			const captchaToken = await getCaptchaToken();
			const response = await loginBasic(data, captchaToken);

			setTokenData({
				token: response.token,
				refresh_token: response.refresh_token,
				expiry: response.expiry
			});
			goto(`${base}/`);
		},
		// The default form data
		defaultData: { email: "", password: "" },
		// Schema for validating the form data
		schema: z.object({
			email: z.string().trim().toLowerCase().email(),
			password: z.string().trim().min(4).max(100)
		})
	});
</script>

<CaptchaContext />

<main
	class="main bg-[url('/background-waves.svg')] bg-no-repeat bg-center bg-cover w-screen h-screen"
>
	<div class="flex flex-row items-center justify-center w-full h-full max-w-7xl mx-auto">
		<div
			class="max-w-md w-full bg-white border-gray-300 border-2 p-8 flex flex-col justify-center rounded-sm gap-4"
		>
			<form on:submit|preventDefault={submit} class="flex flex-col gap-1">
				<h1 class="mb-4 text-3xl font-semibold text-gray-800">Login</h1>
				<p class="text-gray-600 mb-2">Enter your details below</p>

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
					bind:value={$data.email}
				/>

				<TextInput
					label="Password"
					type="password"
					id="password"
					autocomplete="password"
					required
					error={$errors["password"]}
					bind:value={$data.password}
				/>

				<a href="/" class="mb-2 text-sm text-blue-800">Forgot password?</a>

				<button
					class="button block px-3 py-2 bg-blue-600 border-none text-white font-bold text-lg cursor-pointer"
				>
					Login
				</button>

				<a href="{base}/auth/register" class="mb-2 mt-2 text-sm text-blue-800">
					Don't have an account? Register
				</a>
			</form>
			<div>
				<p class="text">Or login with an alternative method below</p>

				<AuthProviders buttonPrefix="Sign-in" />
			</div>
		</div>
		<div class="flex items-center justify-center flex-auto flex-col">
			<Logo textFill="#ffffff" bgFill="#666" class="w-[320px] h-auto" />
			<p class="mt-4 pt-4 rounded-lg bg-white text-gray-500">
				Powerful Quizzes without the extra hassle
			</p>
		</div>
	</div>
</main>

{#if $loading}
	<Loader />
{/if}
