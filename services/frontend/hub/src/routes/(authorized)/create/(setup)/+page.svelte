<script lang="ts">
	import { createForm } from "$lib/stores/form";
	import * as z from "zod";
	import TextInput from "$lib/components/input/TextInput.svelte";
	import Loader from "$lib/components/Loader.svelte";

	const { data, errors, loading, submit } = createForm({
		// The form submission handler
		submitAction: async (data) => {
			console.log(data);
			// TODO: Create API
			// TODO: Goto editor for quiz
			// goto(`${base}/`);
		},
		// The default form data
		defaultData: { name: "" },
		// Schema for validating the form data
		schema: z.object({
			name: z.string().trim().min(4).max(100)
		})
	});
</script>

<main class="main">
	<div class="content">
		<div class="panel">
			<form on:submit|preventDefault={submit} class="form">
				<h1 class="title">Create Quiz</h1>
				<p class="text">Give your quiz a name...</p>

				<TextInput
					label="Quiz Name"
					type="text"
					id="name"
					required
					placeholder="Something spectacular..."
					error={$errors["name"]}
					bind:value={$data.name}
				/>

				<button class="button">Create</button>
			</form>
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

	.form {
		display: flex;
		flex-flow: column;
		gap: 1rem;
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
		font-size: 2rem;
		font-weight: 600;
		color: #444;
	}

	.text {
		color: #555;
	}
</style>
