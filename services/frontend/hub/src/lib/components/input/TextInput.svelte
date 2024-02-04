<script lang="ts">
	export let type: "text" | "email" | "password" = "text";
	export let id: string;
	export let error: string | undefined = undefined;
	export let value: string;
	export let label: string;
</script>

<div>
	<label for={id}>
		{label}

		{#if $$restProps["required"]}
			<span class="required">*</span>
		{/if}
	</label>
	<input {id} {...{ type }} bind:value {...$$restProps} />

	{#if error}
		<span class="error">{error}</span>
	{/if}
</div>

<style lang="scss">
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

	input {
		padding: 0.75rem;
		display: block;
		width: 100%;
		margin-bottom: 0.5rem;
		border: 1px solid #aaa;
		border-radius: 0.2rem;
		font-size: 1rem;
	}

	.error {
		background-color: #ff0000;
		padding: 0.5rem;
		display: block;
		color: #ffffff;
	}
</style>
