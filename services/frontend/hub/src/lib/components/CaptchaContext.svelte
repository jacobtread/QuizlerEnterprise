<script lang="ts" context="module">
	let captchaId: number | null = null;

	interface CaptchaData {
		onCompleted(token: string): void;
		onExpired(): void;
	}

	const captchaData: CaptchaData = {
		onCompleted: () => {},
		onExpired: () => {}
	};

	export function getCaptchaToken(): Promise<string> {
		return new Promise((resolve, reject) => {
			if (grecaptcha === undefined) return reject(new Error("reCaptcha is not initialized"));

			const existing = grecaptcha.getResponse();
			if (existing.length > 0) {
				return resolve(existing);
			}

			captchaData.onCompleted = resolve;
			captchaData.onExpired = () => reject(new Error("Captcha expired"));

			// Execute the captcha
			grecaptcha.execute();
		});
	}
</script>

<script lang="ts">
	import { PUBLIC_RECAPTCHA_SITE_KEY } from "$env/static/public";
	import { onDestroy } from "svelte";

	// Parent container for the reCaptcha button
	let captchaContainer: HTMLDivElement;

	/**
	 * Handles the google browsers scripts being done loading. Executes the
	 * actual post load logic once both scripts have been fully loaded
	 */
	function onScriptLoaded() {
		grecaptcha.ready(() => {
			// Initialize reCaptcha button
			captchaId = grecaptcha.render(captchaContainer, {
				sitekey: PUBLIC_RECAPTCHA_SITE_KEY,
				callback: (token: string) => captchaData.onCompleted(token),
				// Expired callback to clear the reCaptcha token
				"expired-callback": () => captchaData.onExpired(),
				size: "invisible"
			});
		});
	}

	onDestroy(() => {
		if (grecaptcha && captchaId) {
			grecaptcha.reset(captchaId);
		}
	});
</script>

<svelte:head>
	<script
		src="https://www.google.com/recaptcha/api.js"
		async
		defer
		on:load={onScriptLoaded}
	></script>
</svelte:head>

<div bind:this={captchaContainer} />

<slot />
