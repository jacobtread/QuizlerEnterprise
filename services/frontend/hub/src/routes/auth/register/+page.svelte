<script lang="ts">
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
	import MicrosoftAuthButton from "$lib/components/auth/button/MicrosoftAuthButton.svelte";
	import GoogleAuthButton from "$lib/components/auth/button/GoogleAuthButton.svelte";

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
<main>
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

		{:else if openIDData.verified}
			<FinishAccountSetup {openIDData} {defaultUsername} />
		{/if}

		{#if loading}
			<Loader />
		{/if}

	</div>
	
</main>

<style lang="scss">
	main {
		background: url("data:image/svg+xml,%3Csvg%20id%3D%22visual%22%20viewBox%3D%220%200%20900%20600%22%20width%3D%22900%22%20height%3D%22600%22%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20xmlns%3Axlink%3D%22http%3A%2F%2Fwww.w3.org%2F1999%2Fxlink%22%20version%3D%221.1%22%3E%3Crect%20x%3D%220%22%20y%3D%220%22%20width%3D%22900%22%20height%3D%22600%22%20fill%3D%22%23ffffff%22%3E%3C%2Frect%3E%3Cpath%20d%3D%22M0%20424L21.5%20425.7C43%20427.3%2086%20430.7%20128.8%20420.5C171.7%20410.3%20214.3%20386.7%20257.2%20382.8C300%20379%20343%20395%20385.8%20396.5C428.7%20398%20471.3%20385%20514.2%20378C557%20371%20600%20370%20642.8%20373.3C685.7%20376.7%20728.3%20384.3%20771.2%20384.5C814%20384.7%20857%20377.3%20878.5%20373.7L900%20370L900%20601L878.5%20601C857%20601%20814%20601%20771.2%20601C728.3%20601%20685.7%20601%20642.8%20601C600%20601%20557%20601%20514.2%20601C471.3%20601%20428.7%20601%20385.8%20601C343%20601%20300%20601%20257.2%20601C214.3%20601%20171.7%20601%20128.8%20601C86%20601%2043%20601%2021.5%20601L0%20601Z%22%20fill%3D%22%23fa7268%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M0%20474L21.5%20463.2C43%20452.3%2086%20430.7%20128.8%20425.3C171.7%20420%20214.3%20431%20257.2%20430.3C300%20429.7%20343%20417.3%20385.8%20419.8C428.7%20422.3%20471.3%20439.7%20514.2%20439.2C557%20438.7%20600%20420.3%20642.8%20411.5C685.7%20402.7%20728.3%20403.3%20771.2%20414.7C814%20426%20857%20448%20878.5%20459L900%20470L900%20601L878.5%20601C857%20601%20814%20601%20771.2%20601C728.3%20601%20685.7%20601%20642.8%20601C600%20601%20557%20601%20514.2%20601C471.3%20601%20428.7%20601%20385.8%20601C343%20601%20300%20601%20257.2%20601C214.3%20601%20171.7%20601%20128.8%20601C86%20601%2043%20601%2021.5%20601L0%20601Z%22%20fill%3D%22%23ef5f67%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M0%20492L21.5%20488.7C43%20485.3%2086%20478.7%20128.8%20481.5C171.7%20484.3%20214.3%20496.7%20257.2%20498.7C300%20500.7%20343%20492.3%20385.8%20485C428.7%20477.7%20471.3%20471.3%20514.2%20472.7C557%20474%20600%20483%20642.8%20484.3C685.7%20485.7%20728.3%20479.3%20771.2%20480.5C814%20481.7%20857%20490.3%20878.5%20494.7L900%20499L900%20601L878.5%20601C857%20601%20814%20601%20771.2%20601C728.3%20601%20685.7%20601%20642.8%20601C600%20601%20557%20601%20514.2%20601C471.3%20601%20428.7%20601%20385.8%20601C343%20601%20300%20601%20257.2%20601C214.3%20601%20171.7%20601%20128.8%20601C86%20601%2043%20601%2021.5%20601L0%20601Z%22%20fill%3D%22%23e34c67%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M0%20486L21.5%20488.8C43%20491.7%2086%20497.3%20128.8%20505.3C171.7%20513.3%20214.3%20523.7%20257.2%20523.2C300%20522.7%20343%20511.3%20385.8%20506.2C428.7%20501%20471.3%20502%20514.2%20505.5C557%20509%20600%20515%20642.8%20521.7C685.7%20528.3%20728.3%20535.7%20771.2%20532.3C814%20529%20857%20515%20878.5%20508L900%20501L900%20601L878.5%20601C857%20601%20814%20601%20771.2%20601C728.3%20601%20685.7%20601%20642.8%20601C600%20601%20557%20601%20514.2%20601C471.3%20601%20428.7%20601%20385.8%20601C343%20601%20300%20601%20257.2%20601C214.3%20601%20171.7%20601%20128.8%20601C86%20601%2043%20601%2021.5%20601L0%20601Z%22%20fill%3D%22%23d53867%22%3E%3C%2Fpath%3E%3Cpath%20d%3D%22M0%20552L21.5%20548.3C43%20544.7%2086%20537.3%20128.8%20537.3C171.7%20537.3%20214.3%20544.7%20257.2%20548.5C300%20552.3%20343%20552.7%20385.8%20549.7C428.7%20546.7%20471.3%20540.3%20514.2%20539.7C557%20539%20600%20544%20642.8%20548.5C685.7%20553%20728.3%20557%20771.2%20555.2C814%20553.3%20857%20545.7%20878.5%20541.8L900%20538L900%20601L878.5%20601C857%20601%20814%20601%20771.2%20601C728.3%20601%20685.7%20601%20642.8%20601C600%20601%20557%20601%20514.2%20601C471.3%20601%20428.7%20601%20385.8%20601C343%20601%20300%20601%20257.2%20601C214.3%20601%20171.7%20601%20128.8%20601C86%20601%2043%20601%2021.5%20601L0%20601Z%22%20fill%3D%22%23c62368%22%3E%3C%2Fpath%3E%3C%2Fsvg%3E");
	}
</style>

