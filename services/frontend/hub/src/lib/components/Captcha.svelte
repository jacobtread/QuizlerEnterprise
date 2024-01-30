<script lang="ts">
	import { PUBLIC_RECAPTCHA_SITE_KEY } from "$env/static/public";
	import { onDestroy } from "svelte";

	// Parent container for the reCaptcha button
	let captchaContainer: HTMLDivElement;

	// Captcha token value
	export let captchaToken: string | null;

	// Field to hide completed captchas
	let hideCaptcha: boolean = false;

	let captchaId: number | null = null;

	/**
	 * Handles the google browsers scripts being done loading. Executes the
	 * actual post load logic once both scripts have been fully loaded
	 */
	function onScriptLoaded() {
		grecaptcha.ready(() => {
			// Initialize reCaptcha button
			captchaId = grecaptcha.render(captchaContainer, {
				sitekey: PUBLIC_RECAPTCHA_SITE_KEY,
				callback: onCaptchaCompleted,
				// Expired callback to clear the reCaptcha token
				"expired-callback": onCaptchaExpired,
				theme: "dark",
				size: "invisible"
			});

			grecaptcha.execute();
		});
	}

	function onCaptchaCompleted(token: string) {
		console.debug("Stored completed reCaptcha token", token);
		captchaToken = token;
		setTimeout(() => {
			hideCaptcha = true;
		}, 1000);
	}

	function onCaptchaExpired() {
		console.debug("Cleared expired reCaptcha token");
		captchaToken = null;
		hideCaptcha = false;
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

<div class="captcha" bind:this={captchaContainer} class:captcha--hidden={hideCaptcha} />
