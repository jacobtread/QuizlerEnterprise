<script lang="ts">
	import { AuthProvider, openIdProviders, type OIDProvidersResponse } from "$lib/api/auth";
	import GoogleIcon from "~icons/logos/google-icon";
	import MicrosoftIcon from "~icons/logos/microsoft-icon";
	import { onMount, type ComponentType } from "svelte";
	import AuthProviderButton from "./AuthProviderButton.svelte";

	export let buttonPrefix: string;

	interface ProviderButtonData {
		icon: ComponentType;
		name: string;
	}

	const PROVIDER_BUTTON_DATA: Record<AuthProvider, ProviderButtonData> = {
		[AuthProvider.Google]: { icon: GoogleIcon, name: "Google" },
		[AuthProvider.Microsoft]: { icon: MicrosoftIcon, name: "Microsoft" }
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
					name: buttonData.name,
					url: value.auth_url
				});
			}

			providers = providers;
		} catch (e) {
			console.error("Failed to load auth providers");
		}
	}

	onMount(loadProviders);
</script>

<ul class="methods">
	{#each providers as provider}
		<li>
			<AuthProviderButton
				icon={provider.icon}
				text={`${buttonPrefix} with ${provider.name}`}
				url={provider.url}
			/>
		</li>
	{/each}
</ul>

<style lang="scss">
	.methods {
		list-style: none;
		display: flex;
		gap: 1rem;
		flex-flow: column;
		margin-top: 1rem;
	}
</style>
